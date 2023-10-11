use crossbeam_utils::Backoff;
use log::info;
use std::time::Instant;
use std::{collections::HashMap, fmt::Debug, net::TcpStream, thread::sleep, time::Duration};
use url::Url;

use crate::websocket::Message::Ping;
use serde::{Deserialize, Serialize};
use tungstenite::stream::NoDelay;
use tungstenite::{client::connect_with_config, stream::MaybeTlsStream, WebSocket};

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

impl<T: WebsocketChannel> WebsocketConn for PublicChannel<T> {
    fn subscribe(&self, client: &mut dyn WebsocketClient) -> Result<()> {
        client.send(&self.channel.subscribe_message())
    }

    fn unsubscribe(&self, client: &mut dyn WebsocketClient) -> Result<()> {
        client.send(&self.channel.unsubscribe_message())
    }
}

pub trait WebsocketConn {
    fn subscribe(&self, client: &mut dyn WebsocketClient) -> Result<()>;
    fn unsubscribe(&self, client: &mut dyn WebsocketClient) -> Result<()>;
}

pub trait WebsocketChannel {
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

pub struct Subscriptions {
    pub channels: Vec<Box<dyn WebsocketConn>>,
}

impl WebsocketSession for Subscriptions {
    type Client = OKXWebsocketClient;

    fn conns(&self) -> &[Box<dyn WebsocketConn>] {
        &self.channels
    }

    fn conns_mut(&mut self) -> &mut Vec<Box<dyn WebsocketConn>> {
        &mut self.channels
    }
}

pub struct OKXWebsocketClient {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl OKXWebsocketClient {
    pub fn new_with_sync(url: Url) -> Result<Self> {
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
        loop {
            return match self.socket.read() {
                Ok(tungstenite::Message::Text(msg)) => Ok(Message::Data(msg)),
                Ok(msg) => Err(Error::Other(format!("unknown message: {}", msg))),
                Err(err) => Err(Error::Tungstenite(err)),
            };
        }
    }
}

pub trait OKX {
    fn new_with_config() -> Self;
}
