use crate::api::v5::DepositStatus;
use crate::impl_string_enum;
use crate::serde_util::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
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
impl_string_enum!(SelfTradePreventionMode,
    CancelMaker => "cancel_maker",
    CancelTaker => "cancel_taker",
    CancelBoth => "cancel_both",
);
impl_string_enum!(TransferType,
    WithinAccount => "0",
    MasterToSubAccount => "1",
    SubAccountToMaster => "2",
    SubAccountToMasterSA => "3",
    SubAccountToSubAccount => "4",
);
impl_string_enum!(AccountType,
    Funding => "6",
    Trading => "18",
);
impl_string_enum!(FundTransferState,
    Success => "success",
    Pending => "pending",
    Failed => "failed",
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SelfTradePreventionMode {
    CancelMaker,
    CancelTaker,
    CancelBoth,
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
    #[serde(rename = "state")]
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
    /// Update time of account information, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    /// The total amount of equity in USD
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub total_eq: Option<Decimal>,
    /// Isolated margin equity in USD
    // Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: Option<Decimal>,
    /// Adjusted / Effective equity in USD
    /// The net fiat value of the assets in the account that can provide margins for spot, futures, perpetual swap and options under the cross margin mode.
    /// Cause in multi-ccy or PM mode, the asset and margin requirement will all be converted to USD value to process the order check or liquidation.
    /// Due to the volatility of each currency market, our platform calculates the actual USD value of each currency based on discount rates to balance market risks.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub adj_eq: Option<Decimal>,
    /// Cross margin frozen for pending orders in USD
    /// Only applicable to Multi-currency margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub ord_froz: Option<Decimal>,
    /// Initial margin requirement in USD
    /// The sum of initial margins of all open positions and pending orders under cross margin mode in USD.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub imr: Option<Decimal>,
    /// Maintenance margin requirement in USD
    /// The sum of maintenance margins of all open positions under cross margin mode in USD.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub mmr: Option<Decimal>,
    /// Potential borrowing IMR of the account in USD
    /// Only applicable to Multi-currency margin and Portfolio margin. It is "" for other margin modes.
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub borrow_froz: Option<Decimal>,
    /// Margin ratio in USD
    /// The index for measuring the risk of a certain asset in the account.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: Option<Decimal>,
    /// Notional value of positions in USD
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
    pub notional_usd: Option<Decimal>,
    /// Detailed asset information in all currencies
    pub details: Vec<TradingBalance>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalance {
    /// Cash Balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cash_bal: Option<Decimal>,
    /// Equity of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq: Option<Decimal>,
    /// Currency
    pub ccy: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    /// Isolated margin equity of the currency
    /// Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: Option<Decimal>,
    /// Available equity of the currency
    /// The balance that can be used on margin or futures/swap trading.
    /// Applicable to Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_eq: Option<Decimal>,
    /// Discount equity of the currency in USD.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub dis_eq: Option<Decimal>,
    /// Frozen balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub fixed_bal: Option<Decimal>,
    /// Available balance of the currency
    /// The balance that can be withdrawn or transferred or used on spot trading.
    /// Applicable to Simple, Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_bal: Option<Decimal>,
    /// Frozen balance of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub frozen_bal: Option<Decimal>,
    /// Margin frozen for open orders
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ord_frozen: Option<Decimal>,
    /// Liabilities of the currency
    /// It is a positive value, e.g."21625.64". Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liab: Option<Decimal>,
    /// The sum of the unrealized profit & loss of all margin and derivatives positions of the currency.
    /// Applicable to Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl: Option<Decimal>,
    /// Liabilities due to Unrealized loss of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl_liab: Option<Decimal>,
    /// Cross liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cross_liab: Option<Decimal>,
    /// Isolated liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_liab: Option<Decimal>,
    /// Isolated liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: Option<Decimal>,
    /// Accrued interest of the currency
    /// It is a positive value, e.g."9.01". Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    /// Risk indicator of auto liability repayment
    /// Divided into multiple levels from 0 to 5, the larger the number, the more likely the auto repayment will be triggered.
    /// Applicable to Multi-currency margin and Portfolio margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub twap: Option<Decimal>,
    /// Max loan of the currency
    /// Applicable to cross of Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub max_loan: Option<Decimal>,
    /// Equity in USD of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq_usd: Option<Decimal>,
    /// Potential borrowing IMR of the currency in USD
    /// Only applicable to Multi-currency margin and Portfolio margin. It is "" for other margin modes.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub borrow_froz: Option<Decimal>,
    /// Leverage of the currency
    /// Applicable to Single-currency margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub notional_level: Option<Decimal>,
    /// Strategy equity
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub stgy_eq: Option<Decimal>,
    /// Isolated unrealized profit and loss of the currency
    /// Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_upl: Option<Decimal>,
    /// Spot in use amount
    /// Applicable to Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub spot_in_use_amt: Option<Decimal>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetail {
    /// Instrument type
    #[serde(deserialize_with = "deserialize_from_str")]
    pub inst_type: InstrumentType,
    /// Margin mode
    /// cross
    /// isolated
    pub mgn_mode: MarginMode,
    /// Position ID
    pub pos_id: String,
    /// Position side
    /// long, pos is positive
    /// short, pos is positive
    /// net (FUTURES/SWAP/OPTION: positive pos means long position and negative pos means short position. For MARGIN, pos is always positive, posCcy being base currency means long position, posCcy being quote currency means short position.)
    pub pos_side: PositionSide,
    /// Quantity of positions. In the mode of autonomous transfer from position to position, after the deposit is transferred, a position with pos of 0 will be generated
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub pos: Option<Decimal>,
    /// Base currency balance, only applicable to MARGIN（Manual transfers and Quick Margin Mode）
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub base_bal: Option<Decimal>,
    /// Quote currency balance, only applicable to MARGIN（Manual transfers and Quick Margin Mode）
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub quote_bal: Option<Decimal>,
    /// Base currency amount already borrowed, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub base_borrowed: Option<Decimal>,
    /// Base Interest, undeducted interest that has been incurred, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub base_interest: Option<Decimal>,
    /// Quote currency amount already borrowed, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub quote_borrowed: Option<Decimal>,
    /// Quote Interest, undeducted interest that has been incurred, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub quote_interest: Option<Decimal>,
    /// Position currency, only applicable to MARGIN positions.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub pos_ccy: Option<String>,
    /// Position that can be closed
    /// Only applicable to MARGIN, FUTURES/SWAP in the long-short mode and OPTION.
    /// For Margin position, the rest of sz will be SPOT trading after the liability is repaid while closing the position. Please get the available reduce-only amount from "Get maximum available tradable amount" if you want to reduce the amount of SPOT trading as much as possible.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_pos: Option<Decimal>,
    /// Average open price
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avg_px: Option<Decimal>,
    /// Latest Mark price
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mark_px: Option<Decimal>,
    /// Unrealized profit and loss calculated by mark price.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl: Option<Decimal>,
    /// Unrealized profit and loss ratio calculated by mark price.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl_ratio: Option<Decimal>,
    /// Unrealized profit and loss calculated by last price. Main usage is showing, actual value is upl.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl_last_px: Option<Decimal>,
    /// Unrealized profit and loss ratio calculated by last price.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl_ratio_last_px: Option<Decimal>,
    /// Instrument ID, e.g. BTC-USD-180216
    pub inst_id: String,
    /// Leverage, not applicable to OPTION
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub lever: Option<Decimal>,
    /// Estimated liquidation price
    /// Not applicable to OPTION
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liq_px: Option<Decimal>,
    /// Initial margin requirement, only applicable to cross.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub imr: Option<Decimal>,
    /// Margin, can be added or reduced. Only applicable to isolated.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub margin: Option<Decimal>,
    /// Margin ratio
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: Option<Decimal>,
    /// Maintenance margin requirement
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mmr: Option<Decimal>,
    /// Liabilities, only applicable to MARGIN.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liab: Option<Decimal>,
    /// Liabilities currency, only applicable to MARGIN.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liab_ccy: Option<String>,
    /// Interest. Undeducted interest that has been incurred.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    /// Last trade ID
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub trade_id: Option<String>,
    /// Option Value, only applicable to OPTION.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub opt_val: Option<Decimal>,
    /// Notional value of positions in USD
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub notional_usd: Option<Decimal>,
    /// Auto-deleveraging (ADL) indicator
    /// Divided into 5 levels, from 1 to 5, the smaller the number, the weaker the adl intensity.
    pub adl: String,
    /// Currency used for margin
    pub ccy: String,
    /// Latest traded price
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub last: Option<Decimal>,
    /// Latest underlying index price
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub idx_px: Option<Decimal>,
    /// USD price
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub usd_px: Option<Decimal>,
    /// Breakeven price
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub be_px: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub delta_bs: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub delta_pa: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub gamma_bs: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub gamma_pa: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub theta_bs: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub theta_pa: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub vega_bs: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub vega_pa: Option<Decimal>,
    /// Spot in use amount
    /// Applicable to Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub spot_in_use_amt: Option<Decimal>,
    /// Spot in use unit, e.g. BTC
    /// Applicable to Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub spot_in_use_ccy: Option<String>,
    /// External business id, e.g. experience coupon id
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub biz_ref_id: Option<String>,
    /// External business type
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub biz_ref_type: Option<String>,
    /// Realized profit and loss
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub realized_pnl: Option<Decimal>,
    /// Accumulated pnl of closing order(s)
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub pnl: Option<Decimal>,
    /// Accumulated fee
    /// Negative number represents the user transaction fee charged by the platform.Positive number represents rebate.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub fee: Option<Decimal>,
    /// Accumulated funding fee
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub funding_fee: Option<Decimal>,
    /// Latest time position was adjusted, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    /// Creation time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub c_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestAccrued {
    pub r#type: String,
    pub ccy: String,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub inst_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_mode: Option<MarginMode>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub interest: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub interest_rate: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
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

#[derive(Debug, Clone)]
pub struct Level {
    pub price: Decimal,
    pub size: Decimal,
    pub orders: usize,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Levels {
    Depth1([Level; 1]),
    Depth5([Level; 5]),
    Depths(Vec<Level>),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookUpdate<'a> {
    pub phantom: Option<&'a str>,
    // Checksum
    pub checksum: Option<i64>,
    /// Sequence ID of the current message
    pub seq_id: i64,
    /// Sequence ID of the last sent message. Only applicable to books, books-l2-tbt, books50-l2-tbt
    #[serde(default)]
    pub prev_seq_id: i64,
    /// Order book on sell side
    pub asks: Levels,
    /// Order book on bid side
    pub bids: Levels,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
}

#[cfg(test)]
mod test {
    use crate::api::v5::BookUpdate;

    #[test]
    fn size_of_levels() {
        use std::mem::size_of;
        assert_eq!(size_of::<BookUpdate>(), 480);
    }
}

/// Custom deserializer for book level
/// expecting level format: [price, size, "0", orders]
struct LevelVisitor;
impl<'de> Visitor<'de> for LevelVisitor {
    type Value = Level;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("level format: [price, size, \"0\", orders]")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        match (
            seq.next_element::<Decimal>()?,
            seq.next_element::<Decimal>()?,
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
        ) {
            (Some(price), Some(size), Some("0"), Some(n_orders)) => Ok(Level {
                price,
                size,
                orders: usize::from_str(n_orders)
                    .map_err(|_| A::Error::custom("unknown number of orders format"))?,
            }),
            _ => Err(A::Error::custom("invalid level format")),
        }
    }
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LevelVisitor)
    }
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

#[derive(Debug, Deserialize)]
pub struct ChannelArg<'a> {
    pub channel: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    /// Currency, e.g. `BTC`
    pub ccy: String,
    /// Name of currency. There is no related name when it is not shown.
    pub name: String,
    /// The logo link of currency
    pub logo_link: Option<String>,
    /// Chain name, e.g. `USDT-ERC20`, `USDT-TRC20`
    pub chain: Option<String>,
    /// The availability to deposit from chain
    pub can_dep: bool,
    /// The availability to withdraw to chain
    pub can_wd: bool,
    /// The availability to internal transfer
    pub can_internal: bool,
    /// The minimum deposit amount of the currency in a single transaction
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub min_dep: Option<Decimal>,
    /// The minimum withdrawal amount of the currency in a single transaction
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub min_wd: Option<Decimal>,
    /// The maximum amount of currency withdrawal in a single transaction
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub max_wd: Option<Decimal>,
    /// The withdrawal precision, indicating the number of digits after the decimal point.
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub wd_tick_sz: Option<Decimal>,
    /// The withdrawal limit in the past 24 hours (including `on-chain withdrawal` and `internal transfer`), unit in `USD`
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub wd_quota: Option<Decimal>,
    /// The amount of currency withdrawal used in the past 24 hours, unit in `USD`
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub used_wd_quota: Option<Decimal>,
    /// The minimum withdrawal fee for normal address
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub min_fee: Option<Decimal>,
    /// The maximum withdrawal fee for normal address
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub max_fee: Option<Decimal>,
    /// The minimum withdrawal fee for contract address
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub min_fee_for_ct_addr: Option<Decimal>,
    /// The maximum withdrawal fee for contract address
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub max_fee_for_ct_addr: Option<Decimal>,
    /// If current chain is main net, then it will return `true`, otherwise it will return `false`
    pub main_net: bool,
    /// Whether tag/memo information is required for withdrawal, e.g. `EOS` will return `true`
    pub need_tag: bool,
    /// The minimum number of blockchain confirmations to acknowledge fund deposit. The account is credited after that, but the deposit can not be withdrawn
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub min_dep_arrival_confirm: Option<Decimal>,
    /// The minimum number of blockchain confirmations required for withdrawal of a deposit
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub min_wd_unlock_confirm: Option<Decimal>,
    /// The fixed deposit limit, unit in `USD`
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub dep_quota_fixed: Option<Decimal>,
    /// The used amount of fixed deposit quota, unit in `USD`
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub used_dep_quota_fixed: Option<Decimal>,
    /// The layer2 network daily deposit limit
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub dep_quote_daily_layer2: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingBalance {
    /// Available balance
    /// The balance that can be withdrawn or transferred or used for spot trading
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avail_bal: Option<Decimal>,
    /// Balance
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub bal: Option<Decimal>,
    /// Frozen balance
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub frozen_bal: Option<Decimal>,
    /// Currency
    pub ccy: String,
}

/// Transfer type
/// 0: transfer within account
/// 1: master account to sub-account (Only applicable to API Key from master account)
/// 2: sub-account to master account (Only applicable to API Key from master account)
/// 3: sub-account to master account (Only applicable to APIKey from sub-account)
/// 4: sub-account to sub-account (Only applicable to APIKey from sub-account, and target account needs to be another sub-account which belongs to same master account. Sub-account directly transfer out permission is disabled by default, set permission please refer to Set permission of transfer out)
/// The default is 0.
/// If you want to make transfer between sub-accounts by master account API key, refer to Master accounts manage the transfers between sub-accounts
#[derive(Debug, Clone)]
pub enum TransferType {
    /// 0: transfer within account
    WithinAccount,
    /// 1: master account to sub-account (Only applicable to API Key from master account)
    MasterToSubAccount,
    /// 2: sub-account to master account (Only applicable to API Key from master account)
    SubAccountToMaster,
    /// 3: sub-account to master account (Only applicable to APIKey from sub-account)
    SubAccountToMasterSA,
    /// 4: sub-account to sub-account (Only applicable to APIKey from sub-account, and target account needs to be another sub-account which belongs to same master account. Sub-account directly transfer out permission is disabled by default, set permission please refer to Set permission of transfer out)
    SubAccountToSubAccount,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccountType {
    Funding,
    Trading,
}

#[derive(Debug, Clone)]
pub enum FundTransferState {
    Success,
    Pending,
    Failed,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundTransferHistory {
    /// Transfer ID
    pub trans_id: String,
    /// Client-supplied ID
    pub client_id: String,
    /// Currency, e.g. USDT
    pub ccy: String,
    /// Amount to be transferred
    pub amt: Decimal,
    /// Transfer type
    pub r#type: TransferType,
    pub from: AccountType,
    pub to: AccountType,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sub_acct: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: FundTransferState,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    /// Deposit address
    pub addr: String,
    /// Deposit tag (This will not be returned if the currency does not require a tag for deposit)
    pub tag: Option<String>,
    /// Deposit memo (This will not be returned if the currency does not require a payment_id for deposit)
    pub memo: Option<String>,
    /// Deposit payment ID (This will not be returned if the currency does not require a payment_id for deposit)
    pub pmt_id: Option<String>,
    /// Object Deposit address attachment (This will not be returned if the currency does not require this)
    /// e.g. TONCOIN attached tag name is comment, the return will be {'comment':'123456'}
    pub addr_ex: Option<String>,
    /// Currency, e.g. BTC
    pub ccy: String,
    /// Chain name, e.g. USDT-ERC20, USDT-TRC20
    pub chain: String,
    /// The beneficiary account
    /// 6: Funding account 18: Trading account
    pub to: AccountType,
    /// Return true if the current deposit address is selected by the website page
    pub selected: bool,
    /// Last 6 digits of contract address
    pub ct_addr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistory {
    /// Currency, e.g. BTC
    pub ccy: String,
    /// Deposit ID
    pub dep_id: String,
    /// Chain name
    pub chain: String,
    /// Deposit amount
    pub amt: Decimal,
    /// Deposite account
    // If the deposit comes from an internal transfer, this field displays the account information of the internal transfer initiator, which can be mobile phone number, email address, account name, and will return "" in other cases
    pub from: String,
    /// Deposit address
    /// If the deposit comes from the on-chain, this field displays the on-chain address, and will return "" in other cases
    pub to: String,
    /// Hash record of the deposit
    pub tx_id: String,
    /// Time that the deposit record is created, Unix timestamp format in milliseconds, e.g. 1655251200000
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: DepositStatus,
    /// Actual amount of blockchain confirm in a single deposit
    pub actual_dep_blk_confirm: String,
    /// internal transfer initiator's withdrawal ID
    /// If the deposit comes from internal transfer, this field displays the withdrawal ID of the internal transfer initiator
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub from_wd_id: Option<String>,
}
