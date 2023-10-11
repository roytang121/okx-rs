use crate::impl_string_enum;
use crate::serde_util::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl_string_enum!(InstrumentType,
    Spot => "SPOT",
    Margin => "MARGIN",
    Swap => "SWAP",
    Futures => "FUTURES",
    Option => "OPTION",
);
impl_string_enum!(Side,
    Buy => "buy",
    Sell => "sell",
);
impl_string_enum!(PositionSide,
    Long => "long",
    Short => "short",
    Net => "net",
);
impl_string_enum!(MarginMode,
    Cross => "cross",
    Isolated => "isolated",
);
impl_string_enum!(TradeMode,
    Cross => "cross",
    Isolated => "isolated",
    Cash => "cash",
);
impl_string_enum!(OrderType,
    Market => "market",
    Limit => "limit",
    PostOnly => "post_only",
    Fok => "fok",
    Ioc => "ioc",
    OptimalLimitIoc => "optimal_limit_ioc",
);
impl_string_enum!(QuantityType,
    BaseCcy => "base_ccy",
    QuoteCcy => "quote_ccy",
);
impl_string_enum!(OrderState,
    Canceled => "canceled",
    Live => "live",
    PartiallyFilled => "partially_filled",
    Filled => "filled",
);
impl_string_enum!(TakeProfitTriggerPriceType,
    Last => "last",
    Index => "index",
    Mark => "mark",
);
impl_string_enum!(StopLossTriggerPriceType,
    Last => "last",
    Index => "index",
    Mark => "mark",
);
impl_string_enum!(Category,
    Normal => "normal",
    Twap => "twap",
    Adl => "adl",
    FullLiquidation => "full_liquidation",
    PartialLiquidation => "partial_liquidation",
    Delivery => "delivery",
    Ddh => "ddh",
);
impl_string_enum!(InstrumentStatus,
    Live => "live",
    Suspend => "suspend",
    Preopen => "preopen",
    Test => "test",
);
impl_string_enum!(OptionType,
    Call => "C",
    Put => "P",
);
impl_string_enum!(ContractType,
    Linear => "linear",
    Inverse => "inverse",
);
impl_string_enum!(FutureType,
    ThisWeek => "this_week",
    NextWeek => "next_week",
    Quarter => "quarter",
    NextQuarter => "next_quarter",
);
impl_string_enum!(DeliveryExerciseHistoryType,
    Delivery => "delivery",
    Exercised => "exercised",
    ExpiredOtm => "expired_otm",
);
impl_string_enum!(CandleState,
    Uncompleted => "0",
    Completed => "1",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    Long,
    Short,
    Net,
}

#[derive(Debug, Clone, Copy)]
pub enum MarginMode {
    Cross,
    Isolated,
}

#[derive(Debug, Clone, Copy)]
pub enum TradeMode {
    Cross,
    Isolated,
    Cash,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
    OptimalLimitIoc,
}

#[derive(Debug, Clone, Copy)]
pub enum QuantityType {
    BaseCcy,
    QuoteCcy,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
}

#[derive(Debug, Clone, Copy)]
pub enum TakeProfitTriggerPriceType {
    Last,
    Index,
    Mark,
}

