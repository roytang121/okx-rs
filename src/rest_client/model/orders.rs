use crate::rest_client::model::{
    Category, InstrumentType, OrderState, OrderType, PositionSide, QuantityType, Request, Side,
    StopLossTriggerPriceType, TakeProfitTriggerPriceType, TradeMode,
};
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
    pub inst_id: String,
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub td_mode: TradeMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub side: Side,
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub pos_side: Option<PositionSide>,
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub ord_type: OrderType,
    pub sz: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub tgt_ccy: Option<QuantityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_amend: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderData {
    pub cl_ord_id: String,
    pub ord_id: String,
    pub tag: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub s_code: u32,
    pub s_msg: String,
}

impl Request for PlaceOrder {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/trade/order";
    const AUTH: bool = true;

    type Response = Vec<PlaceOrderData>;
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
    pub px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sz: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pnl: Option<Decimal>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub ord_type: OrderType,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub side: Side,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub pos_side: Option<PositionSide>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub td_mode: Option<TradeMode>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub acc_fill_sz: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fill_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub trade_id: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fill_sz: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fill_time: Option<u64>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avg_px: Option<Decimal>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: OrderState,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub lever: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tp_trigger_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tp_trigger_px_type: Option<TakeProfitTriggerPriceType>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub tp_ord_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sl_trigger_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sl_trigger_px_type: Option<StopLossTriggerPriceType>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sl_ord_px: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fee_ccy: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fee: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub rebate_ccy: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub source: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub rebate: Option<Decimal>,
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
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub inst_type: Option<InstrumentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub ord_type: Option<OrderType>,
    #[serde(
        serialize_with = "crate::serde_util::serialize_as_str_opt",
        skip_serializing_if = "Option::is_none"
    )]
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
