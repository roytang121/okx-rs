use crate::api::v5::Request;
use crate::impl_string_enum;
use crate::serde_util::{deserialize_from_opt_str, deserialize_timestamp, MaybeFloat};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum WithdrawalStatus {
    PendingCancel,
    Canceled,
    Failed,
    Pending,
    Sending,
    Sent,
    AwaitingEmailVerification,
    AwaitingManualVerification,
    AwaitingIdentifyVerification,
    Approved,
    WaitingTransfer,
    Unknown(String),
}

impl_string_enum!(WithdrawalStatus,
    Unknown,
    PendingCancel => "-3",
    Canceled => "-2",
    Failed => "-1",
    Pending => "0",
    Sending => "1",
    Sent => "2",
    AwaitingEmailVerification => "3",
    AwaitingManualVerification => "4",
    AwaitingIdentifyVerification => "5",
    Approved => "7",
    WaitingTransfer => "10",
);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalHistory {
    /// Currency
    pub ccy: String,
    /// Chain name, e.g. USDT-ERC20, USDT-TRC20
    pub chain: String,
    /// Withdrawal amount
    #[serde(default)]
    pub amt: MaybeFloat,
    /// Time the withdrawal request was submitted, Unix timestamp format in milliseconds, e.g. 1655251200000.
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    /// Withdrawal account
    /// It can be email/phone
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub from: Option<String>,
    /// Receiving address
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub to: Option<String>,
    /// Some currencies require a tag for withdrawals. This is not returned if not required.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub tag: Option<String>,
    /// Some currencies require a payment ID for withdrawals. This is not returned if not required.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub pmt_id: Option<String>,
    /// Some currencies require this parameter for withdrawals. This is not returned if not required.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub memo: Option<String>,
    /// Hash record of the withdrawal.
    /// This parameter will returned "" for internal transfers.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub tx_id: Option<String>,
    /// Withdrawal fee amount
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub fee: Option<String>,
    /// Status of withdrawal
    pub state: WithdrawalStatus,
    /// Withdrawal ID
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub wd_id: Option<String>,
    /// Client-supplied ID
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub client_id: Option<String>,
}

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-withdrawal
/// ## Withdrawal
/// Withdrawal of tokens. Common sub-account does not support withdrawal.
///
/// > *The API can only make withdrawal to verified addresses, and verified addresses can be set by WEB/APP.*
/// > *About tag: Some token deposits require a deposit address and a tag (e.g. Memo/Payment ID), which is a string that guarantees the uniqueness of your deposit address. Follow the deposit procedure carefully, or you may risk losing your assets.*
/// For currencies with labels, if it is a withdrawal between OKX users, please use internal transfer instead of online withdrawal
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// POST /api/v5/asset/withdrawal
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequest {
    /// Currency, e.g. USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Withdrawal amount
    /// Withdrawal fee is not included in withdrawal amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: MaybeFloat,
    /// Withdrawal method
    /// 3: internal transfer
    /// 4: on-chain withdrawal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    /// If your dest is 4,toAddr should be a trusted crypto currency address. Some crypto currency addresses are formatted as 'address:tag', e.g. 'ARDOR-7JF3-8F2E-QUWZ-CAN7F:123456'
    /// If your dest is 3,toAddr should be a recipient address which can be email, phone or login account name (account name is only for sub-account).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addr: Option<String>,
    /// Transaction fee
    /// You can get fee amount by the endpoint of Get currencies.
    /// For internal transfer, transaction fee is always 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Chain name
    /// There are multiple chains under some currencies, such as USDT has USDT-ERC20, USDT-TRC20
    /// If the parameter is not filled in, the default will be the main chain.
    /// When you withdrawal the non-tradable asset, if the parameter is not filled in, the default will be the unique withdrawal chain.
    /// Apply to on-chain withdrawal.
    /// You can get supported chain name by the endpoint of Get currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Area code for the phone number, e.g. 86
    /// If toAddr is a phone number, this parameter is required.
    /// Apply to internal transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    /// Receiver's info
    /// Specific country/region certified users need to provide this information for on-chain withdrawal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcvr_info: Option<String>,
    /// Wallet Type
    /// exchange: Withdraw to exchange wallet
    /// private: Withdraw to private wallet
    /// If you withdraw to exchange wallet,exchId&rcvrFirstName&rcvrLastName is required
    /// If you withdraw to private wallet, no additional information is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
    /// Exchange ID
    /// You can query supported exchanges through the endpoint of Get exchange list (public)
    /// If the exchange is not in the exchange list, fill in '0' in this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exch_id: Option<String>,
    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

/// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-withdrawal-history
/// ## Get withdrawal history
/// Retrieve the withdrawal records according to the currency, withdrawal status, and time range in reverse chronological order. The 100 most recent records are returned by default.
/// Websocket API is also available, refer to Withdrawal info channel.
///
/// Rate Limit: 6 requests per second
/// Rate limit rule: UserID
/// ## HTTP Request
/// GET /api/v5/asset/withdrawal-history
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalHistory {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Withdrawal ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wd_id: Option<String>,
    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Hash record of the deposit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Withdrawal type
    /// 3: internal transfer
    /// 4: withdrawal to chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Status of withdrawal
    /// -3: canceling
    /// -2: canceled
    /// -1: failed
    /// 0: waiting withdrawal
    /// 1: withdrawing
    /// 2: withdraw success
    /// 7: approved
    /// 10: waiting transfer
    /// 4, 5, 6, 8, 9, 12: waiting mannual review
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WithdrawalStatus>,
    /// Pagination of data to return records earlier than the requested ts, Unix timestamp format in milliseconds, e.g. 1654041600000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ts, Unix timestamp format in milliseconds, e.g. 1656633600000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl Request for GetWithdrawalHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/withdrawal-history";
    const AUTH: bool = true;

    type Response = Vec<WithdrawalHistory>;
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResponse {
    /// Withdrawal amount
    #[serde(default)]
    pub amt: MaybeFloat,
    /// Currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ccy: Option<String>,
    /// Withdrawal ID
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub wd_id: Option<String>,
    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub client_id: Option<String>,
    /// Chain name, e.g. USDT-ERC20, USDT-TRC20
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub chain: Option<String>,
}

impl Request for WithdrawalRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/asset/withdrawal";
    const AUTH: bool = true;

    type Response = Vec<WithdrawalResponse>;
}
