use crate::api::v5::model::{
    Category, InstrumentType, OrderState, OrderType, PositionSide, QuantityType, Side,
    StopLossTriggerPriceType, TakeProfitTriggerPriceType, TradeMode,
};
use crate::api::v5::{ExecType, Request, SelfTradePreventionMode};
use crate::serde_util::{deserialize_from_opt_str, str_opt, MaybeFloat, MaybeString, MaybeU64};
use crate::websocket::WebsocketChannel;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

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
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cl_ord_id: MaybeString,
    pub ord_id: String,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub s_code: MaybeU64,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub s_msg: MaybeString,
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
    pub ord_type: OrderType,
    /// Quantity to buy or sell
    pub sz: String,
    /// Order price. Only applicable to limit,post_only,fok,ioc,mmp,mmp_and_post_only order.
    /// When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
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
    pub tp_trigger_px: Option<String>,
    /// Take-profit order price
    /// If you fill in this parameter, you should fill in the take-profit trigger price as well.
    /// If the price is -1, take-profit will be executed at the market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss trigger price
    /// If you fill in this parameter, you should fill in the stop-loss order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop-loss order price
    /// If you fill in this parameter, you should fill in the stop-loss trigger price.
    /// If the price is -1, stop-loss will be executed at the market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
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
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ord_id: MaybeString,
    /// Client Order ID as assigned by the client
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cl_ord_id: MaybeString,
    /// Order tag
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub tag: MaybeString,
    /// The code of the event execution result, 0 means success.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub s_code: MaybeU64,
    /// Rejection or success message of event execution.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub s_msg: MaybeString,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    pub inst_type: InstrumentType,
    pub inst_id: String,
    #[serde(default, with = "str_opt")]
    pub tgt_ccy: Option<QuantityType>,
    #[serde(default, with = "str_opt")]
    pub ccy: MaybeString,
    #[serde(default, with = "str_opt")]
    pub ord_id: MaybeString,
    #[serde(default, with = "str_opt")]
    pub cl_ord_id: MaybeString,
    #[serde(default, with = "str_opt")]
    pub tag: MaybeString,
    #[serde(default, with = "str_opt")]
    pub px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub sz: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub pnl: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub ord_type: Option<OrderType>,
    #[serde(default, with = "str_opt")]
    pub side: Option<Side>,
    #[serde(default, with = "str_opt")]
    pub pos_side: Option<PositionSide>,
    #[serde(default, with = "str_opt")]
    pub td_mode: Option<TradeMode>,
    #[serde(default, with = "str_opt")]
    pub acc_fill_sz: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub fill_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub trade_id: MaybeString,
    #[serde(default, with = "str_opt")]
    pub fill_sz: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub fill_time: MaybeU64,
    #[serde(default, with = "str_opt")]
    pub avg_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub state: Option<OrderState>,
    #[serde(default, with = "str_opt")]
    pub lever: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub tp_trigger_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub tp_trigger_px_type: Option<TakeProfitTriggerPriceType>,
    #[serde(default, with = "str_opt")]
    pub tp_ord_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub sl_trigger_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub sl_trigger_px_type: Option<StopLossTriggerPriceType>,
    #[serde(default, with = "str_opt")]
    pub sl_ord_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub fee_ccy: MaybeString,
    #[serde(default, with = "str_opt")]
    pub fee: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub rebate_ccy: MaybeString,
    #[serde(default, with = "str_opt")]
    pub source: MaybeString,
    #[serde(default, with = "str_opt")]
    pub rebate: MaybeString,
    #[serde(default, with = "str_opt")]
    pub category: Option<Category>,
    #[serde(default, with = "str_opt")]
    pub u_time: MaybeU64,
    #[serde(default, with = "str_opt")]
    pub c_time: MaybeU64,
    #[serde(default, with = "str_opt")]
    pub exec_type: Option<ExecType>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailRef<'a> {
    pub inst_type: InstrumentType,
    pub inst_id: &'a str,
    #[serde(default, with = "str_opt")]
    pub tgt_ccy: Option<QuantityType>,
    #[serde(default)]
    pub ccy: Option<&'a str>,
    #[serde(default)]
    pub ord_id: Option<&'a str>,
    #[serde(default)]
    pub cl_ord_id: Option<&'a str>,
    #[serde(default)]
    pub tag: Option<&'a str>,
    #[serde(default, with = "str_opt")]
    pub px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub sz: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub pnl: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub ord_type: Option<OrderType>,
    #[serde(default, with = "str_opt")]
    pub side: Option<Side>,
    #[serde(default, with = "str_opt")]
    pub pos_side: Option<PositionSide>,
    #[serde(default, with = "str_opt")]
    pub td_mode: Option<TradeMode>,
    #[serde(default, with = "str_opt")]
    pub acc_fill_sz: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub fill_px: MaybeFloat,
    #[serde(default)]
    pub trade_id: Option<&'a str>,
    #[serde(default, with = "str_opt")]
    pub fill_sz: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub fill_time: MaybeU64,
    #[serde(default, with = "str_opt")]
    pub avg_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub state: Option<OrderState>,
    #[serde(default, with = "str_opt")]
    pub lever: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub tp_trigger_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub tp_trigger_px_type: Option<TakeProfitTriggerPriceType>,
    #[serde(default, with = "str_opt")]
    pub tp_ord_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub sl_trigger_px: MaybeFloat,
    #[serde(default, with = "str_opt")]
    pub sl_trigger_px_type: Option<StopLossTriggerPriceType>,
    #[serde(default, with = "str_opt")]
    pub sl_ord_px: MaybeFloat,
    #[serde(default)]
    pub fee_ccy: Option<&'a str>,
    #[serde(default, with = "str_opt")]
    pub fee: MaybeFloat,
    #[serde(default)]
    pub rebate_ccy: Option<&'a str>,
    #[serde(default)]
    pub source: Option<&'a str>,
    #[serde(default)]
    pub rebate: Option<&'a str>,
    #[serde(default, with = "str_opt")]
    pub category: Option<Category>,
    #[serde(default, with = "str_opt")]
    pub u_time: MaybeU64,
    #[serde(default, with = "str_opt")]
    pub c_time: MaybeU64,
    #[serde(default, with = "str_opt")]
    pub exec_type: Option<ExecType>,
}

impl Request for GetOrderDetails {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/trade/order";
    const AUTH: bool = true;

    type Response = Vec<OrderDetail>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-trade-get-order-list
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderList {
    #[serde(default, with = "str_opt")]
    pub inst_type: Option<InstrumentType>,
    #[serde(default, with = "str_opt")]
    pub uly: Option<String>,
    #[serde(default, with = "str_opt")]
    pub inst_id: Option<String>,
    #[serde(default, with = "str_opt")]
    pub ord_type: Option<OrderType>,
    #[serde(default, with = "str_opt")]
    pub state: Option<OrderState>,
    #[serde(default, with = "str_opt")]
    pub after: Option<String>,
    #[serde(default, with = "str_opt")]
    pub before: Option<String>,
    #[serde(default, with = "str_opt")]
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
        type Response<'de> = [OrderDetailRef<'de>; 1];
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

pub struct OrderOp;
impl WebsocketChannel for OrderOp {
    const CHANNEL: &'static str = "";
    type Response<'de> = [PlaceOrderResponse; 1];
    type ArgType<'de> = ();
}
