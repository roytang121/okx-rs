use log::logger;
use okx_rs::api::v5::websocket::{IndexTickers, Instruments, MarkPrices};
use okx_rs::api::v5::InstrumentType;
use okx_rs::websocket::conn::Channel;
use okx_rs::websocket::{OKXWebsocketClient, PublicChannel, Subscriptions, WebsocketSession};
use url::Url;

fn main() {
    let client = OKXWebsocketClient::new_with_sync(
        Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap(),
    )
    .unwrap();
    let mut session = Subscriptions {
        channels: vec![
            PublicChannel::new(Instruments(InstrumentType::Futures)),
            PublicChannel::new(IndexTickers("BTC-USDT".to_string())),
            PublicChannel::new(MarkPrices("BTC-USDT-SWAP".to_string())),
        ],
    };
    session.start(Box::new(client), |message| {
        println!("message: {:?}", message);
    });
}
