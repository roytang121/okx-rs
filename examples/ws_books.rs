use okx_rs::websocket::conn::Channel;
use okx_rs::websocket::{OKXWebsocketClient, Subscriptions, WebsocketSession};
use url::Url;

fn main() {
    let client = OKXWebsocketClient::new_with_sync(
        Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap(),
    )
    .unwrap();
    let mut session = Subscriptions {
        channels: vec![
            Box::new(Channel::OrderBook {
                channel: "books5".to_string(),
                inst_id: "BTC-USDT".to_string(),
            }),
            Box::new(Channel::OrderBook {
                channel: "books5".to_string(),
                inst_id: "ETH-USDT".to_string(),
            }),
        ],
    };
    session.start(Box::new(client), |message| {
        println!("message: {:?}", message);
    });
}
