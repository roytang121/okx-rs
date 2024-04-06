use crate::serde_util::str_opt;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

pub mod ws_convert;

pub mod funding_account;
pub mod model;
pub mod orderbook_trading;
pub mod public_data;
pub mod testkit;
pub mod trading_account;

pub use self::model::*;
// re-export funding_account module
pub use self::funding_account::bill::*;
pub use self::funding_account::deposit::*;
pub use self::funding_account::transfer::*;
pub use self::funding_account::withdrawal::*;
// re-export trading_account module
pub use self::trading_account::rest::*;
pub use self::trading_account::websocket::*;
// re-export public data module
pub use self::public_data::rest::*;
pub use self::public_data::websocket::*;
// re-export trading module
pub use self::orderbook_trading::fill::*;
pub use self::orderbook_trading::market_data::*;
pub use self::orderbook_trading::orders::*;

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str;
    const AUTH: bool = false;

    type Response: DeserializeOwned + Debug;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(default, with = "str_opt")]
    pub code: Option<u64>,
    #[serde(default, with = "str_opt")]
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsResponse<'a, A: Debug, T: Debug> {
    pub id: Option<&'a str>,
    pub op: Option<&'a str>,
    pub arg: Option<A>,
    #[serde(default, with = "str_opt")]
    pub code: Option<u64>,
    pub conn_id: Option<&'a str>,
    pub event: Option<&'a str>,
    pub action: Option<&'a str>,
    pub data: Option<T>,
    pub msg: Option<&'a str>,
}
