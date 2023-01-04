use crate::impl_serde_from_str;
use anyhow::bail;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub mod account;
pub mod bill;
pub mod currencies;
pub mod deposit;
pub mod fill;
pub mod fund;
pub mod instruments;
pub mod market_data;
pub mod orders;
pub mod withdrawal;

pub use self::account::*;
pub use self::bill::*;
pub use self::currencies::*;
pub use self::deposit::*;
pub use self::fill::*;
pub use self::fund::*;
pub use self::instruments::*;
pub use self::market_data::*;
pub use self::orders::*;
pub use self::withdrawal::*;

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str;
    const AUTH: bool = false;

    type Response: DeserializeOwned + Debug;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub code: u32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
}

impl FromStr for InstrumentType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "SPOT" => Self::Spot,
            "MARGIN" => Self::Margin,
            "SWAP" => Self::Swap,
            "FUTURES" => Self::Futures,
            "OPTION" => Self::Option,
            other => bail!("unknown instrument type {other}"),
        })
    }
}

impl Display for InstrumentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InstrumentType::Spot => write!(f, "SPOT"),
            InstrumentType::Margin => write!(f, "MARGIN"),
            InstrumentType::Swap => write!(f, "SWAP"),
            InstrumentType::Futures => write!(f, "FUTURES"),
            InstrumentType::Option => write!(f, "OPTION"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

impl FromStr for Side {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "buy" => Self::Buy,
            "sell" => Self::Sell,
            other => bail!("unknown Side {other}"),
        })
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "buy"),
            Side::Sell => write!(f, "sell"),
        }
    }
}
impl_serde_from_str!(Side);

#[derive(Debug, Clone, Copy)]
pub enum PositionSide {
    Long,
    Short,
    Net,
}

impl FromStr for PositionSide {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "long" => Self::Long,
            "short" => Self::Short,
            "net" => Self::Net,
            other => bail!("unknown Side {other}"),
        })
    }
}

impl Display for PositionSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionSide::Long => write!(f, "long"),
            PositionSide::Short => write!(f, "short"),
            PositionSide::Net => write!(f, "net"),
        }
    }
}
impl_serde_from_str!(PositionSide);

#[derive(Debug, Clone, Copy)]
pub enum MarginMode {
    Cross,
    Isolated,
}

impl FromStr for MarginMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cross" => Self::Cross,
            "isolated" => Self::Isolated,
            other => bail!("unknown margin mode {other}"),
        })
    }
}

impl Display for MarginMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MarginMode::Cross => write!(f, "cross"),
            MarginMode::Isolated => write!(f, "isolated"),
        }
    }
}
impl_serde_from_str!(MarginMode);

#[derive(Debug, Clone, Copy)]
pub enum TradeMode {
    Cross,
    Isolated,
    Cash,
}

impl FromStr for TradeMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cross" => Self::Cross,
            "isolated" => Self::Isolated,
            "cash" => Self::Cash,
            other => bail!("unknown TradeMode {other}"),
        })
    }
}

impl Display for TradeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeMode::Cross => write!(f, "cross"),
            TradeMode::Isolated => write!(f, "isolated"),
            TradeMode::Cash => write!(f, "cash"),
        }
    }
}
impl_serde_from_str!(TradeMode);

#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
    OptimalLimitIoc,
}

impl FromStr for OrderType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "market" => Self::Market,
            "limit" => Self::Limit,
            "post_only" => Self::PostOnly,
            "fok" => Self::Fok,
            "ioc" => Self::Ioc,
            "optimal_limit_ioc" => Self::OptimalLimitIoc,
            other => bail!("unknown OrderType {other}"),
        })
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "market"),
            OrderType::Limit => write!(f, "limit"),
            OrderType::PostOnly => write!(f, "post_only"),
            OrderType::Fok => write!(f, "fok"),
            OrderType::Ioc => write!(f, "ioc"),
            OrderType::OptimalLimitIoc => write!(f, "optimal_limit_ioc"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuantityType {
    BaseCcy,
    QuoteCcy,
}

impl FromStr for QuantityType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "base_ccy" => Self::BaseCcy,
            "quote_ccy" => Self::QuoteCcy,
            other => bail!("unknown QuantityType {other}"),
        })
    }
}

impl Display for QuantityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantityType::BaseCcy => write!(f, "base_ccy"),
            QuantityType::QuoteCcy => write!(f, "quote_ccy"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OrderState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
}

impl FromStr for OrderState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "canceled" => Self::Canceled,
            "live" => Self::Live,
            "partially_filled" => Self::PartiallyFilled,
            "filled" => Self::Filled,
            other => bail!("unknown OrderState {other}"),
        })
    }
}

impl Display for OrderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderState::Canceled => write!(f, "canceled"),
            OrderState::Live => write!(f, "live"),
            OrderState::PartiallyFilled => write!(f, "partially_filled"),
            OrderState::Filled => write!(f, "filled"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TakeProfitTriggerPriceType {
    Last,
    Index,
    Mark,
}

impl FromStr for TakeProfitTriggerPriceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "last" => Self::Last,
            "index" => Self::Index,
            "mark" => Self::Mark,
            other => bail!("unknown TakeProfitTriggerPriceType {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StopLossTriggerPriceType {
    Last,
    Index,
    Mark,
}

impl FromStr for StopLossTriggerPriceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "last" => Self::Last,
            "index" => Self::Index,
            "mark" => Self::Mark,
            other => bail!("unknown StopLossTriggerPriceType {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Normal,
    Twap,
    Adl,
    FullLiquidation,
    PartialLiquidation,
    Delivery,
    Ddh,
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "normal" => Self::Normal,
            "twap" => Self::Twap,
            "adl" => Self::Adl,
            "full_liquidation" => Self::FullLiquidation,
            "partial_liquidation" => Self::PartialLiquidation,
            "delivery" => Self::Delivery,
            "ddh" => Self::Ddh,
            other => bail!("unknown Category {other}"),
        })
    }
}
