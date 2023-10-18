use crate::api::v5::WsResponse;
use crate::api::v5::websocket::{Instruments, MarkPrices};
use crate::websocket::conn::{BboTbt, Books, Books5};
use crate::websocket::WebsocketChannel;

pub struct OKXWsConvert;

pub trait TryParseEvent {
    type Value<'a>;
    fn try_parse(msg: &str) -> Option<Self::Value<'_>>;
}
impl<T> TryParseEvent for T where T: WebsocketChannel {
    type Value<'a> = <T as WebsocketChannel>::Response<'a>;

    fn try_parse(msg: &str) -> Option<Self::Value<'_>> {
        if msg.contains(&format!(r#""channel":"{}""#, T::CHANNEL)) {
            if msg.contains(r#""event":"error""#) {
                return None;
            }
            if msg.contains(r#""event":"subscribe""#) {
                return None;
            }
            if msg.contains(r#""event":"unsubscribe""#) {
                return None;
            }
            let response: WsResponse<<T as WebsocketChannel>::ArgType<'_>, <T as WebsocketChannel>::Response<'_>> = serde_json::from_str(msg).unwrap();
            response.data
        } else {
            None
        }
    }
}

impl OKXWsConvert {
    pub fn try_parse_instruments(msg: &str) -> Option<<Instruments as WebsocketChannel>::Response<'_>> {
        if msg.contains(Instruments::CHANNEL) {
            if msg.contains(r#"event": "error"#) {
                return None;
            }
            if msg.contains(r#"event": "subscribe"#) {
                return None;
            }
            if msg.contains(r#"event": "unsubscribe"#) {
                return None;
            }
            let response: WsResponse<<Instruments as WebsocketChannel>::ArgType<'_>, <Instruments as WebsocketChannel>::Response<'_>> = serde_json::from_str(msg).ok()?;
            response.data
        } else {
            None
        }
    }

    pub fn try_parse_mark_prices(msg: &str) -> Option<<MarkPrices as WebsocketChannel>::Response<'_>> {
        if msg.contains(MarkPrices::CHANNEL) {
            if msg.contains(r#"event": "error"#) {
                return None;
            }
            if msg.contains(r#"event": "subscribe"#) {
                return None;
            }
            if msg.contains(r#"event": "unsubscribe"#) {
                return None;
            }
            let response: WsResponse<<MarkPrices as WebsocketChannel>::ArgType<'_>, <MarkPrices as WebsocketChannel>::Response<'_>> = serde_json::from_str(msg).unwrap();
            response.data
        } else {
            None
        }
    }

    pub fn try_parse_books5(msg: &str) -> Option<<Books5 as WebsocketChannel>::Response<'_>> {
        println!("{}", msg);
        if msg.contains(Books5::CHANNEL) {
            if msg.contains(r#"event": "error"#) {
                return None;
            }
            if msg.contains(r#"event": "subscribe"#) {
                return None;
            }
            if msg.contains(r#"event": "unsubscribe"#) {
                return None;
            }
            let response: WsResponse<<Books5 as WebsocketChannel>::ArgType<'_>, <Books5 as WebsocketChannel>::Response<'_>> = serde_json::from_str(msg).unwrap();
            response.data
        } else {
            None
        }
    }

    pub fn try_parse_book_update(msg: &str) -> Option<<Books as WebsocketChannel>::Response<'_>> {
        if msg.contains(BboTbt::CHANNEL) || msg.contains(Books5::CHANNEL) || msg.contains(Books::CHANNEL) {
            if msg.contains(r#"event": "error"#) {
                return None;
            }
            if msg.contains(r#"event": "subscribe"#) {
                return None;
            }
            if msg.contains(r#"event": "unsubscribe"#) {
                return None;
            }
            // println!("{}", msg);
            let response: WsResponse<<Books as WebsocketChannel>::ArgType<'_>, <Books as WebsocketChannel>::Response<'_>> = serde_json::from_str(msg).unwrap();
            response.data
        } else {
            None
        }
    }
}