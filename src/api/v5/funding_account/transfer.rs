//! https://www.okx.com/docs-v5/en/#rest-api-funding-get-funds-transfer-state

use crate::api::v5::model::{AccountType, FundTransferHistory, TransferType};
use crate::api::v5::Request;
use crate::serde_util::{deserialize_from_opt_str, MaybeFloat};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-funds-transfer-state
/// ## Get funds transfer state
/// Retrieve the transfer state data of the last 2 weeks.
///
/// Rate Limit: 1 request per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/transfer-state
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFundTransferHistory {
    /// Transfer ID
    /// Either transId or clientId is required. If both are passed, transId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_id: Option<String>,
    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Transfer type
    /// The default is WithinAccount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransferType>,
}

impl Request for GetFundTransferHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/transfer-state";
    const AUTH: bool = true;

    type Response = Vec<FundTransferHistory>;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundTransferResponse {
    pub trans_id: String,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub client_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ccy: Option<String>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub amt: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub from: Option<AccountType>,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub to: Option<AccountType>,
}

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-funds-transfer
/// ## Funds transfer
/// Only API keys with Trade privilege can call this endpoint.
///
/// This endpoint supports the transfer of funds between your funding account and trading account, and from the master account to sub-accounts.
///
/// Sub-account can transfer out to master account by default. Need to call Set permission of transfer out to grant privilege first if you want sub-account transferring to another sub-account (sub-accounts need to belong to same master account.)
///
/// **Failure of the request does not mean the transfer has failed. Recommend to call "Get funds transfer state" to confirm the status.**
///
/// Rate Limit: 1 request per second
/// Rate limit rule: UserID + Currency
/// ## HTTP Request
/// POST /api/v5/asset/transfer
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundsTransfer {
    /// Transfer type
    pub r#type: TransferType,
    /// Transfer currency, e.g. USDT
    pub ccy: String,
    /// Amount to be transferred
    #[serde(default)]
    pub amt: MaybeFloat,
    /// The remitting account
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub from: AccountType,
    /// The beneficiary account
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub to: AccountType,
    /// Name of the sub-account
    /// When type is 1/2/4, this parameter is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl Request for FundsTransfer {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/asset/transfer";
    const AUTH: bool = true;

    type Response = Vec<FundTransferResponse>;
}
