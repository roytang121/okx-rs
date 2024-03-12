use okx_rs::api::v5::ws_convert::TryParseEvent;
use okx_rs::websocket::{WebsocketChannel};
use okx_rs::websocket::conn::Books5;

fn main() {
    let (mut client, response) = tungstenite::connect("wss://ws.okx.com:8443/ws/v5/public").unwrap();
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    println!("{:?}", response.headers());

    client.send(Books5 { inst_id: "BTC-USDT-SWAP".into() }.subscribe_message().into()).unwrap();
    client.send(Books5 { inst_id: "BTC-USDT".into() }.subscribe_message().into()).unwrap();
    client.send(Books5 { inst_id: "ETH-USDT-SWAP".into() }.subscribe_message().into()).unwrap();
    client.send(Books5 { inst_id: "ETH-USDT".into() }.subscribe_message().into()).unwrap();
    
    loop {
        let msg = client.read().unwrap();
        let data = msg.into_text().unwrap();
        
        match Books5::try_parse(&data) {
            Ok(Some(resp)) => {
                match resp.data {
                    Some([book_update, ..]) => {
                        println!("book_update: {:?}", book_update);
                    }
                    None => println!("other response: {:?}", resp),
                }
            }
            Ok(None) => continue,
            Err(err) => {
                println!("Error parsing response: {:?}", err);
            }
        }
    }
}
