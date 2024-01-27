use crate::api::v5::model::{
    Category, InstrumentType, OrderState, OrderType, PositionSide, QuantityType, Side,
    StopLossTriggerPriceType, TakeProfitTriggerPriceType, TradeMode,
};
use crate::api::v5::{Request, SelfTradePreventionMode};
use crate::serde_util::{deserialize_from_opt_str, deserialize_timestamp};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// https://www.okx.com/docs-v5/en/#rest-api-trade-cancel-order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrder {
    pub inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderData {
    pub cl_ord_id: String,
    pub ord_id: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub s_code: u32,
    pub s_msg: String,
}

impl Request for CancelOrder {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/trade/cancel-order";
    const AUTH: bool = true;

    type Response = Vec<CancelOrderData>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-trade-cancel-multiple-orders
/// For now assume we only use cancel multiple orders API.
pub type CancelMultipleOrders = Vec<CancelOrder>;

impl Request for CancelMultipleOrders {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/trade/cancel-batch-orders";
    const AUTH: bool = true;

    type Response = Vec<CancelOrderData>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-trade-place-order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder {
    /// Instrument ID, e.g. BTC-USD-190927-5000-C
    pub inst_id: String,
    /// Trade mode
    /// Margin mode cross isolated
    /// Non-Margin mode cash
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub td_mode: TradeMode,
    /// Margin currency
    /// Only applicable to cross MARGIN orders in Single-currency margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Client Order ID as assigned by the client
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Order side, buy sell
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub side: Side,
    /// Position side
    /// The default is net in the net mode
    /// It is required in the long/short mode, and can only be long or short.
    /// Only applicable to FUTURES/SWAP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,
    /// Order type
    /// market: Market order
    /// limit: Limit order
    /// post_only: Post-only order
    /// fok: Fill-or-kill order
    /// ioc: Immediate-or-cancel order
    /// optimal_limit_ioc: Market order with immediate-or-cancel order (applicable only to Futures and Perpetual swap).
    /// mmp：Market Maker Protection (only applicable to Option in Portfolio Margin mode)
    /// mmp_and_post_only：Market Maker Protection and Post-only order(only applicable to Option in Portfolio Margin mode)V
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub ord_type: OrderType,
    /// Quantity to buy or sell
    pub sz: Decimal,
    /// Order price. Only applicable to limit,post_only,fok,ioc,mmp,mmp_and_post_only order.
    /// When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<Decimal>,
    /// Whether orders can only reduce in position size.
    /// Valid options: true or false. The default value is false.
    /// Only applicable to MARGIN orders, and FUTURES/SWAP orders in net mode
    /// Only applicable to Single-currency margin and Multi-currency margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Whether the target currency uses the quote or base currency.
    /// base_ccy: Base currency ,quote_ccy: Quote currency
    /// Only applicable to SPOT Market Orders
    /// Default is quote_ccy for buy, base_ccy for sell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<QuantityType>,
    /// Whether to disallow the system from amending the size of the SPOT Market Order.
    /// Valid options: true or false. The default value is false.
    /// If true, system will not amend and reject the market order if user does not have sufficient funds.
    /// Only applicable to SPOT Market Orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_amend: Option<bool>,
    /// Client-supplied Algo ID when placing order attaching TP/SL
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    /// It will be posted to algoClOrdId when placing TP/SL order once the general order is filled completely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_algo_cl_ord_id: Option<String>,
    /// Take-profit trigger price
    /// If you fill in this parameter, you should fill in the take-profit order price as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<Decimal>,
    /// Take-profit order price
    /// If you fill in this parameter, you should fill in the take-profit trigger price as well.
    /// If the price is -1, take-profit will be executed at the market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<Decimal>,
    /// Stop-loss trigger price
    /// If you fill in this parameter, you should fill in the stop-loss order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<Decimal>,
    /// Stop-loss order price
    /// If you fill in this parameter, you should fill in the stop-loss trigger price.
    /// If the price is -1, stop-loss will be executed at the market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<Decimal>,
    /// Take-profit trigger price type
    /// last: last price
    /// index: index price
    /// mark: mark price
    /// The Default is last
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<TakeProfitTriggerPriceType>,
    /// Stop-loss trigger price type
    /// last: last price
    /// index: index price
    /// mark: mark price
    /// The Default is last
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<StopLossTriggerPriceType>,
    /// Quick Margin type. Only applicable to Quick Margin Mode of isolated margin
    /// manual, auto_borrow, auto_repay
    /// The default value is manual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<String>,
    /// Self trade prevention ID. Orders from the same master account with the same ID will be prevented from self trade.
    /// Numerical integers defined by user in the range of 1<= x<= 999999999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<String>,
    /// Self trade prevention mode. It is available only when stpId is filled.
    /// Default to cancel maker
    /// cancel_maker,cancel_taker, cancel_both
    /// Cancel both does not support FOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<SelfTradePreventionMode>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Order ID
    pub ord_id: String,
    /// Client Order ID as assigned by the client
    pub cl_ord_id: String,
    /// Order tag
    pub tag: String,
    /// The code of the event execution result, 0 means success.
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub s_code: u32,
    /// Rejection or success message of event execution.
    pub s_msg: String,
}

impl Request for PlaceOrder {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/trade/order";
    const AUTH: bool = true;

    type Response = Vec<PlaceOrderResponse>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-trade-get-order-details
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDetails {
    pub inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub inst_type: InstrumentType,
    pub inst_id: String,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tgt_ccy: Option<QuantityType>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub ccy: Option<String>,
    pub ord_id: String,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub cl_ord_id: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tag: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub px: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sz: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pnl: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub ord_type: OrderType,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub side: Side,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pos_side: Option<PositionSide>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub td_mode: Option<TradeMode>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub acc_fill_sz: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fill_px: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub trade_id: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fill_sz: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fill_time: Option<u64>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avg_px: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: OrderState,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub lever: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tp_trigger_px: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tp_trigger_px_type: Option<TakeProfitTriggerPriceType>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tp_ord_px: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sl_trigger_px: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sl_trigger_px_type: Option<StopLossTriggerPriceType>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sl_ord_px: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fee_ccy: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fee: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub rebate_ccy: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub source: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub rebate: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub category: Option<Category>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub c_time: DateTime<Utc>,
}

impl Request for GetOrderDetails {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/trade/order";
    const AUTH: bool = true;

    type Response = Vec<OrderDetail>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-trade-get-order-list
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetOrderList {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/trade/orders-pending";
    const AUTH: bool = true;

    type Response = Vec<OrderDetail>;
}

pub mod websocket {
    use super::*;
    use crate::websocket::WebsocketChannel;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OrdersChannelArg<'a> {
        pub channel: Option<&'a str>,
        pub inst_type: InstrumentType,
    }

    pub struct OrdersChannel(pub InstrumentType);
    impl WebsocketChannel for OrdersChannel {
        const CHANNEL: &'static str = "orders";
        const AUTH: bool = true;
        type Response<'de> = Vec<OrderDetail>;
        type ArgType<'de> = OrdersChannelArg<'de>;

        fn subscribe_message(&self) -> String {
            let OrdersChannel(inst_type) = self;
            serde_json::json!({
                "op": "subscribe",
                "args": [
                    {
                        "channel": Self::CHANNEL,
                        "instType": inst_type,
                    }
                ]
            })
            .to_string()
        }
    }
}