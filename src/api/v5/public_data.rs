use std::{fmt::Display, str::FromStr};

use crate::api::v5::{FundingRate, MarkPrice, TradeMode};
use crate::{api::v5::Request, serde_util::*};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::v5::model::{
    Candle, DeliveryExerciseHistory, DiscountRateAndInterestFreeQuota, FundingRateHistory,
    IndexTicker, Instrument, InstrumentType, InsuranceFund, OKXSystemTime, OpenInterest,
    PositionTier, PriceLimit,
};

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-instruments
/// ## Get instruments
/// Retrieve a list of instruments with open contracts.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP + instrumentType
///
/// ## HTTP Request
/// GET /api/v5/public/instruments
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstruments {
    /// Instrument type
    /// SPOT
    /// MARGIN
    /// SWAP
    /// FUTURES
    /// OPTION
    pub inst_type: InstrumentType,
    /// Underlying
    /// Only applicable to FUTURES/SWAP/OPTION.If instType is OPTION, either uly or instFamily is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Only applicable to FUTURES/SWAP/OPTION. If instType is OPTION, either uly or instFamily is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

impl Request for GetInstruments {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/instruments";
    const AUTH: bool = false;

    type Response = Vec<Instrument>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-delivery-exercise-history
///
/// ## Get delivery/exercise history
/// Retrieve delivery records of Futures and exercise records of Options in the last 3 months.
///
/// Rate Limit: 40 requests per 2 seconds
/// Rate limit rule: IP + (instrumentType + uly)
///
/// ## HTTP Request
/// GET /api/v5/public/delivery-exercise-history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryExerciseHistory {
    pub inst_type: InstrumentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(
        serialize_with = "serialize_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub after: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetDeliveryExerciseHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/delivery-exercise-history";
    const AUTH: bool = false;

    type Response = Vec<DeliveryExerciseHistory>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-open-interest
/// ## Get open interest
/// Retrieve the total open interest for contracts on OKX.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP +instrumentID
/// ## HTTP Request
/// GET /api/v5/public/open-interest
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterest {
    pub inst_type: InstrumentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

impl Request for GetOpenInterest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/open-interest";
    const AUTH: bool = false;

    type Response = Vec<OpenInterest>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-funding-rate
/// ## Get funding rate
/// Retrieve funding rate.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP +instrumentID
/// ## HTTP Request
/// GET /api/v5/public/funding-rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRate {
    /// Instrument ID, e.g. BTC-USD-SWAP
    /// only applicable to SWAP
    pub inst_id: String,
}

impl Request for GetFundingRate {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/funding-rate";
    const AUTH: bool = false;

    type Response = Vec<FundingRate>;
}

/// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-funding-rate-history
/// ## Get funding rate history
/// Retrieve funding rate history. This endpoint can retrieve data from the last 3 months.
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP +instrumentID
/// ## HTTP Request
/// GET /api/v5/public/funding-rate-history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistory {
    /// Instrument ID, e.g. BTC-USD-SWAP
    /// only applicable to SWAP
    pub inst_id: String,
    /// Pagination of data to return records newer than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<Utc>>,
    /// Pagination of data to return records earlier than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetFundingRateHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/funding-rate-history";
    const AUTH: bool = false;

    type Response = Vec<FundingRateHistory>;
}

/// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-limit-price
/// ## Get limit price
/// Retrieve the highest buy limit and lowest sell limit of the instrument.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/public/price-limit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLimitPrice {
    /// Instrument ID, e.g. BTC-USD-SWAP
    /// only applicable to FUTURES/SWAP/OPTION
    pub inst_id: String,
}

impl Request for GetLimitPrice {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/price-limit";
    const AUTH: bool = false;

    type Response = Vec<PriceLimit>;
}

/// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-discount-rate-and-interest-free-quota
/// ## Get discount rate and interest-free quota
/// Retrieve discount rate level and interest-free quota.
///
/// Rate Limit: 2 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/public/discount-rate-interest-free-quota
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountRateAndInterestFreeQuota {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Discount level
    /// 1:level 1
    /// 2:level 2
    /// 3:level 3
    /// 4:level 4
    /// 5:level 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_lv: Option<usize>,
}

impl Request for GetDiscountRateAndInterestFreeQuota {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/discount-rate-interest-free-quota";
    const AUTH: bool = false;

    type Response = Vec<DiscountRateAndInterestFreeQuota>;
}

/// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-system-time
/// ## Get system time
/// Retrieve API server time.
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/public/time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemTime;
impl Request for GetSystemTime {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/time";
    const AUTH: bool = false;

    type Response = Vec<OKXSystemTime>;
}

/// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-mark-price
/// ## Get mark price
/// Retrieve mark price.
///
/// We set the mark price based on the SPOT index and at a reasonable basis to prevent individual users from manipulating the market and causing the contract price to fluctuate.
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP +instrumentID
/// ## HTTP Request
/// GET /api/v5/public/mark-price
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPrice {
    /// Instrument type
    /// MARGIN
    /// SWAP
    /// FUTURES
    /// OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Underlying
    /// Applicable to FUTURES/SWAP/OPTION
    pub uly: Option<String>,
    /// Instrument family
    /// Applicable to FUTURES/SWAP/OPTION
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. BTC-USD-SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

impl Request for GetMarkPrice {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/mark-price";
    const AUTH: bool = false;

    type Response = Vec<MarkPrice>;
}

/// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-position-tiers
/// ## Get position tiers
/// Retrieve position tiers information, maximum leverage depends on your borrowings and margin ratio.
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/public/position-tiers
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiers {
    /// Instrument type
    /// MARGIN
    /// SWAP
    /// FUTURES
    /// OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Trade mode
    /// Margin mode cross
    /// isolated
    pub td_mode: TradeMode,
    /// Single underlying or multiple underlyings (no more than 3) separated with comma.
    /// If instType is SWAP/FUTURES/OPTION, either uly or instFamily is required.
    /// If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Single instrument familiy or multiple instrument families (no more than 5) separated with comma.
    /// If instType is SWAP/FUTURES/OPTION, either uly or instFamily is required.
    /// If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Single instrument or multiple instruments (no more than 5) separated with comma.
    /// Either instId or ccy is required, if both are passed, instId will be used, ignore when instType is one of SWAP,FUTURES,OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Margin currency
    /// Only applicable to cross MARGIN. It will return borrowing amount for Multi-currency margin and Portfolio margin when ccy takes effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Tiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl Request for GetPositionTiers {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/position-tiers";
    const AUTH: bool = false;

    type Response = Vec<PositionTier>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-underlying
/// ## Get underlying
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/public/underlying
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnderlying {
    /// Instrument type
    /// SWAP
    /// FUTURES
    /// OPTION
    pub inst_type: InstrumentType,
}

impl Request for GetUnderlying {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/underlying";
    const AUTH: bool = false;

    type Response = Vec<String>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-insurance-fund
/// ## Get insurance fund
/// Get insurance fund balance information
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/public/insurance-fund
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInsuranceFund {
    /// Instrument type
    /// MARGIN
    /// SWAP
    /// FUTURES
    /// OPTION
    pub inst_type: InstrumentType,
    /// Type
    /// liquidation_balance_deposit
    /// bankruptcy_loss
    /// platform_revenue
    /// The default is all type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Underlying
    /// Required for FUTURES/SWAP/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Required for FUTURES/SWAP/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Currency, only applicable to MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Pagination of data to return records newer than the requested ts
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub before: Option<DateTime<Utc>>,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub after: Option<DateTime<Utc>>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetInsuranceFund {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/public/insurance-fund";
    const AUTH: bool = false;

    type Response = Vec<InsuranceFund>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-index-tickers
/// ## Get index tickers
/// Retrieve index tickers.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/market/index-tickers
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexTickers {
    /// Quote currency
    /// Currently there is only an index with USD/USDT/BTC/USDC as the quote currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<String>,
    /// Index, e.g. BTC-USD
    /// Either quoteCcy or instId is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

impl Request for GetIndexTickers {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/index-tickers";
    const AUTH: bool = false;

    type Response = Vec<IndexTicker>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-index-candlesticks
/// ## Get index candlesticks
/// Retrieve the candlestick charts of the index. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/market/index-candles
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexCandles {
    /// Index, e.g. BTC-USD
    pub inst_id: String,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub after: Option<DateTime<Utc>>,
    /// Pagination of data to return records newer than the requested ts. The latest data will be returned when using before individually
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub before: Option<DateTime<Utc>>,
    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line：[6H/12H/1D/1W/1M/3M]
    /// UTC time opening price k-line：[6Hutc/12Hutc/1Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Number of results per request. The maximum is 100. The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetIndexCandles {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/index-candles";
    const AUTH: bool = false;

    type Response = Vec<Candle>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-index-candlesticks-history
/// ## Get index candlesticks history
/// Retrieve the candlestick charts of the index from recent years.
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/market/history-index-candles
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoryIndexCandles {
    /// Index, e.g. BTC-USD
    pub inst_id: String,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub after: Option<DateTime<Utc>>,
    /// Pagination of data to return records newer than the requested ts. The latest data will be returned when using before individually
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub before: Option<DateTime<Utc>>,
    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line：[6H/12H/1D/1W/1M]
    /// UTC time opening price k-line：[/6Hutc/12Hutc/1Dutc/1Wutc/1Mutc]
    pub bar: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    pub limit: Option<usize>,
}

impl Request for GetHistoryIndexCandles {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/history-index-candles";
    const AUTH: bool = false;

    type Response = Vec<Candle>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-mark-price-candlesticks
/// ## Get mark price candlesticks
/// Retrieve the candlestick charts of mark price. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/market/mark-price-candles
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceCandles {
    /// Instrument ID, e.g. BTC-USD-SWAP
    pub inst_id: String,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub after: Option<DateTime<Utc>>,
    /// Pagination of data to return records newer than the requested ts. The latest data will be returned when using before individually
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub before: Option<DateTime<Utc>>,
    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line：[6H/12H/1D/1W/1M/3M]
    /// UTC time opening price k-line：[6Hutc/12Hutc/1Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetMarkPriceCandles {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/mark-price-candles";
    const AUTH: bool = false;

    type Response = Vec<Candle>;
}

/// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-mark-price-candlesticks-history
/// ## Get mark price candlesticks history
/// Retrieve the candlestick charts of mark price from recent years.
///
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/market/history-mark-price-candles
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoryMarkPriceCandles {
    /// Instrument ID, e.g. BTC-USD-SWAP
    pub inst_id: String,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub after: Option<DateTime<Utc>>,
    /// Pagination of data to return records newer than the requested ts. The latest data will be returned when using before individually
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_timestamp"
    )]
    pub before: Option<DateTime<Utc>>,
    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line：[6H/12H/1D/1W/1M/3M]
    /// UTC time opening price k-line：[6Hutc/12Hutc/1Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetHistoryMarkPriceCandles {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/history-mark-price-candles";
    const AUTH: bool = false;

    type Response = Vec<Candle>;
}

/// ## Get index components
/// Get the index component information data on the market
///
/// Rate Limit: 20 requests per 2 seconds
/// Rate limit rule: IP
/// ## HTTP Request
/// GET /api/v5/market/index-components
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexComponents {
    /// Index, e.g. BTC-USD
    pub index: String,
}

impl Request for GetIndexComponents {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/index-components";
    const AUTH: bool = false;

    type Response = Vec<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponent {
    /// Index
    pub index: String,
    /// Latest Index Price
    pub last: Decimal,
    /// Data generation time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    /// Components
    pub components: Vec<IndexComponentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponentItem {
    /// Name of Exchange
    pub exch: String,
    /// Name of Exchange Trading Pairs
    pub symbol: String,
    /// Price of Exchange Trading Pairs
    pub sym_px: Decimal,
    /// Weights
    pub wgt: Decimal,
    /// Price converted to index
    pub cnv_px: Decimal,
}

// Websockets
pub mod websocket {
    use super::*;
    use crate::websocket::WebsocketChannel;
    use std::time::Duration;

    pub struct Instruments(pub InstrumentType);
    impl WebsocketChannel for Instruments {
        fn subscribe_message(&self) -> String {
            serde_json::json!({
                "op": "subscribe",
                "args": [
                    {
                      "channel": "instruments",
                      "instType": self.0,
                    }
                ]
            })
            .to_string()
        }

        fn unsubscribe_message(&self) -> String {
            todo!()
        }
    }

    /// MarkPrices(InstId)
    pub struct MarkPrices(pub String);
    impl WebsocketChannel for MarkPrices {
        fn subscribe_message(&self) -> String {
            serde_json::json!({
                "op": "subscribe",
                "args": [
                    {
                      "channel": "mark-price",
                      "instId": self.0,
                    }
                ]
            })
            .to_string()
        }

        fn unsubscribe_message(&self) -> String {
            todo!()
        }
    }

    /// IndexCandles(InstId)
    pub struct IndexTickers(pub String);
    impl WebsocketChannel for IndexTickers {
        fn subscribe_message(&self) -> String {
            serde_json::json!({
                "op": "subscribe",
                "args": [
                    {
                      "channel": "index-tickers",
                      "instId": self.0,
                    }
                ]
            })
            .to_string()
        }

        fn unsubscribe_message(&self) -> String {
            todo!()
        }
    }
}
