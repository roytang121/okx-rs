use crate::api::v5::{
    AccountBill, AccountBillSubType, AccountBillType, AssetBill, Currency, FundingBalance, Request,
};
use crate::serde_util::*;

use reqwest::Method;
use serde::{Deserialize, Serialize};

pub mod bill;
pub mod deposit;
pub mod transfer;
pub mod withdrawal;

/// https://www.okx.com/docs/en/#rest-api-funding-get-currencies
/// ## Get currencies
/// Retrieve a list of all currencies.
///
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/currencies
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrencies {
    /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl Request for GetCurrencies {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/currencies";
    const AUTH: bool = true;

    type Response = Vec<Currency>;
}

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-balance
/// Get balance
/// Retrieve the funding account balances of all the assets and the amount that is available or on hold.
///
///  Only asset information of a currency with a balance greater than 0 will be returned.
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// HTTP Request
/// GET /api/v5/asset/balances
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingBalances {
    /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl Request for GetFundingBalances {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/balances";
    const AUTH: bool = true;
    type Response = Vec<FundingBalance>;
}

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-account-asset-valuation
/// ## Get account asset valuation
/// View account asset valuation
///
/// Rate Limit: 1 request per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/asset-valuation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountAssetValuation {
    /// Asset valuation calculation unit
    /// BTC, USDT
    /// USD, CNY, JP, KRW, RUB, EUR
    /// VND, IDR, INR, PHP, THB, TRY
    /// AUD, SGD, ARS, SAR, AED, IQD
    /// The default is the valuation in BTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl Request for GetAccountAssetValuation {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/asset-valuation";
    const AUTH: bool = true;
    type Response = Vec<AccountAssetValuation>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAssetValuation {
    /// Valuation of total account assets
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub total_bal: MaybeFloat,
    /// Unix timestamp format in milliseconds, e.g.<code>1597026383085</code>
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ts: Option<u64>,
    pub details: AccountAssetValuationDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAssetValuationDetails {
    /// Funding account
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub funding: MaybeFloat,
    /// Trading account
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub trading: MaybeFloat,
    /// [Deprecated] Classic account
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    #[deprecated(note = "[Deprecated] Classic account")]
    pub classic: MaybeFloat,
    /// Earn account
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub earn: MaybeFloat,
}

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-bills-details-last-7-days
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountBills {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AccountBillType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<AccountBillSubType>,
}

impl Request for GetAccountBills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/bills";
    const AUTH: bool = true;
    type Response = Vec<AccountBill>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-funding-asset-bills-details
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetBills {}

impl Request for GetAssetBills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/bills";
    const AUTH: bool = true;

    type Response = Vec<AssetBill>;
}
