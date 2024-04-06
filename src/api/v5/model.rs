use crate::api::v5::DepositStatus;
use crate::impl_string_enum;
use crate::serde_util::*;
use crate::time::UTCDateTime;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Unknown;
impl Display for Unknown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown")
    }
}
impl FromStr for Unknown {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}

impl From<&str> for Unknown {
    fn from(_: &str) -> Self {
        Self
    }
}

impl_string_enum!(InstrumentType,
    Spot => "SPOT",
    Margin => "MARGIN",
    Swap => "SWAP",
    Futures => "FUTURES",
    Option => "OPTION",
    Any => "ANY",
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
    Other,
    BaseCcy => "base_ccy",
    QuoteCcy => "quote_ccy",
);
impl_string_enum!(OrderState,
    Other,
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
impl_string_enum!(SubAccountBillType,
    MasterToSubAccount => "0",
    SubAccountToMaster => "1",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionSide {
    Long,
    Short,
    Net,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum MarginMode {
    Cross,
    Isolated,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum TradeMode {
    Cross,
    Isolated,
    Cash,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
    OptimalLimitIoc,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum QuantityType {
    BaseCcy,
    QuoteCcy,
    Other(Unknown),
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum OrderState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
    Other(Unknown),
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum TakeProfitTriggerPriceType {
    Last,
    Index,
    Mark,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum StopLossTriggerPriceType {
    Last,
    Index,
    Mark,
}

#[derive(Debug, Clone, Copy, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubAccountBillType {
    MasterToSubAccount,
    SubAccountToMaster,
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
    #[serde(rename = "ctVal", default)]
    pub face_value: MaybeFloat, // Contract value; Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctMult", default)]
    pub contract_multiplier: MaybeFloat, // Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctValCcy", deserialize_with = "deserialize_from_opt_str")]
    pub contract_value_currency: Option<String>, // Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "optType", deserialize_with = "deserialize_from_opt_str")]
    pub option_type: Option<OptionType>, // Only applicable to OPTION
    #[serde(rename = "stk", default)]
    pub strike_price: MaybeFloat, // Only applicable to OPTION
    #[serde(rename = "listTime", default)]
    pub listing_time: MaybeU64,
    #[serde(rename = "expTime", default)]
    pub expiry_time: MaybeU64,
    #[serde(rename = "lever", default)]
    pub max_leverage: MaybeFloat, // Only applicable to FUTURES/OPTION; Not applicable to SPOT, OPTION
    #[serde(rename = "tickSz", default)]
    pub tick_size: MaybeFloat,
    #[serde(rename = "lotSz", default)]
    pub lot_size: MaybeFloat,
    #[serde(rename = "minSz", default)]
    pub min_size: MaybeFloat,
    #[serde(rename = "ctType", deserialize_with = "deserialize_from_opt_str")]
    pub contract_type: Option<ContractType>, // Only applicable to FUTURES/SWAP
    #[serde(rename = "alias", deserialize_with = "deserialize_from_opt_str")]
    pub future_type: Option<FutureType>, // Only applicable to FUTURES
    #[serde(rename = "state")]
    pub status: InstrumentStatus,
    #[serde(rename = "maxLmtSz", default)]
    pub max_lmt_size: MaybeFloat, // The maximum order quantity of the contract or spot limit order
    #[serde(rename = "maxMktSz", default)]
    pub max_mkt_size: MaybeFloat, // The maximum order quantity of the contract or spot market order
    #[serde(rename = "maxTwapSz", default)]
    pub max_twap_size: MaybeFloat, // The maximum order quantity of the contract or spot twap order
    #[serde(rename = "maxIcebergSz", default)]
    pub max_iceberg_size: MaybeFloat, // The maximum order quantity of the contract or spot iceBerg order
    #[serde(rename = "maxTriggerSz", default)]
    pub max_trigger_size: MaybeFloat, // The maximum order quantity of the contract or spot trigger order
    #[serde(rename = "maxStopSz", default)]
    pub max_stop_size: MaybeFloat, // The maximum order quantity of the contract or spot stop order
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryExerciseHistory {
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ts: MaybeU64,
    pub details: Vec<DeliveryExerciseHistoryDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryExerciseHistoryDetail {
    pub inst_id: String,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub px: MaybeFloat,
    pub r#type: DeliveryExerciseHistoryType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalanceDetail {
    /// Update time of account information, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub u_time: Option<u64>,
    /// The total amount of equity in USD
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub total_eq: MaybeFloat,
    /// Isolated margin equity in USD
    // Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: MaybeFloat,
    /// Adjusted / Effective equity in USD
    /// The net fiat value of the assets in the account that can provide margins for spot, futures, perpetual swap and options under the cross margin mode.
    /// Cause in multi-ccy or PM mode, the asset and margin requirement will all be converted to USD value to process the order check or liquidation.
    /// Due to the volatility of each currency market, our platform calculates the actual USD value of each currency based on discount rates to balance market risks.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub adj_eq: MaybeFloat,
    /// Cross margin frozen for pending orders in USD
    /// Only applicable to Multi-currency margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ord_froz: MaybeFloat,
    /// Initial margin requirement in USD
    /// The sum of initial margins of all open positions and pending orders under cross margin mode in USD.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub imr: MaybeFloat,
    /// Maintenance margin requirement in USD
    /// The sum of maintenance margins of all open positions under cross margin mode in USD.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mmr: MaybeFloat,
    /// Potential borrowing IMR of the account in USD
    /// Only applicable to Multi-currency margin and Portfolio margin. It is "" for other margin modes.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub borrow_froz: MaybeFloat,
    /// Margin ratio in USD
    /// The index for measuring the risk of a certain asset in the account.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: MaybeFloat,
    /// Notional value of positions in USD
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub notional_usd: MaybeFloat,
    /// Detailed asset information in all currencies
    pub details: Vec<TradingBalance>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalance {
    /// Cash Balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cash_bal: MaybeFloat,
    /// Equity of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq: MaybeFloat,
    /// Currency
    pub ccy: String,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub u_time: Option<u64>,
    /// Isolated margin equity of the currency
    /// Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: MaybeFloat,
    /// Available equity of the currency
    /// The balance that can be used on margin or futures/swap trading.
    /// Applicable to Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_eq: MaybeFloat,
    /// Discount equity of the currency in USD.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub dis_eq: MaybeFloat,
    /// Frozen balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub fixed_bal: MaybeFloat,
    /// Available balance of the currency
    /// The balance that can be withdrawn or transferred or used on spot trading.
    /// Applicable to Simple, Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_bal: MaybeFloat,
    /// Frozen balance of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub frozen_bal: MaybeFloat,
    /// Margin frozen for open orders
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ord_frozen: MaybeFloat,
    /// Liabilities of the currency
    /// It is a positive value, e.g."21625.64". Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liab: MaybeFloat,
    /// The sum of the unrealized profit & loss of all margin and derivatives positions of the currency.
    /// Applicable to Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl: MaybeFloat,
    /// Liabilities due to Unrealized loss of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl_liab: MaybeFloat,
    /// Cross liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cross_liab: MaybeFloat,
    /// Isolated liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_liab: MaybeFloat,
    /// Isolated liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: MaybeFloat,
    /// Accrued interest of the currency
    /// It is a positive value, e.g."9.01". Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub interest: MaybeFloat,
    /// Risk indicator of auto liability repayment
    /// Divided into multiple levels from 0 to 5, the larger the number, the more likely the auto repayment will be triggered.
    /// Applicable to Multi-currency margin and Portfolio margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub twap: MaybeFloat,
    /// Max loan of the currency
    /// Applicable to cross of Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub max_loan: MaybeFloat,
    /// Equity in USD of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq_usd: MaybeFloat,
    /// Potential borrowing IMR of the currency in USD
    /// Only applicable to Multi-currency margin and Portfolio margin. It is "" for other margin modes.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub borrow_froz: MaybeFloat,
    /// Leverage of the currency
    /// Applicable to Single-currency margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub notional_level: MaybeFloat,
    /// Strategy equity
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub stgy_eq: MaybeFloat,
    /// Isolated unrealized profit and loss of the currency
    /// Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_upl: MaybeFloat,
    /// Spot in use amount
    /// Applicable to Portfolio margin
    #[serde(default)]
    pub spot_in_use_amt: MaybeFloat,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetail {
    /// Instrument type
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
    #[serde(default)]
    pub pos: MaybeFloat,
    /// Base currency balance, only applicable to MARGIN（Manual transfers and Quick Margin Mode）
    #[serde(default)]
    pub base_bal: MaybeFloat,
    /// Quote currency balance, only applicable to MARGIN（Manual transfers and Quick Margin Mode）
    #[serde(default)]
    pub quote_bal: MaybeFloat,
    /// Base currency amount already borrowed, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default)]
    pub base_borrowed: MaybeFloat,
    /// Base Interest, undeducted interest that has been incurred, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default)]
    pub base_interest: MaybeFloat,
    /// Quote currency amount already borrowed, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default)]
    pub quote_borrowed: MaybeFloat,
    /// Quote Interest, undeducted interest that has been incurred, only applicable to MARGIN(Quick Margin Mode）
    #[serde(default)]
    pub quote_interest: MaybeFloat,
    /// Position currency, only applicable to MARGIN positions.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub pos_ccy: Option<String>,
    /// Position that can be closed
    /// Only applicable to MARGIN, FUTURES/SWAP in the long-short mode and OPTION.
    /// For Margin position, the rest of sz will be SPOT trading after the liability is repaid while closing the position. Please get the available reduce-only amount from "Get maximum available tradable amount" if you want to reduce the amount of SPOT trading as much as possible.
    #[serde(default)]
    pub avail_pos: MaybeFloat,
    /// Average open price
    #[serde(default)]
    pub avg_px: MaybeFloat,
    /// Latest Mark price
    #[serde(default)]
    pub mark_px: MaybeFloat,
    /// Unrealized profit and loss calculated by mark price.
    #[serde(default)]
    pub upl: MaybeFloat,
    /// Unrealized profit and loss ratio calculated by mark price.
    #[serde(default)]
    pub upl_ratio: MaybeFloat,
    /// Unrealized profit and loss calculated by last price. Main usage is showing, actual value is upl.
    #[serde(default)]
    pub upl_last_px: MaybeFloat,
    /// Unrealized profit and loss ratio calculated by last price.
    #[serde(default)]
    pub upl_ratio_last_px: MaybeFloat,
    /// Instrument ID, e.g. BTC-USD-180216
    pub inst_id: String,
    /// Leverage, not applicable to OPTION
    #[serde(default)]
    pub lever: MaybeFloat,
    /// Estimated liquidation price
    /// Not applicable to OPTION
    #[serde(default)]
    pub liq_px: MaybeFloat,
    /// Initial margin requirement, only applicable to cross.
    #[serde(default)]
    pub imr: MaybeFloat,
    /// Margin, can be added or reduced. Only applicable to isolated.
    #[serde(default)]
    pub margin: MaybeFloat,
    /// Margin ratio
    #[serde(default)]
    pub mgn_ratio: MaybeFloat,
    /// Maintenance margin requirement
    #[serde(default)]
    pub mmr: MaybeFloat,
    /// Liabilities, only applicable to MARGIN.
    #[serde(default)]
    pub liab: MaybeFloat,
    /// Liabilities currency, only applicable to MARGIN.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liab_ccy: Option<String>,
    /// Interest. Undeducted interest that has been incurred.
    #[serde(default)]
    pub interest: MaybeFloat,
    /// Last trade ID
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub trade_id: Option<String>,
    /// Option Value, only applicable to OPTION.
    #[serde(default)]
    pub opt_val: MaybeFloat,
    /// Notional value of positions in USD
    #[serde(default)]
    pub notional_usd: MaybeFloat,
    /// Auto-deleveraging (ADL) indicator
    /// Divided into 5 levels, from 1 to 5, the smaller the number, the weaker the adl intensity.
    pub adl: String,
    /// Currency used for margin
    pub ccy: String,
    /// Latest traded price
    #[serde(default)]
    pub last: MaybeFloat,
    /// Latest underlying index price
    #[serde(default)]
    pub idx_px: MaybeFloat,
    /// USD price
    #[serde(default)]
    pub usd_px: MaybeFloat,
    /// Breakeven price
    #[serde(rename = "be_px", default)]
    pub breakeven_price: MaybeFloat,
    #[serde(default)]
    pub delta_bs: MaybeFloat,
    #[serde(default)]
    pub delta_pa: MaybeFloat,
    #[serde(default)]
    pub gamma_bs: MaybeFloat,
    #[serde(default)]
    pub gamma_pa: MaybeFloat,
    #[serde(default)]
    pub theta_bs: MaybeFloat,
    #[serde(default)]
    pub theta_pa: MaybeFloat,
    #[serde(default)]
    pub vega_bs: MaybeFloat,
    #[serde(default)]
    pub vega_pa: MaybeFloat,
    /// Spot in use amount
    /// Applicable to Portfolio margin
    #[serde(default)]
    pub spot_in_use_amt: MaybeFloat,
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
    #[serde(default)]
    pub realized_pnl: MaybeFloat,
    /// Accumulated pnl of closing order(s)
    #[serde(default)]
    pub pnl: MaybeFloat,
    /// Accumulated fee
    /// Negative number represents the user transaction fee charged by the platform.Positive number represents rebate.
    #[serde(default)]
    pub fee: MaybeFloat,
    /// Accumulated funding fee
    #[serde(default)]
    pub funding_fee: MaybeFloat,
    /// Latest time position was adjusted, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, with = "str_opt")]
    pub u_time: Option<u64>,
    /// Creation time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, with = "str_opt")]
    pub c_time: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAndPositionDetail {
    /// Push time of both balance and position information, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(default, with = "str_opt")]
    pub p_time: Option<u64>,
    /// Event Type
    /// snapshot,delivered,exercised,transferred,filled,liquidation,claw_back,adl,funding_fee,adjust_margin,set_leverage,interest_deduction
    pub event_type: BalanceAndPositionEventType,
    // /// Balance data
    pub bal_data: Vec<BalanceData>,
    // /// Position data
    pub pos_data: Vec<PosData>,
    // /// Details of trade
    pub trades: Vec<TradeData>,
}

#[derive(Debug, Clone, Hash)]
pub enum BalanceAndPositionEventType {
    Snapshot,
    Delivered,
    Exercised,
    Transferred,
    Filled,
    Liquidation,
    ClawBack,
    Adl,
    FundingFee,
    AdjustMargin,
    SetLeverage,
    InterestDeduction,
    Other(Unknown),
}
impl_string_enum!(BalanceAndPositionEventType,
    Other,
    Snapshot => "snapshot",
    Delivered => "delivered",
    Exercised => "exercised",
    Transferred => "transferred",
    Filled => "filled",
    Liquidation => "liquidation",
    ClawBack => "claw_back",
    Adl => "adl",
    FundingFee => "funding_fee",
    AdjustMargin => "adjust_margin",
    SetLeverage => "set_leverage",
    InterestDeduction => "interest_deduction",
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecType {
    Taker,
    Maker,
}
impl_string_enum!(ExecType,
    Taker => "T",
    Maker => "M",
);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    #[serde(default, with = "str_opt")]
    pub ccy: MaybeString,
    #[serde(default, with = "str_opt")]
    pub cash_bal: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub u_time: MaybeU64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PosData {
    /// Position ID
    #[serde(default, with = "str_opt")]
    pub pos_id: MaybeString,
    /// Last trade ID
    #[serde(default, with = "str_opt")]
    pub trade_id: MaybeString,
    /// Instrument ID, e.g BTC-USD-180213
    #[serde(default, with = "str_opt")]
    pub inst_id: MaybeString,
    /// Instrument type
    #[serde(default, with = "str_opt")]
    pub inst_type: Option<InstrumentType>,
    /// Margin mode
    /// isolated, cross
    #[serde(default, with = "str_opt")]
    pub mgn_mode: Option<MarginMode>,
    /// Position side
    /// long, short, net
    #[serde(default, with = "str_opt")]
    pub pos_side: Option<PositionSide>,
    /// Quantity of positions. In the mode of autonomous transfer from position to position, after the deposit is transferred, a position with pos of 0 will be generated
    #[serde(default, with = "str_opt")]
    pub pos: MaybeFloat,
    /// Base currency balance, only applicable to MARGIN（Manual transfers and Quick Margin Mode
    #[serde(default, with = "str_opt")]
    pub base_bal: MaybeFloat,
    /// Quote currency balance, only applicable to MARGIN（Manual transfers and Quick Margin Mode
    #[serde(default, with = "str_opt")]
    pub quote_bal: MaybeFloat,
    /// Currency
    #[serde(default, with = "str_opt")]
    pub ccy: MaybeString,
    /// Position currency, only applicable to MARGIN positions.
    #[serde(default, with = "str_opt")]
    pub pos_ccy: MaybeString,
    /// Average open price
    #[serde(default, with = "str_opt")]
    pub avg_px: MaybeFloat,
    /// Update time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, with = "str_opt")]
    pub u_time: MaybeU64,
}

#[derive(Debug, Deserialize, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TradeData {
    /// Instrument ID, e.g. BTC-USDT
    pub inst_id: String,
    /// Trade ID
    pub trade_id: String,
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
    #[serde(default)]
    pub interest: MaybeFloat,
    #[serde(default)]
    pub interest_rate: MaybeFloat,
    #[serde(default)]
    pub liab: MaybeFloat,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ts: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestLimitResponse {
    #[serde(default)]
    pub debt: MaybeFloat,
    #[serde(default)]
    pub interest: MaybeFloat,
    pub records: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestLimit {
    #[serde(default)]
    pub avail_loan: MaybeFloat,
    pub ccy: String,
    #[serde(default)]
    pub interest: MaybeFloat,
    #[serde(default)]
    pub loan_quota: MaybeFloat,
    #[serde(default)]
    pub pos_loan: MaybeFloat,
    #[serde(default)]
    pub rate: MaybeFloat,
    #[serde(default)]
    pub surplus_lmt: MaybeFloat,
    #[serde(default)]
    pub used_lmt: MaybeFloat,
    #[serde(default)]
    pub used_loan: MaybeFloat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    // Instrument type
    pub inst_type: InstrumentType,
    // instrument ID, e.g. BTC-USD-200626
    pub inst_id: String,
    // Open interest (cont)
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub oi: MaybeFloat,
    // Open interest (coin)
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub oi_ccy: MaybeFloat,
    // Data return time,  Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, rename = "ts", deserialize_with = "deserialize_from_opt_str")]
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    // Instrument ID, e.g. BTC-USD-SWAP
    pub inst_id: String,
    // Instrument type
    pub inst_type: InstrumentType,
    // Current funding rate
    #[serde(default, with = "str_opt")]
    pub funding_rate: Option<f64>,
    // Settlement time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, with = "str_opt")]
    pub funding_time: Option<u64>,
    // Forecasted funding rate for the next period
    #[serde(default, with = "str_opt")]
    pub next_funding_rate: Option<f64>,
    // Forecasted funding time for the next period , Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, with = "str_opt")]
    pub next_funding_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    // Instrument type. SWAP
    pub inst_type: InstrumentType,
    // Instrument ID, e.g. BTC-USD-SWAP
    pub inst_id: String,
    // Predicted funding rate
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub funding_rate: MaybeFloat,
    // Actual funding rate
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub realized_rate: MaybeFloat,
    // Settlement time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub funding_time: Option<u64>,
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
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub buy_lmt: MaybeFloat,
    // Lowest sell limit
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub sell_lmt: MaybeFloat,
    // Data return time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ts: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountRateAndInterestFreeQuota {
    // Currency
    pub ccy: String,
    // Interest-free quota
    #[serde(default)]
    pub amt: MaybeFloat,
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
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountInfo {
    // Discount rate
    #[serde(default)]
    pub discount_rate: MaybeFloat,
    // Tier - upper bound, "" means positive infinity
    #[serde(default)]
    pub max_amt: MaybeFloat,
    // Tier - lower bound, the minimum is 0
    #[serde(default)]
    pub min_amt: MaybeFloat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    /// Instrument type
    /// MARGIN
    /// SWAP
    /// FUTURES
    /// OPTION
    pub inst_type: InstrumentType,
    /// Instrument ID, e.g. BTC-USD-200214
    pub inst_id: String,
    /// Mark price
    #[serde(rename = "markPx")]
    #[serde(default)]
    pub mark_price: MaybeFloat,
    /// Data return time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "ts", default)]
    pub timestamp: MaybeU64,
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
    #[serde(default)]
    pub min_sz: MaybeFloat,
    /// The maximum borrowing amount or number of positions held in this position is only applicable to margin/options/perpetual/delivery
    /// It will return the maximum borrowing amount when ccy takes effect.
    #[serde(default)]
    pub max_sz: MaybeFloat,
    /// Maintenance margin requirement rate
    #[serde(default)]
    pub mmr: MaybeFloat,
    /// Initial margin requirement rate
    #[serde(default)]
    pub imr: MaybeFloat,
    /// Maximum available leverage
    #[serde(default)]
    pub max_lever: MaybeFloat,
    /// Option Margin Coefficient (only applicable to options)
    #[serde(default)]
    pub opt_mgn_factor: MaybeFloat,
    /// Quote currency borrowing amount (only applicable to leverage and the case when instId takes effect)
    #[serde(default)]
    pub quote_max_loan: MaybeFloat,
    /// Base currency borrowing amount (only applicable to leverage and the case when instId takes effect)
    #[serde(default)]
    pub base_max_loan: MaybeFloat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceFund {
    /// The total balance of insurance fund, in USD
    #[serde(default)]
    pub total: MaybeFloat,
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
    #[serde(default)]
    pub amt: MaybeFloat,
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
    #[serde(default)]
    pub idx_px: MaybeFloat,
    /// Highest price in the past 24 hours
    #[serde(default)]
    pub high_24h: MaybeFloat,
    /// Lowest price in the past 24 hours
    #[serde(default)]
    pub low_24h: MaybeFloat,
    /// Open price in the past 24 hours
    #[serde(default)]
    pub open_24h: MaybeFloat,
    /// Open price in the UTC 0
    #[serde(default)]
    pub sod_utc0: MaybeFloat,
    /// Open price in the UTC 8
    #[serde(default)]
    pub sod_utc8: MaybeFloat,
    /// Index price update time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    /// Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: u64,
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// highest price
    #[serde(rename = "h")]
    pub high: f64,
    /// Lowest price
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// The state of candlesticks.
    /// 0 represents that it is uncompleted, 1 represents that it is completed.
    pub confirm: CandleState,
}

#[derive(Debug, Clone, Copy)]
pub struct Level<'a> {
    pub price: &'a str,
    pub size: &'a str,
    pub orders: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Levels<'a> {
    #[serde(borrow)]
    Depth1([Level<'a>; 1]),
    #[serde(borrow)]
    Depth5([Level<'a>; 5]),
    #[serde(borrow)]
    Depths(Vec<Level<'a>>),
}

#[allow(clippy::len_without_is_empty)]
impl<'a> Levels<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Level> + '_ {
        match self {
            Levels::Depth1(s) => s.iter(),
            Levels::Depth5(s) => s.iter(),
            Levels::Depths(s) => s.iter(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Levels::Depth1(_) => 1,
            Levels::Depth5(_) => 5,
            Levels::Depths(s) => s.len(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookUpdate<'a> {
    // Checksum
    #[serde(default, with = "str_opt")]
    pub checksum: MaybeI64,
    /// Sequence ID of the current message
    #[serde(default, with = "str_opt")]
    pub seq_id: MaybeI64,
    /// Sequence ID of the last sent message. Only applicable to books, books-l2-tbt, books50-l2-tbt
    #[serde(default, with = "str_opt")]
    pub prev_seq_id: MaybeI64,
    /// Order book on sell side
    #[serde(borrow)]
    pub asks: Levels<'a>,
    /// Order book on bid side
    #[serde(borrow)]
    pub bids: Levels<'a>,
    #[serde(default, with = "str_opt")]
    pub ts: MaybeU64,
}

#[cfg(test)]
mod test {
    use crate::api::v5::{BookUpdate, Levels};

    #[test]
    fn size_of_levels() {
        use std::mem::size_of;
        assert_eq!(size_of::<BookUpdate>(), 560);
        assert_eq!(size_of::<Levels>(), 248);
    }
}

/// Custom deserializer for book level
/// expecting level format: [price, size, "0", orders]
struct LevelVisitor;
impl<'de> Visitor<'de> for LevelVisitor {
    type Value = Level<'de>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("level format: [price, size, \"0\", orders]")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        match (
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
        ) {
            (Some(price), Some(size), Some("0"), Some(orders)) => Ok(Level {
                price,
                size,
                orders,
            }),
            _ => Err(A::Error::custom("invalid level format")),
        }
    }
}

impl<'de> Deserialize<'de> for Level<'de> {
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
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
            seq.next_element::<&str>()?,
        ) {
            (Some(ts), Some(open), Some(high), Some(low), Some(close), Some(confirm)) => {
                let ts =
                    u64::from_str(ts).map_err(|_| S::Error::custom("unknown timestamp format"))?;
                let open =
                    f64::from_str(open).map_err(|_| S::Error::custom("unknown open format"))?;
                let high =
                    f64::from_str(high).map_err(|_| S::Error::custom("unknown high format"))?;
                let low = f64::from_str(low).map_err(|_| S::Error::custom("unknown low format"))?;
                let close =
                    f64::from_str(close).map_err(|_| S::Error::custom("unknown close format"))?;
                let confirm = CandleState::from_str(confirm)
                    .map_err(|_| S::Error::custom(format!("unknown candle state: {}", confirm)))?;
                Ok(Candle {
                    ts,
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
        assert_eq!(candle.ts, 1597026383085);
        assert_eq!(candle.open, 3.721);
        assert_eq!(candle.high, 3.743);
        assert_eq!(candle.low, 3.677);
        assert_eq!(candle.close, 3.708);
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
    #[serde(default)]
    pub min_dep: MaybeFloat,
    /// The minimum withdrawal amount of the currency in a single transaction
    #[serde(default)]
    pub min_wd: MaybeFloat,
    /// The maximum amount of currency withdrawal in a single transaction
    #[serde(default)]
    pub max_wd: MaybeFloat,
    /// The withdrawal precision, indicating the number of digits after the decimal point.
    #[serde(default)]
    pub wd_tick_sz: MaybeFloat,
    /// The withdrawal limit in the past 24 hours (including `on-chain withdrawal` and `internal transfer`), unit in `USD`
    #[serde(default)]
    pub wd_quota: MaybeFloat,
    /// The amount of currency withdrawal used in the past 24 hours, unit in `USD`
    #[serde(default)]
    pub used_wd_quota: MaybeFloat,
    /// The minimum withdrawal fee for normal address
    #[serde(default)]
    pub min_fee: MaybeFloat,
    /// The maximum withdrawal fee for normal address
    #[serde(default)]
    pub max_fee: MaybeFloat,
    /// The minimum withdrawal fee for contract address
    #[serde(default)]
    pub min_fee_for_ct_addr: MaybeFloat,
    /// The maximum withdrawal fee for contract address
    #[serde(default)]
    pub max_fee_for_ct_addr: MaybeFloat,
    /// If current chain is main net, then it will return `true`, otherwise it will return `false`
    pub main_net: bool,
    /// Whether tag/memo information is required for withdrawal, e.g. `EOS` will return `true`
    pub need_tag: bool,
    /// The minimum number of blockchain confirmations to acknowledge fund deposit. The account is credited after that, but the deposit can not be withdrawn
    #[serde(default)]
    pub min_dep_arrival_confirm: MaybeFloat,
    /// The minimum number of blockchain confirmations required for withdrawal of a deposit
    #[serde(default)]
    pub min_wd_unlock_confirm: MaybeFloat,
    /// The fixed deposit limit, unit in `USD`
    #[serde(default)]
    pub dep_quota_fixed: MaybeFloat,
    /// The used amount of fixed deposit quota, unit in `USD`
    #[serde(default)]
    pub used_dep_quota_fixed: MaybeFloat,
    /// The layer2 network daily deposit limit
    #[serde(default)]
    pub dep_quote_daily_layer2: MaybeFloat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingBalance {
    /// Available balance
    /// The balance that can be withdrawn or transferred or used for spot trading
    #[serde(default)]
    pub avail_bal: MaybeFloat,
    /// Balance
    #[serde(default)]
    pub bal: MaybeFloat,
    /// Frozen balance
    #[serde(default)]
    pub frozen_bal: MaybeFloat,
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
    #[serde(default)]
    pub amt: MaybeFloat,
    /// Transfer type
    pub r#type: TransferType,
    pub from: AccountType,
    pub to: AccountType,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub sub_acct: Option<String>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub state: Option<FundTransferState>,
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
    #[serde(default, with = "str_opt")]
    pub ccy: Option<String>,
    /// Deposit ID
    #[serde(default, with = "str_opt")]
    pub dep_id: Option<String>,
    /// Chain name
    #[serde(default, with = "str_opt")]
    pub chain: Option<String>,
    /// Deposit amount
    #[serde(default)]
    pub amt: MaybeFloat,
    /// Deposite account
    // If the deposit comes from an internal transfer, this field displays the account information of the internal transfer initiator, which can be mobile phone number, email address, account name, and will return "" in other cases
    #[serde(default, with = "str_opt")]
    pub from: Option<String>,
    /// Deposit address
    /// If the deposit comes from the on-chain, this field displays the on-chain address, and will return "" in other cases
    #[serde(default, with = "str_opt")]
    pub to: Option<String>,
    /// Hash record of the deposit
    #[serde(default, with = "str_opt")]
    pub tx_id: Option<String>,
    /// Time that the deposit record is created, Unix timestamp format in milliseconds, e.g. 1655251200000
    #[serde(default, with = "str_opt")]
    pub ts: Option<u64>,
    #[serde(default, with = "str_opt")]
    pub state: Option<DepositStatus>,
    /// Actual amount of blockchain confirm in a single deposit
    #[serde(default, with = "str_opt")]
    pub actual_dep_blk_confirm: Option<String>,
    /// internal transfer initiator's withdrawal ID
    /// If the deposit comes from internal transfer, this field displays the withdrawal ID of the internal transfer initiator
    #[serde(default, with = "str_opt")]
    pub from_wd_id: Option<String>,
}
