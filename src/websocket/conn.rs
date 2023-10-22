use crate::api::v5::BookUpdate;
use crate::websocket::WebsocketChannel;
use serde::{Deserialize, Serialize};
use serde_json::json;

// FIXME: each book type can largely be combined into single Enum

#[derive(Debug, Deserialize)]
pub struct BookChannelArg<'a> {
    pub channel: Option<&'a str>,
    pub inst_id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Books5 {
    pub inst_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Books {
    pub inst_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BboTbt {
    pub inst_id: String,
}

impl WebsocketChannel for Books {
    const CHANNEL: &'static str = "books";
    type Response<'de> = [BookUpdate<'de>; 1];
    type ArgType<'de> = BookChannelArg<'de>;

    fn subscribe_message(&self) -> String {
        let Books { inst_id } = self;
        json!({
            "op": "subscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                    "instId": inst_id,
                }
            ]
        })
        .to_string()
    }

    fn unsubscribe_message(&self) -> String {
        todo!()
    }
}

impl WebsocketChannel for Books5 {
    const CHANNEL: &'static str = "books5";
    type Response<'de> = [BookUpdate<'de>; 1];
    type ArgType<'de> = BookChannelArg<'de>;

    fn subscribe_message(&self) -> String {
        let Books5 { inst_id } = self;
        json!({
            "op": "subscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                    "instId": inst_id,
                }
            ]
        })
        .to_string()
    }

    fn unsubscribe_message(&self) -> String {
        todo!()
    }
}

impl WebsocketChannel for BboTbt {
    const CHANNEL: &'static str = "bbo-tbt";
    type Response<'de> = [BookUpdate<'de>; 1];
    type ArgType<'de> = BookChannelArg<'de>;

    fn subscribe_message(&self) -> String {
        let BboTbt { inst_id } = self;
        json!({
            "op": "subscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                    "instId": inst_id,
                }
            ]
        })
        .to_string()
    }

    fn unsubscribe_message(&self) -> String {
        todo!()
    }
}