#[derive(Debug, Clone, Copy)]
pub enum StopLossTriggerPriceType {
    Last,
    Index,
    Mark,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentStatus {
    // live
    Live,
    // suspend
    Suspend,
    // preopen. e.g. There will be preopen before the Futures and Options new contracts state is live.
    Preopen,
    // test: Test pairs, can't be traded
    Test,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContractType {
    Linear,
    Inverse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FutureType {
    ThisWeek,
    NextWeek,
    Quarter,
    NextQuarter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeliveryExerciseHistoryType {
    /// Delivery
    Delivery,
    /// Exercised
    Exercised,
    /// Expired out of the money
    ExpiredOtm,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CandleState {
    Uncompleted,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "instType")]
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
    #[serde(rename = "state", deserialize_with = "deserialize_from_str")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryExerciseHistory {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    pub details: Vec<DeliveryExerciseHistoryDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryExerciseHistoryDetail {
    pub inst_id: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub px: Decimal,
    pub r#type: DeliveryExerciseHistoryType,
}

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
    #[serde(deserialize_with = "deserialize_from_str")]
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
    #[serde(deserialize_with = "deserialize_from_str")]
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
    #[serde(deserialize_with = "deserialize_from_str")]
    pub pos_side: PositionSide,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub upl: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub upl_ratio: Option<Decimal>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestLimitResponse {
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub debt: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    pub records: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    // Instrument type
    pub inst_type: InstrumentType,
    // instrument ID, e.g. BTC-USD-200626
    pub inst_id: String,
    // Open interest (cont)
    pub oi: Decimal,
    // Open interest (coin)
    pub oi_ccy: Decimal,
    // Data return time,  Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "ts", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    // Instrument ID, e.g. BTC-USD-SWAP
    pub inst_id: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    // Instrument type
    pub inst_type: InstrumentType,
    // Current funding rate
    pub funding_rate: Decimal,
    // Settlement time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    // Settlement time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub funding_time: DateTime<Utc>,
    // Forecasted funding rate for the next period
    pub next_funding_rate: Decimal,
    // Forecasted funding time for the next period , Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub next_funding_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    // Instrument type. SWAP
    pub inst_type: InstrumentType,
    // Instrument ID, e.g. BTC-USD-SWAP
    pub inst_id: String,
    // Predicted funding rate
    pub funding_rate: Decimal,
    // Actual funding rate
    pub realized_rate: Decimal,
    // Settlement time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub funding_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimit {
    // Instrument type
    // SWAP
    // FUTURES
    // OPTION
    pub inst_type: InstrumentType,
    // Instrument ID, e.g. BTC-USD-SWAP
    // only applicable to FUTURES/SWAP/OPTION
    pub inst_id: String,
    // Highest buy limit
    pub buy_lmt: Decimal,
    // Lowest sell limit
    pub sell_lmt: Decimal,
    // Data return time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountRateAndInterestFreeQuota {
    // Currency
    pub ccy: String,
    // Interest-free quota
    pub amt: Decimal,
    // Discount rate level
    // 1:level 1
    // 2:level 2
    // 3:level 3
    // 4:level 4
    // 5:level 5
    pub discount_lv: u32,
    // Discount details
    pub discount_info: Vec<DiscountInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OKXSystemTime {
    // System time
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountInfo {
    // Discount rate
    pub discount_rate: Decimal,
    // Tier - upper bound, "" means positive infinity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amt: Option<Decimal>,
    // Tier - lower bound, the minimum is 0
    pub min_amt: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    /// Instrument ID, e.g. BTC-USD-200214
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub inst_type: InstrumentType,
    /// Instrument type
    /// MARGIN
    /// SWAP
    /// FUTURES
    /// OPTION
    pub inst_id: String,
    #[serde(rename = "markPx")]
    /// Mark price
    pub mark_price: Decimal,
    /// Data return time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "ts", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTier {
    /// Underlying
    /// Applicable to FUTURES/SWAP/OPTION
    pub uly: String,
    /// Instrument family
    /// Applicable to FUTURES/SWAP/OPTION
    pub inst_family: String,
    /// Instrument ID
    pub inst_id: String,
    /// Tiers
    pub tier: String,
    /// The minimum borrowing amount or position of this gear is only applicable to margin/options/perpetual/delivery, the minimum position is 0 by default
    /// It will return the minimum borrowing amount when ccy takes effect.
    pub min_sz: Decimal,
    /// The maximum borrowing amount or number of positions held in this position is only applicable to margin/options/perpetual/delivery
    /// It will return the maximum borrowing amount when ccy takes effect.
    pub max_sz: Decimal,
    /// Maintenance margin requirement rate
    pub mmr: Decimal,
    /// Initial margin requirement rate
    pub imr: Decimal,
    /// Maximum available leverage
    pub max_lever: Decimal,
    /// Option Margin Coefficient (only applicable to options)
    pub opt_mgn_factor: Option<Decimal>,
    /// Quote currency borrowing amount (only applicable to leverage and the case when instId takes effect)
    pub quote_max_loan: Option<Decimal>,
    /// Base currency borrowing amount (only applicable to leverage and the case when instId takes effect)
    pub base_max_loan: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceFund {
    /// The total balance of insurance fund, in USD
    pub total: Decimal,
    /// Instrument family
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Insurance fund data
    pub details: Vec<InsuranceFundDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceFundDetail {
    /// The balance of insurance fund
    pub balance: String,
    /// The change in the balance of insurance fund
    pub amt: Decimal,
    /// The currency of insurance fund
    pub ccy: String,
    /// The type of insurance fund
    pub r#type: String,
    /// The update timestamp of insurance fund. Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexTicker {
    /// Index
    pub inst_id: String,
    /// Latest index price
    pub idx_px: Decimal,
    /// Highest price in the past 24 hours
    pub high_24h: Decimal,
    /// Lowest price in the past 24 hours
    pub low_24h: Decimal,
    /// Open price in the past 24 hours
    pub open_24h: Decimal,
    /// Open price in the UTC 0
    pub sod_utc0: Decimal,
    /// Open price in the UTC 8
    pub sod_utc8: Decimal,
    /// Index price update time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    /// Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    /// Open price
    #[serde(rename = "o")]
    pub open: Decimal,
    /// highest price
    #[serde(rename = "h")]
    pub high: Decimal,
    /// Lowest price
    #[serde(rename = "l")]
    pub low: Decimal,
    /// Close price
    #[serde(rename = "c")]
    pub close: Decimal,
    /// The state of candlesticks.
    /// 0 represents that it is uncompleted, 1 represents that it is completed.
    pub confirm: CandleState,
}

/// Custom deserializer for candlestick
/// expecting candle format: [ts, open, high, low, close, confirm]
struct CandleVisitor;
impl<'de> Visitor<'de> for CandleVisitor {
    type Value = Candle;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("candle of format: [ts, open, high, low, close, confirm]")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        match (
            seq.next_element::<&str>()?,
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
        ) {
            (Some(ts), Some(open), Some(high), Some(low), Some(close), Some(confirm)) => {
                let ts_milli =
                    i64::from_str(ts).map_err(|_| S::Error::custom("unknown timestamp format"))?;
                let ts_utc = Utc.from_utc_datetime(
                    &NaiveDateTime::from_timestamp_millis(ts_milli)
                        .ok_or(S::Error::custom("unknown timestamp format"))?,
                );
                Ok(Candle {
                    ts: ts_utc,
                    open,
                    high,
                    low,
                    close,
                    confirm,
                })
            }
            _ => Err(serde::de::Error::custom("invalid candle format")),
        }
    }
}

impl<'de> Deserialize<'de> for Candle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CandleVisitor)
    }
}

#[cfg(test)]
mod tests_parse_candle {
    use crate::api::v5::model::Candle;

    #[test]
    /// test deserialization of candle from json string to Candle struct
    fn test_deser_candle() {
        let json = r#"["1597026383085","3.721","3.743","3.677","3.708","0"]"#;
        let candle: Candle = serde_json::from_str(json).unwrap();
        assert_eq!(candle.ts.timestamp_millis(), 1597026383085);
        assert_eq!(candle.open, "3.721".parse().unwrap());
        assert_eq!(candle.high, "3.743".parse().unwrap());
        assert_eq!(candle.low, "3.677".parse().unwrap());
        assert_eq!(candle.close, "3.708".parse().unwrap());
        assert_eq!(candle.confirm, super::CandleState::Uncompleted);
    }
}
