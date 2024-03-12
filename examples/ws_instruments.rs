use tungstenite::Message;

use okx_rs::api::v5::ws_convert::TryParseEvent;
use okx_rs::api::v5::InstrumentType::Futures;
use okx_rs::api::v5::Instruments;
use okx_rs::websocket::WebsocketChannel;

fn main() {
    let (mut client, response) =
        tungstenite::connect("wss://ws.okx.com:8443/ws/v5/public").unwrap();
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    println!("{:?}", response.headers());

    client
        .send(Instruments(Futures).subscribe_message().into())
        .unwrap();

    loop {
        let msg = match client.read() {
            Ok(Message::Text(msg)) => msg,
            Err(err) => {
                panic!("{:?}", err);
            }
            _ => continue,
        };
        
        match Instruments::try_parse(&msg) {
            Ok(Some(resp)) => match resp.data {
                Some(instruments) => {
                    println!("instruments: {:?}", instruments);
                }
                None => println!("other response: {:?}", resp),
            },
            Err(err) => panic!("Error parsing response: {:?}", err),
            _ => {}
        }
    }
}
