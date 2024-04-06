use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use crate::api::v5::model::{InstrumentType, Side};
use crate::api::v5::{ExecType, PositionSide, Request};
use crate::serde_util::str_opt;

/// https://www.okx.com/docs-v5/en/#order-book-trading-trade-get-transaction-details-last-3-days
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFillHistory {
    #[serde(default, with = "str_opt")]
    pub inst_type: Option<InstrumentType>,
    pub uly: Option<String>,
    #[serde(default, with = "str_opt")]
    pub inst_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub ord_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub after: Option<String>,
    #[serde(default, with = "str_opt")]
    pub before: Option<String>,
    #[serde(default, with = "str_opt")]
    pub begin: Option<u64>,
    #[serde(default, with = "str_opt")]
    pub end: Option<u64>,
    #[serde(default, with = "str_opt")]
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillHistory {
    pub inst_type: InstrumentType,
    pub inst_id: String,
    #[serde(default, with = "str_opt")]
    pub trade_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub ord_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub cl_ord_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub bill_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub tag: Option<String>,
    #[serde(default, with = "str_opt")]
    pub fill_px: Option<f64>,
    #[serde(default, with = "str_opt")]
    pub fill_sz: Option<f64>,
    #[serde(default, with = "str_opt")]
    pub side: Option<Side>,
    #[serde(default, with = "str_opt")]
    pub pos_side: Option<PositionSide>,
    #[serde(default, with = "str_opt")]
    pub exec_type: Option<ExecType>,
    #[serde(default, with = "str_opt")]
    pub fee_ccy: Option<String>,
    #[serde(default, with = "str_opt")]
    pub fee: Option<f64>,
    #[serde(default, with = "str_opt")]
    pub ts: Option<u64>,
}

impl Request for GetFillHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/trade/fills";
    const AUTH: bool = true;
    type Response = Vec<FillHistory>;
}
