use okx_rs::api::v5::ws_convert::{OKXWsConvert, TryParseEvent};
use okx_rs::api::v5::BookUpdate;
use okx_rs::websocket::async_client::OKXWebsocketClient;
use okx_rs::websocket::conn::{BboTbt, Books, Books5};
use okx_rs::websocket::{
    AsyncWebsocketSession, Message, PublicChannel, Subscriptions, WebsocketSession,
};
use std::future::Future;
use url::Url;

#[tokio::main]
async fn main() {
    let client = OKXWebsocketClient::new(Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap())
        .await
        .unwrap();
    let mut session = Subscriptions {
        channels: vec![
            PublicChannel::new(Books5 {
                inst_id: "BTC-USDT-SWAP".to_string(),
            }),
            PublicChannel::new(Books {
                inst_id: "BTC-USDT-SWAP".to_string(),
            }),
            PublicChannel::new(BboTbt {
                inst_id: "BTC-USDT-SWAP".to_string(),
            }),
        ],
    };
    let (inbound, mut inbound_recv) = tokio::sync::mpsc::channel::<Message>(1024);
    tokio::spawn(async move {
        loop {
            match inbound_recv.recv().await {
                None => {}
                Some(Message::Data(msg)) => {
                    let update = match (
                        BboTbt::try_parse(&msg),
                        Books5::try_parse(&msg),
                        Books::try_parse(&msg),
                    ) {
                        (Some(update), None, None) => update,
                        (None, Some(update), None) => update,
                        (None, None, Some(update)) => update,
                        _ => continue,
                    };
                    println!("{:?}", update);
                }
                _ => {}
            }
        }
    });
    session.spawn(Box::new(client), inbound).await
}
