use url::Url;

use okx_rs::api::v5::InstrumentType::{Futures, Spot, Swap};
use okx_rs::api::v5::Instruments;
use okx_rs::websocket::async_client::OKXWebsocketClient;
use okx_rs::websocket::AsyncWebsocketClient;
use okx_rs::websocket::{AsyncWebsocketSession, Subscriptions};

#[tokio::main]
async fn main() {
    let mut client =
        OKXWebsocketClient::new(Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap())
            .await
            .unwrap();
    let session = Subscriptions::default();
    session
        .subscribe_channel(&mut client, Instruments(Futures))
        .await
        .unwrap();
    session
        .subscribe_channel(&mut client, Instruments(Swap))
        .await
        .unwrap();
    session
        .subscribe_channel(&mut client, Instruments(Spot))
        .await
        .unwrap();
    let mut ping = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        let data = tokio::select! {
            res = client.next() => res,
            _ = ping.tick() => {
                client.send("ping").await.unwrap();
                continue
            }
            else => continue
        };
        println!("{:?}", data);
    }
}
