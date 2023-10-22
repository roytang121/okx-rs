use crate::api::v5::Request;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::api::v5::model::{DepositAddress, DepositHistory};
use crate::impl_string_enum;
use crate::serde_util::*;

#[derive(Debug, Clone)]
pub enum DepositStatus {
    WaitingForConfirmation,
    DepositCredited,
    DepositSuccessful,
    /// pending due to temporary deposit suspension on this crypto currency
    Pending,
    /// match the address blacklist
    MatchAddressBlacklist,
    /// account or deposit is frozen
    AccountOrDepositFrozen,
    /// sub-account deposit interception
    SubAccountDepositInterception,
    /// KYC Limit
    KycLimit,
    Unknown(String),
}

impl_string_enum!(DepositStatus,
    Unknown,
    WaitingForConfirmation => "0",
    DepositCredited => "1",
    DepositSuccessful => "2",
    Pending => "8",
    MatchAddressBlacklist => "11",
    AccountOrDepositFrozen => "12",
    SubAccountDepositInterception => "13",
    KycLimit => "14",
);

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-deposit-history
/// ## Get deposit history
/// Retrieve the deposit records according to the currency, deposit status, and time range in reverse chronological order. The 100 most recent records are returned by default.
/// Websocket API is also available, refer to Deposit info channel.
///
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/deposit-history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositHistory {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Deposit ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dep_id: Option<String>,
    /// Internal transfer initiator's withdrawal ID
    /// If the deposit comes from internal transfer, this field displays the withdrawal ID of the internal transfer initiator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_wd_id: Option<String>,
    /// Hash record of the deposit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Deposit Type
    /// 3: internal transfer
    /// 4: deposit from chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Status of deposit
    /// 0: waiting for confirmation
    /// 1: deposit credited
    /// 2: deposit successful
    /// 8: pending due to temporary deposit suspension on this crypto currency
    /// 11: match the address blacklist
    /// 12: account or deposit is frozen
    /// 13: sub-account deposit interception
    /// 14: KYC limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Pagination of data to return records earlier than the requested ts, Unix timestamp format in milliseconds, e.g. 1654041600000
    #[serde(
        serialize_with = "serialize_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub after: Option<DateTime<Utc>>,
    /// Pagination of data to return records newer than the requested ts, Unix timestamp format in milliseconds, e.g. 1656633600000
    #[serde(
        serialize_with = "serialize_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub before: Option<DateTime<Utc>>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Request for GetDepositHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/deposit-history";
    const AUTH: bool = true;

    type Response = Vec<DepositHistory>;
}

// gen test get deposit history
#[cfg(test)]
mod tests_get_deposit_history {
    use crate::api::v5::testkit::test_with_credentials;
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_deser() {
        test_with_credentials(|rest| async move {
            let req = GetDepositHistory::default();
            let rval = rest.request(req).await.unwrap();
            println!("{:?}", rval);
        }).await;
    }
}


/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-deposit-address
/// ## Get deposit address
/// Retrieve the deposit addresses of currencies, including previously-used addresses.
///
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/deposit-address
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositAddress {
    /// Currency, e.g. BTC
    pub ccy: String,
}

impl Request for GetDepositAddress {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/deposit-address";
    const AUTH: bool = true;
    type Response = Vec<DepositAddress>;
}
