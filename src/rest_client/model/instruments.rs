use std::str::FromStr;

use crate::{
    serde_util::{deserialize_from_opt_str, deserialize_timestamp, deserialize_timestamp_opt},
    rest_client::model::Request,
};
use anyhow::bail;
use rust_decimal::Decimal;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::InstrumentType;

/// https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstruments {
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub inst_type: InstrumentType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Instrument {
    #[serde(
        rename = "instType",
        deserialize_with = "crate::serde_util::deserialize_from_str"
    )]
    pub inst_type: InstrumentType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "uly", deserialize_with = "deserialize_from_opt_str")]
    pub underlying: Option<String>, // Only applicable to FUTURES/SWAP/OPTION
    pub category: String, // Fee schedule
    #[serde(rename = "baseCcy", deserialize_with = "deserialize_from_opt_str")]
    pub base_currency: Option<String>, // Only applicable to SPOT/MARGIN
    #[serde(rename = "quoteCcy", deserialize_with = "deserialize_from_opt_str")]
    pub quote_currency: Option<String>, // Only applicable to SPOT/MARGIN
    #[serde(rename = "settleCcy", deserialize_with = "deserialize_from_opt_str")]
    pub margin_currency: Option<String>, // Settlement and margin currency; Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctVal", deserialize_with = "deserialize_from_opt_str")]
    pub face_value: Option<Decimal>, // Contract value; Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctMult", deserialize_with = "deserialize_from_opt_str")]
    pub contract_multiplier: Option<Decimal>, // Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctValCcy", deserialize_with = "deserialize_from_opt_str")]
    pub contract_value_currency: Option<String>, // Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "optType", deserialize_with = "deserialize_from_opt_str")]
    pub option_type: Option<OptionType>, // Only applicable to OPTION
    #[serde(rename = "stk", deserialize_with = "deserialize_from_opt_str")]
    pub strike_price: Option<Decimal>, // Only applicable to OPTION
    #[serde(rename = "listTime", deserialize_with = "deserialize_timestamp")]
    pub listing_time: DateTime<Utc>,
    #[serde(rename = "expTime", deserialize_with = "deserialize_timestamp_opt")]
    pub expiry_time: Option<DateTime<Utc>>,
    #[serde(rename = "lever", deserialize_with = "deserialize_from_opt_str")]
    pub max_leverage: Option<Decimal>, // Only applicable to FUTURES/OPTION; Not applicable to SPOT, OPTION
    #[serde(rename = "tickSz")]
    pub tick_size: Decimal,
    #[serde(rename = "lotSz")]
    pub lot_size: Decimal,
    #[serde(rename = "minSz")]
    pub min_size: Decimal,
    #[serde(rename = "ctType", deserialize_with = "deserialize_from_opt_str")]
    pub contract_type: Option<ContractType>, // Only applicable to FUTURES/SWAP
    #[serde(rename = "alias", deserialize_with = "deserialize_from_opt_str")]
    pub future_type: Option<FutureType>, // Only applicable to FUTURES
    #[serde(
        rename = "state",
        deserialize_with = "crate::serde_util::deserialize_from_str"
    )]
    pub status: InstrumentStatus,
    #[serde(rename = "maxLmtSz")]
    pub max_lmt_size: Decimal, // The maximum order quantity of the contract or spot limit order
    #[serde(rename = "maxMktSz")]
    pub max_mkt_size: Decimal, // The maximum order quantity of the contract or spot market order
    #[serde(rename = "maxTwapSz")]
    pub max_twap_size: Decimal, // The maximum order quantity of the contract or spot twap order
    #[serde(rename = "maxIcebergSz")]
    pub max_iceberg_size: Decimal, // The maximum order quantity of the contract or spot iceBerg order
    #[serde(rename = "maxTriggerSz")]
    pub max_trigger_size: Decimal, // The maximum order quantity of the contract or spot trigger order
    #[serde(rename = "maxStopSz")]
    pub max_stop_size: Decimal, // The maximum order quantity of the contract or spot stop order
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentStatus {
    Live,
    Suspend,
    Preopen,
}

impl FromStr for InstrumentStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "live" => Self::Live,
            "suspend" => Self::Suspend,
            "preopen" => Self::Preopen,
            other => bail!("unknown InstrumentStatus {other}"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionType {
    Call,
    Put,
}

impl FromStr for OptionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "C" => Self::Call,
            "P" => Self::Put,
            other => bail!("unknown OptionType {other}"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContractType {
    Linear,
    Inverse,
}

impl FromStr for ContractType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "linear" => Self::Linear,
            "inverse" => Self::Inverse,
            other => bail!("unknown ContractType {other}"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FutureType {
    ThisWeek,
    NextWeek,
    Quarter,
    NextQuarter,
}

impl FromStr for FutureType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "this_week" => Self::ThisWeek,
            "next_week" => Self::NextWeek,
            "quarter" => Self::Quarter,
            "next_quarter" => Self::NextQuarter,
            other => bail!("unknown FutureType {other}"),
        })
    }
}

impl Request for GetInstruments {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/instruments";
    const AUTH: bool = false;

    type Response = Vec<Instrument>;
}

#[cfg(test)]
mod test {
    #[test]
    fn get_okx_instrument() {}
}
