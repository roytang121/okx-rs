use log::{error, info};
use okx_rs::api::options::Options;
use okx_rs::api::v5::{AccountChannel, BalanceAndPositionChannel, BalanceAndPositionDetail, InstrumentType, PositionsChannel};
use okx_rs::websocket::async_client::OKXWebsocketClient;
use okx_rs::websocket::{AsyncWebsocketClient, AsyncWebsocketSession, Message, PublicChannel, Subscriptions};
use url::Url;
use okx_rs::api::v5::ws_convert::TryParseEvent;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let mut client =
        OKXWebsocketClient::new(Url::parse("wss://ws.okx.com:8443/ws/v5/private").unwrap())
            .await
            .unwrap();

    let key = std::env::var("OKX_API_KEY").unwrap();
    let secret = std::env::var("OKX_API_SECRET").unwrap();
    let passphrase = std::env::var("OKX_API_PASSPHRASE").unwrap();
    let options = Options {
        key: Some(key),
        secret: Some(secret),
        passphrase: Some(passphrase),
    };

    let session = Subscriptions::default();
    session.auth(&mut client, options).await.unwrap();
    session
        .subscribe_channel(&mut client, AccountChannel)
        .await
        .unwrap();
    session
        .subscribe_channel(&mut client, PositionsChannel {
            inst_type: InstrumentType::Any,
            inst_family: None,
            inst_id: None,
        })
        .await
        .unwrap();
    session.subscribe_channel(&mut client, BalanceAndPositionChannel).await.unwrap();

    let mut ping = tokio::time::interval(tokio::time::Duration::from_secs(10));

    loop {
        let res = tokio::select! {
            res = client.next() => {
                log::debug!("{:?}", res);
                res
            }
            _ = ping.tick() => {
                client.send("ping".into()).await.unwrap();
                continue
            }
            else => continue
        };

        let msg = match res {
            Ok(Some(Message::Data(msg))) => msg,
            Err(err) => {
                log::error!("{:?}", err);
                continue;
            },
            _ => continue,
        };

        if let Ok(Some(bal_and_pos)) = BalanceAndPositionChannel::try_parse(&msg) {
            info!("{:?}", bal_and_pos);
        } else if let Ok(Some(account)) = AccountChannel::try_parse(&msg) {
            info!("{:?}", account);
        } else if let Ok(Some(pos)) = PositionsChannel::try_parse(&msg) {
            info!("{:?}", pos);
        } else {
            continue
        }
    }
}
