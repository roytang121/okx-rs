use crate::api::v5::Request;
use crate::serde_util::{deserialize_from_opt_str, deserialize_timestamp};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone)]
pub enum WithdrawalStatus {
    PendingCancel,
    Canceled,
    Failed,
    Pending,
    Sending,
    Sent,
    AwaitingEmailVerification,
    AwaitingManualVerification,
    AwaitingIdentifyVerification,
    Unknown(String),
}

impl FromStr for WithdrawalStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-3" => Self::PendingCancel,
            "-2" => Self::Canceled,
            "-1" => Self::Failed,
            "0" => Self::Pending,
            "1" => Self::Sending,
            "2" => Self::Sent,
            "3" => Self::AwaitingEmailVerification,
            "4" => Self::AwaitingManualVerification,
            "5" => Self::AwaitingIdentifyVerification,
            unknown => Self::Unknown(unknown.to_owned()),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalHistory {
    pub ccy: String,
    pub chain: String,
    pub amt: Decimal,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    pub from: String,
    pub to: String,
    pub tag: Option<String>,
    pub pmt_id: Option<String>,
    pub memo: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tx_id: Option<String>,
    pub fee: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: WithdrawalStatus,
    pub wd_id: String,
    pub client_id: Option<String>,
}

/// https://www.okx.com/docs-v5/en/#rest-api-funding-get-withdrawal-history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalHistory {}

impl Request for GetWithdrawalHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/withdrawal-history";
    const AUTH: bool = true;

    type Response = Vec<WithdrawalHistory>;
}

/// https://www.okx.com/docs/en/#rest-api-funding-withdrawal
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequest {
    pub ccy: String,
    pub amt: Decimal,
    pub dest: String,
    pub to_addr: String,
    pub fee: String,
    pub chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResponse {
    pub amt: Decimal,
    pub ccy: String,
    pub wd_id: String,
    pub client_id: String,
    pub chain: String,
}

impl Request for WithdrawalRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/asset/withdrawal";
    const AUTH: bool = true;

    type Response = Vec<WithdrawalResponse>;
}
