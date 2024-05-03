use log::info;
use okx_rs::api::{DemoTrading, OKXEnv};
use tungstenite::Message;

use okx_rs::api::v5::ws_convert::TryParseEvent;
use okx_rs::api::v5::{
    AccountChannel, BalanceAndPositionChannel, InstrumentType, PositionsChannel,
};
use okx_rs::api::Options;
use okx_rs::websocket::OKXAuth;
use okx_rs::websocket::WebsocketChannel;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let key = std::env::var("OKX_API_KEY").unwrap();
    let secret = std::env::var("OKX_API_SECRET").unwrap();
    let passphrase = std::env::var("OKX_API_PASSPHRASE").unwrap();
    let options = Options::new_with(DemoTrading, key, secret, passphrase);

    let (mut client, response) = tungstenite::connect(DemoTrading.private_websocket()).unwrap();

    let auth_msg = OKXAuth::ws_auth(options).unwrap();
    client.send(auth_msg.into()).unwrap();

    let auth_resp = client.read().unwrap();
    info!("auth_resp: {:?}", auth_resp);

    client
        .send(AccountChannel.subscribe_message().into())
        .unwrap();
    client
        .send(
            PositionsChannel {
                inst_type: InstrumentType::Any,
                inst_family: None,
                inst_id: None,
            }
            .subscribe_message()
            .into(),
        )
        .unwrap();
    client
        .send(BalanceAndPositionChannel.subscribe_message().into())
        .unwrap();

    loop {
        let msg = match client.read() {
            Ok(Message::Text(msg)) => msg,
            Err(err) => {
                panic!("{:?}", err);
            }
            _ => continue,
        };

        if let Ok(Some(bal_and_pos)) = BalanceAndPositionChannel::try_parse(&msg) {
            info!("{:?}", bal_and_pos);
        } else if let Ok(Some(account)) = AccountChannel::try_parse(&msg) {
            info!("{:?}", account);
        } else if let Ok(Some(pos)) = PositionsChannel::try_parse(&msg) {
            info!("{:?}", pos);
        } else {
            continue;
        }
    }
}
