use crate::api::v5::Request;
use crate::serde_util::deserialize_timestamp;
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::api::v5::model::AccountType;
use crate::serde_util::*;

#[derive(Debug, Deserialize, Clone)]
pub enum DepositStatus {
    WaitingForConfirmation,
    Credited,
    Complete,
    /// pending due to temporary deposit suspension on this crypto currency
    Pending,
    Unknown(String),
}

impl FromStr for DepositStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::WaitingForConfirmation,
            "1" => Self::Credited,
            "2" => Self::Complete,
            "8" => Self::Pending,
            unknown => Self::Unknown(unknown.to_owned()),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistory {
    pub ccy: String,
    pub chain: String,
    pub amt: Decimal,
    pub from: String,
    pub to: String,
    pub tx_id: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: DepositStatus,
    pub dep_id: String,
    pub actual_dep_blk_confirm: String,
}

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-deposit-history
/// ## Get deposit history
/// Retrieve the deposit records according to the currency, deposit status, and time range in reverse chronological order. The 100 most recent records are returned by default.
/// Websocket API is also available, refer to Deposit info channel.
///
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/deposit-history
///
/// <table><thead>
/// <tr>
/// <th style="text-align: left">Parameter</th>
/// <th style="text-align: left">Type</th>
/// <th style="text-align: left">Required</th>
/// <th style="text-align: left">Description</th>
/// </tr>
/// </thead><tbody>
/// <tr>
/// <td style="text-align: left">ccy</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Currency, e.g. <code>BTC</code></td>
/// </tr>
/// <tr>
/// <td style="text-align: left">depId</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Deposit ID</td>
/// </tr>
/// <tr>
/// <td style="text-align: left">fromWdId</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Internal transfer initiator's withdrawal ID<br>If the deposit comes from internal transfer, this field displays the withdrawal ID of the internal transfer initiator</td>
/// </tr>
/// <tr>
/// <td style="text-align: left">txId</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Hash record of the deposit</td>
/// </tr>
/// <tr>
/// <td style="text-align: left">type</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Deposit Type<br><code>3</code>: internal transfer<br><code>4</code>: deposit from chain</td>
/// </tr>
/// <tr>
/// <td style="text-align: left">state</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Status of deposit <br><code>0</code>: waiting for confirmation<br><code>1</code>: deposit credited  <br><code>2</code>: deposit successful <br><code>8</code>: pending due to temporary deposit suspension on this crypto currency<br><code>11</code>: match the address blacklist<br><code>12</code>: account or deposit is frozen<br><code>13</code>: sub-account deposit interception<br><code>14</code>: KYC limit</td>
/// </tr>
/// <tr>
/// <td style="text-align: left">after</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Pagination of data to return records earlier than the requested ts, Unix timestamp format in milliseconds, e.g. <code>1654041600000</code></td>
/// </tr>
/// <tr>
/// <td style="text-align: left">before</td>
/// <td style="text-align: left">String</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Pagination of data to return records newer than the requested ts, Unix timestamp format in milliseconds, e.g. <code>1656633600000</code></td>
/// </tr>
/// <tr>
/// <td style="text-align: left">limit</td>
/// <td style="text-align: left">string</td>
/// <td style="text-align: left">No</td>
/// <td style="text-align: left">Number of results per request. The maximum is <code>100</code>; The default is <code>100</code></td>
/// </tr>
/// </tbody></table>
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositHistory {
    /// Currency, e.g. BTC
    pub ccy: Option<String>,
    /// Deposit ID
    pub dep_id: Option<String>,
    /// Internal transfer initiator's withdrawal ID
    /// If the deposit comes from internal transfer, this field displays the withdrawal ID of the internal transfer initiator
    pub from_wd_id: Option<String>,
    /// Hash record of the deposit
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
mod test_get_deposit_history {
    use crate::api::v5::testkit::test_with_credentials;
    use super::*;

    #[tokio::test]
    async fn test_get_deposit_history() {
        test_with_credentials(|rest| async move {
            let req = GetDepositHistory::default();
            let rval = rest.request(req).await.unwrap();
            println!("{:?}", rval);
        }).await;
    }
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
    /// Object	Deposit address attachment (This will not be returned if the currency does not require this)
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
