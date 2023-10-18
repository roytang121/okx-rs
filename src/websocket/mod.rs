use crossbeam_utils::Backoff;
use std::time::Instant;
use std::{fmt::Debug, time::Duration};
use std::future::Future;
use std::net::ToSocketAddrs;
use serde::{Deserialize, Serialize};
use serde_json::de::Read;
use tungstenite::stream::NoDelay;
use crate::api::v5::ChannelArg;

pub mod conn;

#[derive(Debug)]
pub enum Message {
    Data(String),
    Ping,
    Pong,
    Error,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("tungstenite websocket error: {0}")]
    Tungstenite(#[from] tungstenite::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("websocket error: {0}")]
    Other(String),
}

pub trait WebsocketClient {
    fn send(&mut self, msg: &str) -> Result<()>;
    fn next(&mut self) -> Result<Message>;
}

#[async_trait::async_trait]
pub trait AsyncWebsocketClient {
    async fn next(&mut self) -> Result<Message>;
    async fn send(&mut self, msg: &str) -> Result<()>;
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
    pub fn new(channel: T) -> Box<Self> {
        Box::new(Self { channel })
    }
}

#[async_trait::async_trait]
impl<T: WebsocketChannel + Send + Sync> WebsocketConn for PublicChannel<T> {
    fn subscribe(&self, client: &mut dyn WebsocketClient) -> Result<()> {
        client.send(&self.channel.subscribe_message())
    }

    fn unsubscribe(&self, client: &mut dyn WebsocketClient) -> Result<()> {
        client.send(&self.channel.unsubscribe_message())
    }

    async fn subscribe_async(&self, sender: &tokio::sync::mpsc::Sender<String>) -> Result<()> {
        sender.send(self.channel.subscribe_message()).await.unwrap();
        Ok(())
    }

    async fn unsubscribe_async(&self, sender: &tokio::sync::mpsc::Sender<String>) -> Result<()> {
        todo!()
    }
}

#[async_trait::async_trait]
pub trait WebsocketConn: Send + Sync {
    fn subscribe(&self, client: &mut dyn WebsocketClient) -> Result<()>;
    fn unsubscribe(&self, client: &mut dyn WebsocketClient) -> Result<()>;

    async fn subscribe_async(&self, sender: &tokio::sync::mpsc::Sender<String>) -> Result<()>;
    async fn unsubscribe_async(&self, sender: &tokio::sync::mpsc::Sender<String>) -> Result<()>;
}

pub trait WebsocketChannel: Send + Sync {
    const CHANNEL: &'static str;
    type Response<'de>: Deserialize<'de>;
    type ArgType<'de>: Deserialize<'de>;

    fn subscribe_message(&self) -> String;
    fn unsubscribe_message(&self) -> String;
}

pub trait WebsocketSession {
    type Client: WebsocketClient;

    fn start<F>(&mut self, mut client: Box<dyn WebsocketClient>, mut handler: F)
    where
        F: FnMut(Result<Message>),
    {
        // subscribe all channels
        for conn in self.conns() {
            if let Err(err) = conn.subscribe(&mut *client) {
                handler(Err(err));
            }
        }

        let backoff = Backoff::new();
        let mut last_ping = Instant::now();
        loop {
            if last_ping.elapsed() > Duration::from_millis(5000) {
                client
                    .send("ping")
                    .expect("unhandled ping error. probably connection closed");
                last_ping = Instant::now();
            }
            match client.next() {
                Ok(msg) => handler(Ok(msg)),
                Err(Error::Tungstenite(tungstenite::Error::Io(err)))
                    if matches!(err.kind(), std::io::ErrorKind::WouldBlock) =>
                {
                    // backoff
                    // TODO: opt spinning for minimal latency
                    backoff.snooze();
                    continue;
                }
                Err(err) => handler(Err(err)),
            }
        }
    }

    fn conns(&self) -> &[Box<dyn WebsocketConn>];
    fn conns_mut(&mut self) -> &mut Vec<Box<dyn WebsocketConn>>;
}

#[async_trait::async_trait]
pub trait AsyncWebsocketSession where Self: Send + Sync + WebsocketSession {
    type Client: AsyncWebsocketClient + Send;

