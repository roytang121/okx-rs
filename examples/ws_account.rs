use log::info;
use okx_rs::api::options::Options;
use okx_rs::api::v5::AccountChannel;
use okx_rs::websocket::async_client::OKXWebsocketClient;
use okx_rs::websocket::{AsyncWebsocketClient, AsyncWebsocketSession, Message, PublicChannel, Subscriptions};
use url::Url;

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

    let mut ping = tokio::time::interval(tokio::time::Duration::from_secs(30));

    loop {
        tokio::select! {
            _ = ping.tick() => {
                client.send("ping".into()).await.unwrap();
            }
            res = client.next() => {
                info!("{:?}", res);
            }
            else => continue
        }
    }
}
