use crate::api::v5::model::Side;
use crate::api::v5::Request;
use crate::serde_util::deserialize_from_str;
use crate::serde_util::{deserialize_from_opt_str, deserialize_timestamp};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// https://www.okx.com/docs-v5/en/#rest-api-market-data-get-index-tickers
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexTicker {
    pub inst_id: String,
    #[serde(rename = "idxPx")]
    pub index_price: Decimal,
    // high24h: Decimal,
    // sodUtc0: Decimal,
    // open24h: Decimal,
    // low24h: Decimal,
    // sodUtc8: Decimal,
    #[serde(rename = "ts", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
}

impl Request for GetIndexPrice {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/index-tickers";
    const AUTH: bool = false;

    type Response = Vec<IndexTicker>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-public-data-get-interest-rate-and-loan-quota
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestRates {}

#[derive(Debug, Clone, Deserialize)]
pub struct InterestRates {
    pub basic: Vec<BaseInterestRate>,
    pub vip: Vec<InterestRateTier>,
    pub regular: Vec<InterestRateTier>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BaseInterestRate {
    #[serde(rename = "ccy")]
    pub asset: String,
    pub quota: Decimal,
    pub rate: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InterestRateTier {
    #[serde(rename = "irDiscount", deserialize_with = "deserialize_from_opt_str")]
    pub discount: Option<Decimal>,
    #[serde(rename = "loanQuotaCoef", deserialize_with = "deserialize_from_str")]
    pub loan_quota_coef: Decimal,
    pub level: String,
}

impl Request for GetInterestRates {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/interest-rate-loan-quota";
    const AUTH: bool = false;

    type Response = Vec<InterestRates>;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub inst_id: String,
    pub trade_id: String,
    pub px: Decimal,
    pub sz: Decimal,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub side: Side,
    #[serde(deserialize_with = "crate::serde_util::deserialize_timestamp")]
    pub ts: DateTime<Utc>,
}

/// https://www.okx.com/docs-v5/en/#rest-api-market-data-get-trades-history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTrades {
    pub inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl Request for GetTrades {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/history-trades";
    type Response = Vec<TradeHistory>;
}