    async fn spawn(&mut self, mut client: Box<<Self as AsyncWebsocketSession>::Client>, inbound: tokio::sync::mpsc::Sender<Message>) {
        let (outbound, mut outbound_recv) = tokio::sync::mpsc::channel(1024);
        for conn in self.conns() {
            if let Err(err) = conn.subscribe_async(&outbound).await {
                panic!("{}", err)
            }
        }

        loop {
            tokio::select! {
                Some(msg) = outbound_recv.recv() => {
                    client.send(&msg).await.unwrap();
                },
                result = client.next() => {
                    match result {
                        Ok(msg) => inbound.send(msg).await.unwrap(),
                        Err(err) => todo!()
                    }
                }
                else => continue,
            }
        }
    }
}


pub struct Subscriptions {
    pub channels: Vec<Box<dyn WebsocketConn>>,
}

impl WebsocketSession for Subscriptions {
    type Client = self::sync::OKXWebsocketClient;

    fn conns(&self) -> &[Box<dyn WebsocketConn>] {
        &self.channels
    }

    fn conns_mut(&mut self) -> &mut Vec<Box<dyn WebsocketConn>> {
        &mut self.channels
    }
}

impl AsyncWebsocketSession for Subscriptions {
    type Client = async_client::OKXWebsocketClient;
}

pub mod sync {
    use super::Result;
    use super::Error;
    use tungstenite::{client, client::connect_with_config, connect, stream::MaybeTlsStream, WebSocket};
    use std::net::TcpStream;
    use log::info;
    use tungstenite::stream::NoDelay;
    use url::Url;
    use crate::websocket::{Message, WebsocketClient};

    pub struct OKXWebsocketClient {
        socket: WebSocket<MaybeTlsStream<TcpStream>>,
    }

    impl OKXWebsocketClient {
        pub fn new(url: Url) -> Result<Self> {
            let socket = match connect_with_config(url, None, 3) {
                Ok((mut socket, response)) => {
                    info!("Connected to the server");
                    info!("Response HTTP code: {}", response.status());
                    info!("Response contains the following headers:");

                    // make nodelay
                    socket.get_mut().set_nodelay(true)?;
                    // make nonblocking
                    match socket.get_mut() {
                        MaybeTlsStream::Plain(s) => {
                            s.set_nonblocking(true)?;
                        }
                        MaybeTlsStream::NativeTls(s) => {
                            s.get_mut().set_nonblocking(true)?;
                        }
                        _ => unimplemented!("tls stream type not supported yet"),
                    }
                    socket
                }
                Err(err) => {
                    return Err(Error::Tungstenite(err));
                }
            };

            Ok(Self { socket })
        }
    }

    impl WebsocketClient for OKXWebsocketClient {
        fn send(&mut self, msg: &str) -> Result<()> {
            info!("send: {}", msg);
            self.socket
                .send(tungstenite::Message::Text(msg.to_string()))
                .map_err(|err| Error::Tungstenite(err))
        }

        fn next(&mut self) -> Result<Message> {
            match self.socket.read() {
                Ok(tungstenite::Message::Text(msg)) => Ok(Message::Data(msg)),
                Ok(msg) => Err(Error::Other(format!("unknown message: {}", msg))),
                Err(err) => Err(Error::Tungstenite(err)),
            }
        }
    }
}

pub mod async_client {
    use async_trait::async_trait;
    use super::{Result, Error};
    use log::info;
    use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tungstenite::stream::NoDelay;
    use url::Url;
    use futures_util::stream::{SplitSink, SplitStream};
    use futures_util::{SinkExt, StreamExt};
    use crate::websocket::{AsyncWebsocketClient, Message};

    pub struct OKXWebsocketClient {
        pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>,
        pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
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
                            stream.get_mut().get_mut().get_mut().set_nodelay(true).unwrap();
                        }
                        _ => unimplemented!()
                    }
                    socket
                },
                Err(err) => panic!("unhandled err: {}", err)
            };
            let (write, read) = socket.split();
            Ok(Self { write, read })
        }
    }

    #[async_trait::async_trait]
    impl AsyncWebsocketClient for OKXWebsocketClient {
        async fn next(&mut self) -> Result<Message> {
            match self.read.next().await {
                None => Err(Error::Other("NoData".to_string())),
                Some(Ok(tungstenite::Message::Text(msg))) => Ok(Message::Data(msg)),
                Some(Ok(_)) => Err(Error::Other("unhandled message type".to_string())),
                Some(Err(err)) => Err(Error::Tungstenite(err)),
            }
        }

        async fn send(&mut self, msg: &str) -> Result<()> {
            info!("send: {}", msg);
            self.write
                .send(tungstenite::Message::Text(msg.to_string()))
                .await
                .map_err(|err| Error::Tungstenite(err))
        }
    }
}
