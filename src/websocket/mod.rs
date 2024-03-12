use std::fmt::Debug;

use anyhow::bail;
use serde::Deserialize;

use crate::api::credential::Credential;
use crate::api::options::Options;

pub mod conn;

pub trait WebsocketChannel: Send + Sync {
    const CHANNEL: &'static str;
    const AUTH: bool = false;
    type Response<'de>: Deserialize<'de> + Debug;
    type ArgType<'de>: Deserialize<'de> + Debug;

    fn subscribe_message(&self) -> String {
        serde_json::json!({
            "op": "subscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                }
            ]
        })
        .to_string()
    }

    fn unsubscribe_message(&self) -> String {
        serde_json::json!({
            "op": "unsubscribe",
            "args": [
                {
                    "channel": Self::CHANNEL,
                }
            ]
        })
        .to_string()
    }
    fn is_private(&self) -> bool {
        Self::AUTH
    }
}

pub struct OKXAuth;
impl OKXAuth {
    pub fn ws_auth(options: Options) -> anyhow::Result<String> {
        let credential: Credential = match (&options).try_into() {
            Ok(credential) => credential,
            Err(_) => bail!("Invalid credential"),
        };
        let timestamp = format!("{}", chrono::Utc::now().timestamp_millis() / 1000);
        let (key, signature) =
            credential.signature_ws(reqwest::Method::GET, &timestamp, "/users/self/verify");

        // FIXME: just do a simple r## string
        Ok(serde_json::json!({
            "op": "login",
            "args": [
                {
                  "apiKey": key,
                  "passphrase": options.passphrase.unwrap(),
                  "timestamp": timestamp,
                  "sign": signature,
                }
            ]
        })
        .to_string())
    }
}
