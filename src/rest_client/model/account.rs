use crate::rest_client::model::{InstrumentType, MarginMode, PositionSide, Request};
use crate::serde_util::{deserialize_from_opt_str, deserialize_timestamp};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalanceDetail {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub total_eq: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub adj_eq: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub ord_froz: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub imr: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mmr: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub notional_usd: Option<Decimal>,
    pub details: Vec<TradingBalance>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalance {
    /// Available balance of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_bal: Option<Decimal>,
    /// Cash Balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cash_bal: Option<Decimal>,
    /// Equity of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq: Option<Decimal>,
    /// Margin ratio in USD
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: Option<Decimal>,
    /// Currency
    pub ccy: String,
}

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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetail {
    pub adl: String,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avail_pos: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avg_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub c_time: DateTime<Utc>,
    pub ccy: String,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub delta_bs: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub delta_pa: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub gamma_bs: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub gamma_pa: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub theta_bs: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub theta_pa: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub vega_bs: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub vega_pa: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub imr: Option<Decimal>,
    pub inst_id: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub inst_type: InstrumentType,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub usd_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub last: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub lever: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub liab: Option<Decimal>,
    pub liab_ccy: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub liq_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mark_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub margin: Option<Decimal>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub mgn_mode: MarginMode,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mmr: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub notional_usd: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub opt_val: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_timestamp")]
    // pub p_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pos: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pos_ccy: Option<String>,
    pub pos_id: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub pos_side: PositionSide,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub upl: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub upl_ratio: Option<Decimal>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestAccrued {
    pub r#type: String,
    pub ccy: String,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub inst_id: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mgn_mode: Option<MarginMode>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub interest_rate: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub liab: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct InterestLimitResponse {
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub debt: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    pub records: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InterestLimit {
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avail_loan: Option<Decimal>,
    pub ccy: String,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub loan_quota: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pos_loan: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub rate: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub surplus_lmt: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub used_lmt: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub used_loan: Option<Decimal>,
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
