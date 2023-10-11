//! https://www.okx.com/docs-v5/en/#rest-api-funding-get-funds-transfer-state

use crate::api::v5::Request;
use crate::impl_serde_from_str;
use crate::serde_util::deserialize_from_opt_str;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferType {
    WithinAccount,
    MasterToSubAccount,
    SubAccountToMaster,
    SubAccountToSubAccount,
}
impl FromStr for TransferType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::WithinAccount,
            "1" => Self::MasterToSubAccount,
            // sub account to master (api key from master account)
            "2" => Self::SubAccountToMaster,
            // "3" is sub account to master (api key from sub account)
            // although sub-account to sub-account is documented from api doc, it is not
            // possible to perform such kind of transfer on OKx GUI
            // and it is also not possible to reconcile funds transfer between sub-account
            "4" => Self::SubAccountToSubAccount,
            unknown => anyhow::bail!("unknown Fund transfer type {}", unknown),
        })
    }
}
impl Display for TransferType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferType::WithinAccount => write!(f, "0"),
            TransferType::MasterToSubAccount => write!(f, "1"),
            TransferType::SubAccountToMaster => write!(f, "2"),
            TransferType::SubAccountToSubAccount => write!(f, "4"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccountType {
    Funding,
    Trading,
}
impl FromStr for AccountType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "6" => Self::Funding,
            "18" => Self::Trading,
            unknown => anyhow::bail!("unknown account type {}", unknown),
        })
    }
}
impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Funding => write!(f, "6"),
            AccountType::Trading => write!(f, "18"),
        }
    }
}
impl_serde_from_str!(AccountType);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FundTransferState {
    Success,
    Pending,
    Failed,
}
impl FromStr for FundTransferState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "success" => Self::Success,
            "pending" => Self::Pending,
            "failed" => Self::Failed,
            unknown => anyhow::bail!("unknown Fund transfer state {}", unknown),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundTransferHistory {
    pub trans_id: String,
    pub client_id: String,
    pub ccy: String,
    pub amt: Decimal,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub r#type: TransferType,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub from: AccountType,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub to: AccountType,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sub_acct: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub state: FundTransferState,
}

/// https://www.okx.com/docs-v5/en/#rest-api-funding-get-funds-transfer-state
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFundTransferHistory {
    pub trans_id: String,
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
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub client_id: Option<String>,
    pub ccy: String,
    pub amt: Decimal,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub from: AccountType,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub to: AccountType,
}

/// https://www.okx.com/docs-v5/en/#rest-api-funding-funds-transfer
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundTransferRequest {
    pub ccy: String,
    pub amt: Decimal,
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub from: AccountType,
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub to: AccountType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
    pub r#type: TransferType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl Request for FundTransferRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/asset/transfer";
    const AUTH: bool = true;

    type Response = Vec<FundTransferResponse>;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingBalance {
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub avail_bal: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub bal: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub frozen_bal: Option<Decimal>,
    pub ccy: String,
}

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-balance
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingBalances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl Request for GetFundingBalances {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/balances";
    const AUTH: bool = true;
    type Response = Vec<FundingBalance>;
}
