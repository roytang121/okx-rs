use crate::rest_client::model::Request;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// https://www.okx.com/docs/en/#rest-api-funding-get-currencies
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrencies {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub ccy: String,
    pub name: String,
    pub chain: String,
    pub can_dep: bool,
    pub can_wd: bool,
    pub can_internal: bool,
    pub min_dep: Decimal,
    pub min_wd: Decimal,
    pub max_wd: Decimal,
    pub wd_tick_sz: Decimal,
    pub wd_quota: Decimal,
    pub used_wd_quota: Decimal,
    pub min_fee: Decimal,
    pub max_fee: Decimal,
    pub main_net: bool,
    pub need_tag: bool,
}

impl Request for GetCurrencies {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/currencies";
    const AUTH: bool = true;

    type Response = Vec<Currency>;
}
