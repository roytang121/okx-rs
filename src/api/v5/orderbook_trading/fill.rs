use crate::api::v5::model::{InstrumentType, Side};
use crate::api::v5::Request;
use crate::serde_util::{deserialize_timestamp, FloatOpt, serialize_timestamp};
use anyhow::bail;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum ExecType {
    Taker,
    Maker,
}

impl FromStr for ExecType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "T" => Self::Taker,
            "M" => Self::Maker,
            other => bail!("unknown Side {other}"),
        })
    }
}

/// https://www.okx.com/docs-v5/en/#rest-api-trade-get-transaction-details-last-3-months
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFillHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(
        serialize_with = "serialize_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub begin: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillHistory {
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub inst_type: InstrumentType,
    pub inst_id: String,
    pub trade_id: String,
    pub ord_id: String,
    pub cl_ord_id: String,
    pub bill_id: String,
    pub tag: String,
    #[serde(default)]
    pub fill_px: FloatOpt,
    #[serde(default)]
    pub fill_sz: FloatOpt,
    pub side: Side,
    #[serde(default, deserialize_with = "crate::serde_util::deserialize_from_opt_str")]
    pub pos_side: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub exec_type: ExecType,
    #[serde(default, deserialize_with = "crate::serde_util::deserialize_from_opt_str")]
    pub fee_ccy: Option<String>,
    #[serde(default)]
    pub fee: FloatOpt,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
}

impl Request for GetFillHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/trade/fills";
    const AUTH: bool = true;
    type Response = Vec<FillHistory>;
}
