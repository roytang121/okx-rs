//! https://www.okx.com/docs-v5/en/#rest-api-funding-get-funds-transfer-state

use crate::rest_client::model::{ExecType, InstrumentType, MarginMode, Request};
use crate::serde_util::{
    deserialize_from_opt_str, deserialize_timestamp, deserialize_timestamp_opt,
};
use anyhow::bail;
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::str::FromStr;

/// https://www.okx.com/docs-v5/en/#rest-api-funding-asset-bills-details
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetBills {}

#[derive(Debug, Deserialize, Clone)]
pub enum AssetBillType {
    Deposit,
    Withdrawal,
    CanceledWithdrawal,
    TransferToSubAccount,
    TransferFromSubAccount,
    TransferFromTradingAccount,
    TransferToTradingAccount,
    Other(String),
}

impl FromStr for AssetBillType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Deposit,
            "1" => Self::Withdrawal,
            "2" => Self::CanceledWithdrawal,
            "20" => Self::TransferToSubAccount,
            "21" => Self::TransferFromSubAccount,
            "130" => Self::TransferFromTradingAccount,
            "131" => Self::TransferToTradingAccount,
            unknown => Self::Other(unknown.to_owned()),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetBill {
    pub bill_id: String,
    pub ccy: String,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub client_id: Option<String>,
    pub bal_chg: Decimal,
    pub bal: Decimal,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub r#type: AssetBillType,
    #[serde(default, deserialize_with = "deserialize_timestamp_opt")]
    pub ts: Option<DateTime<Utc>>,
}

impl Request for GetAssetBills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/bills";
    const AUTH: bool = true;

    type Response = Vec<AssetBill>;
}

/// https://www.okx.com/docs-v5/en/#rest-api-subaccount-history-of-sub-account-transfer
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountBills {}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountBill {
    pub bill_id: String,
    pub ccy: String,
    pub amt: Decimal,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub r#type: SubAccountBillType,
    pub sub_acct: String,
    #[serde(default, deserialize_with = "deserialize_timestamp_opt")]
    pub ts: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum SubAccountBillType {
    MasterToSubAccount,
    SubAccountToMaster,
}

impl FromStr for SubAccountBillType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::MasterToSubAccount,
            "1" => Self::SubAccountToMaster,
            unknown => anyhow::bail!("unknown SubAccountBillType {}", unknown),
        })
    }
}

impl Request for GetSubAccountBills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/asset/subaccount/bills";
    const AUTH: bool = true;

    type Response = Vec<SubAccountBill>;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBill {
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub inst_type: InstrumentType,
    pub bill_id: String,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub r#type: AccountBillType,
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub sub_type: AccountBillSubType,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub ts: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub bal_chg: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub post_bal_chg: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub bal: Option<Decimal>,
    // #[serde(deserialize_with = "deserialize_from_opt_str")]
    // pub post_bal: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub sz: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub ccy: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub fee: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub mgn_mode: Option<MarginMode>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub inst_id: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub ord_id: Option<String>,
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub exec_type: Option<ExecType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AccountBillType {
    InterestDeduction, // 7
    FundingFee,        // 8
}

impl FromStr for AccountBillType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "7" => Self::InterestDeduction,
            "8" => Self::FundingFee,
            other => bail!("unhandled bill type {}", other),
        })
    }
}

impl std::fmt::Display for AccountBillType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountBillType::InterestDeduction => write!(f, "7"),
            AccountBillType::FundingFee => write!(f, "8"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AccountBillSubType {
    InterestDeductionForMarketLoans, // 9
    FundingFeeExpense,               // 173
    FundingFeeIncome,                // 174
}

impl FromStr for AccountBillSubType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "9" => Self::InterestDeductionForMarketLoans,
            "173" => Self::FundingFeeExpense,
            "174" => Self::FundingFeeIncome,
            other => bail!("unhandled bill sub_type {}", other),
        })
    }
}

impl std::fmt::Display for AccountBillSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountBillSubType::InterestDeductionForMarketLoans => write!(f, "9"),
            AccountBillSubType::FundingFeeExpense => write!(f, "173"),
            AccountBillSubType::FundingFeeIncome => write!(f, "174"),
        }
    }
}

/// https://www.okx.com/docs-v5/en/#rest-api-account-get-bills-details-last-7-days
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountBills {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serde_util::serialize_as_str_opt"
    )]
    pub r#type: Option<AccountBillType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serde_util::serialize_as_str_opt"
    )]
    pub sub_type: Option<AccountBillSubType>,
}

impl Request for GetAccountBills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/bills";
    const AUTH: bool = true;
    type Response = Vec<AccountBill>;
}

#[cfg(test)]
mod tests {
    use crate::rest_client::model::AssetBill;

    #[test]
    fn parse_bill_empty_client_id() {
        let json = r#"
        {
            "billId": "12344",
            "ccy": "BTC",
            "clientId": "",
            "balChg": "2",
            "bal": "12",
            "type": "1",
            "ts": "1597026383085"
        }
        "#;
        let bill = serde_json::from_str::<AssetBill>(json).unwrap();
        assert!(bill.client_id.is_none());
    }
}
