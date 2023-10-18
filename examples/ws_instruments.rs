use log::logger;
use okx_rs::api::v5::websocket::{IndexTickers, Instruments, MarkPrices};
use okx_rs::api::v5::InstrumentType;
use okx_rs::websocket::conn::Books5;
use okx_rs::websocket::{AsyncWebsocketSession, Message, PublicChannel, Subscriptions, WebsocketSession};
use url::Url;
use okx_rs::api::v5::ws_convert::OKXWsConvert;
use okx_rs::websocket::async_client::OKXWebsocketClient;

#[tokio::main]
async fn main() {
    let client = OKXWebsocketClient::new(
        Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap(),
    ).await
    .unwrap();
    let mut session = Subscriptions {
        channels: vec![
            PublicChannel::new(Instruments(InstrumentType::Futures)),
            PublicChannel::new(IndexTickers("BTC-USDT".to_string())),
            PublicChannel::new(MarkPrices("BTC-USDT-SWAP".to_string())),
        ],
    };
    // session.start(Box::new(client), |message| {
    //     println!("message: {:?}", message);
    // });
    let (inbound, mut inbound_recv) = tokio::sync::mpsc::channel(1024);
    let handle = tokio::spawn(async move {
        session.spawn(Box::new(client), inbound).await
    });
    loop {
        match inbound_recv.recv().await{
            None => {}
            Some(Message::Data(msg)) => {
                if let Some(mark_prices) = OKXWsConvert::try_parse_mark_prices(&msg) {
                    println!("{:?}", mark_prices)
                }
            }
            _ => {}
        }
    }
}
