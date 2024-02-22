use okx_rs::api::v5::WsResponse;
use okx_rs::websocket::async_client::OKXWebsocketClient;
use okx_rs::websocket::conn::{BboTbt, Books};
use okx_rs::websocket::{AsyncWebsocketClient, AsyncWebsocketSession, Message, Subscriptions};
use url::Url;

#[tokio::main]
async fn main() {
    let mut client =
        OKXWebsocketClient::new(Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap())
            .await
            .unwrap();
    let session = Subscriptions::default();
    session
        .subscribe_channel(
            &mut client,
            BboTbt {
                inst_id: "BTC-USDT-SWAP".into(),
            },
        )
        .await
        .unwrap();
    session
        .subscribe_channel(
            &mut client,
            Books {
                inst_id: "BTC-USDT-SWAP".into(),
            },
        )
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
        // println!("{:?}", data);
        let mut msg = match data {
            Ok(Some(Message::Data(msg))) => msg,
            Err(err) => {
                // abnormal connection handling
                eprintln!("{:?}", err);
                continue;
            }
            _ => continue,
        };

        match Books::try_parse_books(&mut msg) {
            None => {}
            Some(WsResponse {
                arg,
                action,
                event,
                data,
                ..
            }) => {
                println!("{:?} {:?} {:?} {:?}", arg, action, event, data);
            }
        }
    }
}
