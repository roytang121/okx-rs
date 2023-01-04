use crate::serde_util::deserialize_timestamp;
use crate::rest_client::model::{AccountType, Request};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Clone)]
pub enum DepositStatus {
    WaitingForConfirmation,
    Credited,
    Complete,
    /// pending due to temporary deposit suspension on this crypto currency
    Pending,
    Unknown(String),
}

impl FromStr for DepositStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::WaitingForConfirmation,
            "1" => Self::Credited,
            "2" => Self::Complete,
            "8" => Self::Pending,
            unknown => Self::Unknown(unknown.to_owned()),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistory {
    pub ccy: String,
    pub chain: String,
    pub amt: Decimal,
    pub from: String,
    pub to: String,
    pub tx_id: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: DepositStatus,
    pub dep_id: String,
    pub actual_dep_blk_confirm: String,
}

/// https://www.okx.com/docs-v5/en/#rest-api-funding-get-deposit-history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositHistory {}

impl Request for GetDepositHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/deposit-history";
    const AUTH: bool = true;

    type Response = Vec<DepositHistory>;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub chain: String,
    pub ccy: String,
    pub to: AccountType,
    pub addr: String,
    pub selected: bool,
}

/// https://www.okx.com/docs-v5/en/#rest-api-funding-get-deposit-history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositAddress {
    pub ccy: String,
}

impl Request for GetDepositAddress {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/deposit-address";
    const AUTH: bool = true;
    type Response = Vec<DepositAddress>;
}
