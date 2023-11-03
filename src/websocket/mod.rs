use std::fmt::Debug;
use std::os::fd::RawFd;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::api::options::Options;

pub mod conn;

#[derive(Debug)]
pub enum Message {
    Data(String),
    Ping,
    Pong,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type TokioWsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type TokioSink = SplitSink<TokioWsStream, tungstenite::Message>;
pub type TokioStream = SplitStream<TokioWsStream>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("tungstenite websocket error: {0}")]
    Tungstenite(#[from] tungstenite::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    // #[error("tokio channel error: {0}")]
    // TokioChannelSend(#[from] tokio::sync::mpsc::error::SendError<_>),
    #[error("websocket error: {0}")]
    Other(String),
}

#[async_trait::async_trait]
pub trait AsyncWebsocketClient {
    type Sink;
    type Stream;

    async fn try_auth(&mut self, options: Options) -> Result<()>;
    async fn send(&mut self, msg: &str) -> Result<()>;
    async fn next(&mut self) -> Result<Option<Message>>;
    async fn quick_ack(&mut self);
    fn split(self) -> (Self::Sink, Self::Stream);
    fn fd(&self) -> RawFd;
}

pub struct PublicChannel<T>
where
    T: WebsocketChannel,
{
    pub channel: T,
}

impl<T> PublicChannel<T>
where
    T: WebsocketChannel,
{
    pub fn new(channel: T) -> Self {
        Self { channel }
    }
}

pub trait WebsocketChannel: Send + Sync {
    const CHANNEL: &'static str;
    const AUTH: bool = false;
    type Response<'de>: Deserialize<'de> + Debug;
    type ArgType<'de>: Deserialize<'de> + Debug;

    fn subscribe_message(&self) -> String {
        serde_json::json!({
            "op": "subscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                }
            ]
        })
        .to_string()
    }

    fn unsubscribe_message(&self) -> String {
        serde_json::json!({
            "op": "unsubscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                }
            ]
        })
        .to_string()
    }
    fn is_private(&self) -> bool {
        Self::AUTH
    }
}

#[async_trait::async_trait]
pub trait AsyncWebsocketSession
where
    Self: Send + Sync,
{
    async fn auth(
        &self,
        client: &mut (impl AsyncWebsocketClient + Send),
        options: Options,
    ) -> Result<()> {
        client.try_auth(options).await
    }
    async fn subscribe_channel(
        &self,
        client: &mut (impl AsyncWebsocketClient + Send),
        channel: impl WebsocketChannel + Send,
    ) -> Result<()> {
        client.send(&channel.subscribe_message()).await
    }

    fn multiplex<I: Send, O: Send + From<I>>(
        &mut self,
        mut r#in: impl Inbound<Value = I> + Send + 'static,
        mut out: impl Outbound<Value = O> + Send + 'static,
    ) {
        tokio::task::spawn(async move {
            loop {
                match r#in.inbound().await {
                    Ok(Some(msg)) => out.outbound(msg.into()).await.unwrap(),
                    Ok(None) => {}
                    Err(err) => log::error!("{}", err),
                }
            }
        });
    }
}

#[async_trait::async_trait]
pub trait Outbound {
    type Value;
    async fn outbound(&mut self, msg: Self::Value) -> Result<()>;
}

pub trait OutboundSync {
    type Value;
    fn outbound(&mut self, msg: Self::Value) -> Result<()>;
}

impl<T> OutboundSync for std::sync::mpsc::Sender<T> {
    type Value = T;

    fn outbound(&mut self, msg: Self::Value) -> Result<()> {
        self.send(msg)
            .map_err(|err| Error::Other("sync channel dropped".into()))
    }
}

impl<T> OutboundSync for crossbeam_channel::Sender<T> {
    type Value = T;

    fn outbound(&mut self, msg: Self::Value) -> Result<()> {
        self.try_send(msg)
            .map_err(|err| Error::Other("sync channel dropped".into()))
    }
}

#[async_trait::async_trait]
pub trait Inbound {
    type Value;
    async fn inbound(&mut self) -> Result<Option<Self::Value>>;
}
#[async_trait::async_trait]
impl Outbound for TokioSink {
    type Value = tungstenite::Message;

    async fn outbound(&mut self, msg: Self::Value) -> Result<()> {
        self.send(msg).await.map_err(Error::Tungstenite)
    }
}

