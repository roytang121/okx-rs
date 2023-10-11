use crate::api::v5::model::{
    InstrumentType, InterestAccrued, InterestLimitResponse, MarginMode, PositionDetail,
    TradingBalanceDetail,
};
use crate::api::v5::Request;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-balance
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTradingBalances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl Request for GetTradingBalances {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/balance";
    const AUTH: bool = true;
    type Response = Vec<TradingBalanceDetail>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-positions
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositions {
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub inst_type: Option<InstrumentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
}

impl Request for GetPositions {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/positions";
    const AUTH: bool = true;
    type Response = Vec<PositionDetail>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-interest-accrued-data
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestAccrued {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
    mgn_mode: Option<MarginMode>,
}

impl Request for GetInterestAccrued {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/interest-accrued";
    const AUTH: bool = true;
    type Response = Vec<InterestAccrued>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-borrow-interest-and-limit
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl Request for GetInterestLimits {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/interest-limits";
    const AUTH: bool = true;
    type Response = Vec<InterestLimitResponse>;
}