#[async_trait::async_trait]
impl<T> Outbound for tokio::sync::mpsc::Sender<T>
where
    T: Send,
{
    type Value = T;

    async fn outbound(&mut self, msg: T) -> Result<()> {
        self.send(msg)
            .await
            .map_err(|err| Error::Other(err.to_string()))
    }
}

#[async_trait::async_trait]
impl Inbound for TokioStream {
    type Value = tungstenite::Message;

    async fn inbound(&mut self) -> Result<Option<Self::Value>> {
        match self.try_next().await {
            Ok(msg) => Ok(msg),
            Err(err) => Err(Error::Tungstenite(err)),
        }
    }
}

#[async_trait::async_trait]
impl Inbound for tokio::sync::mpsc::Receiver<tungstenite::Message> {
    type Value = tungstenite::Message;

    async fn inbound(&mut self) -> Result<Option<Self::Value>> {
        match self.recv().await {
            None => Ok(None),
            Some(msg) => Ok(Some(msg)),
        }
    }
}

#[derive(Debug, Default)]
pub struct Subscriptions {}
impl AsyncWebsocketSession for Subscriptions {}

pub mod async_client {
    use std::os::fd::{AsRawFd, RawFd};

    use futures_util::{SinkExt, StreamExt};
    use log::info;
    use serde_json::json;
    use tokio::net::TcpStream;
    use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
    use url::Url;

    use crate::api::credential::Credential;
    use crate::api::options::Options;
    use crate::websocket::{AsyncWebsocketClient, Message};

    use super::{Error, Result, TokioSink, TokioStream};

    pub struct OKXWebsocketClient {
        // pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>,
        // pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    }

    impl OKXWebsocketClient {
        pub async fn new(url: Url) -> Result<Self> {
            let socket = match connect_async(url).await {
                Ok((mut socket, response)) => {
                    info!("Connected to the server");
                    info!("Response HTTP code: {}", response.status());
                    info!("Response contains the following headers:");
                    match socket.get_mut() {
                        MaybeTlsStream::Plain(stream) => {
                            stream.set_nodelay(true).unwrap();
                        }
                        MaybeTlsStream::NativeTls(stream) => {
                            stream
                                .get_mut()
                                .get_mut()
                                .get_mut()
                                .set_nodelay(true)
                                .unwrap();
                        }
                        _ => unimplemented!(),
                    }
                    socket
                }
                Err(err) => panic!("unhandled err: {}", err),
            };
            Ok(Self { socket })
        }
    }

    #[async_trait::async_trait]
    impl AsyncWebsocketClient for OKXWebsocketClient {
        type Sink = TokioSink;
        type Stream = TokioStream;

        async fn try_auth(&mut self, options: Options) -> Result<()> {
            let credential: Credential = match (&options).try_into() {
                Ok(credential) => credential,
                Err(_) => todo!(),
            };
            let timestamp = format!("{}", chrono::Utc::now().timestamp_millis() / 1000);
            let (key, signature) =
                credential.signature_ws(reqwest::Method::GET, &timestamp, "/users/self/verify");
            let auth_msg = json!({
                "op": "login",
                "args": [
                    {
                      "apiKey": key,
                      "passphrase": options.passphrase.unwrap(),
                      "timestamp": timestamp,
                      "sign": signature,
                    }
                ]
            })
            .to_string();
            // send auth message
            self.send(&auth_msg).await?;
            self.next().await?;
            Ok(())
        }

        async fn send(&mut self, msg: &str) -> Result<()> {
            log::info!("send: {}", msg);
            self.socket
                .send(tungstenite::Message::Text(msg.to_string()))
                .await
                .map_err(Error::Tungstenite)
        }

        async fn next(&mut self) -> Result<Option<Message>> {
            match self.socket.next().await {
                Some(Ok(tungstenite::Message::Text(msg))) => Ok(Some(Message::Data(msg))),
                Some(Ok(_)) => Err(Error::Other("unhandled message type".to_string())),
                Some(Err(err)) => Err(Error::Tungstenite(err)),
                None => Ok(None),
            }
        }

        async fn quick_ack(&mut self) {
            todo!()
        }

        fn split(self) -> (Self::Sink, Self::Stream) {
            self.socket.split()
        }

        fn fd(&self) -> RawFd {
            match self.socket.get_ref() {
                MaybeTlsStream::Plain(raw) => raw.as_raw_fd(),
                MaybeTlsStream::NativeTls(stream) => {
                    stream.get_ref().get_ref().get_ref().as_raw_fd()
                }
                _ => unimplemented!(),
            }
        }
    }
}
