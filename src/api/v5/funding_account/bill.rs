//! https://www.okx.com/docs-v5/en/#rest-api-funding-get-funds-transfer-state

use crate::api::v5::model::{InstrumentType, MarginMode};
use crate::api::v5::{ExecType, Request};
use crate::impl_string_enum;
use crate::serde_util::{
    deserialize_from_opt_str, deserialize_timestamp, deserialize_timestamp_opt, MaybeString,
};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum AssetBillType {
    Deposit,
    Withdrawal,
    CanceledWithdrawal,
    TransferToSubAccount,
    TransferFromSubAccount,
    TransferOutFromSubToMasterAccount,
    TransferInFromMasterToSubAccount,
    ManuallyClaimedAirdrop,
    SystemReversal,
    EventReward,
    EventGiveaway,
    ReceivedFromAppointments,
    DeductedFromAppointments,
    RedPacketSent,
    RedPacketSnatched,
    RedPacketRefunded,
    Conversion,
    ClaimRebateCard,
    DistributeRebateCard,
    TokenReceived,
    TokenGivenAway,
    TokenRefunded,
    SubscriptionToSavings,
    RedemptionToSavings,
    Distribute,
    LockUp,
    NodeVoting,
    DeFiStackingPurchase,
    VoteRedemption,
    DeFiStackingRedemption,
    StakingYield,
    ViolationFee,
    PoWMiningYield,
    CloudMiningPay,
    CloudMiningYield,
    Subsidy,
    DeFiYield,
    AddCollateral,
    RedeemCollateral,
    Investment,
    BorrowerBorrows,
    PrincipalTransferredIn,
    BorrowerTransferredLoanOut,
    BorrowerTransferredInterestOut,
    InvestorTransferredInterestIn,
    PrepaymentPenaltyTransferredIn,
    PrepaymentPenaltyTransferredOut,
    MortgageFeeTransferredIn,
    MortgageFeeTransferredOut,
    OverdueFeeTransferredIn,
    OverdueFeeTransferredOut,
    OverdueInterestTransferredOut,
    OverdueInterestTransferredIn,
    CollateralForClosedPositionTransferredIn,
    CollateralForClosedPositionTransferredOut,
    CollateralForLiquidationTransferredIn,
    CollateralForLiquidationTransferredOut,
    InsuranceFundTransferredIn,
    InsuranceFundTransferredOut,
    PlaceAnOrder,
    FulfillAnOrder,
    CancelAnOrder,
    MerchantsUnlockDeposit,
    MerchantsAddDeposit,
    FiatGatewayPlaceAnOrder,
    FiatGatewayCancelAnOrder,
    FiatGatewayFulfillAnOrder,
    JumpstartUnlocking,
    ManualDeposit,
    InterestDeposit,
    InvestmentFeeTransferredIn,
    InvestmentFeeTransferredOut,
    RewardsTransferredIn,
    TransferFromTradingAccount,
    TransferToTradingAccount,
    FrozenByCustomerService,
    UnfrozenByCustomerService,
    TransferredByCustomerService,
    CrossChainExchange,
    Convert,
    ETH20Subscription,
    ETH20Swapping,
    ETH20Earnings,
    SystemReverse,
    TransferOutOfUnifiedAccountReserve,
    RewardExpired,
    CustomerFeedback,
    VestedSushiRewards,
    AffiliateCommission,
    ReferralReward,
    BrokerReward,
    JoinerReward,
    MysteryBoxReward,
    RewardsWithdraw,
    FeeRebate,
    CollectedCrypto,
    DualInvestmentSubscribe,
    DualInvestmentCollection,
    DualInvestmentProfit,
    DualInvestmentRefund,
    NewYearRewards,
    SubAffiliateCommission,
    Pay,
    LockedCollateral,
    Loan,
    AddedCollateral,
    ReturnedCollateral,
    Repayment,
    UnlockedCollateral,
    AirdropPayment,
    FeedbackBounty,
    InviteFriendsRewards,
    DivideTheRewardPool,
    BrokerConvertReward,
    FreeETH,
    ConvertTransfer,
    SlotAuctionConversion,
    MysteryBoxBonus,
    CardPaymentBuy,
    UntradableAssetWithdrawal,
    UntradableAssetWithdrawalRevoked,
    UntradableAssetDeposit,
    UntradableAssetCollectionReduce,
    UntradableAssetCollectionIncrease,
    Buy,
    PriceLockSubscribe,
    PriceLockCollection,
    PriceLockProfit,
    PriceLockRefund,
    DualInvestmentLiteSubscribe,
    DualInvestmentLiteCollection,
    DualInvestmentLiteProfit,
    DualInvestmentLiteRefund,
    WinCryptoWithSatoshi,
    MultiCollateralLoanCollateralLocked,
    CollateralTransferedOutFromUserAccount,
    CollateralReturnedToUsers,
    MultiCollateralLoanCollateralReleased,
    LoanTransferredToUsersAccount,
    MultiCollateralLoanBorrowed,
    MultiCollateralLoanRepaid,
    RepaymentTransferredFromUsersAccount,
    DelistedCrypto,
    BlockchainsWithdrawalFee,
    WithdrawalFeeRefund,
    CopyTradingProfitShare,
    ServiceFee,
    SharkFinSubscribe,
    SharkFinCollection,
    SharkFinProfit,
    SharkFinRefund,
    Airdrop,
    TokenMigrationCompleted,
    SubsidizedInterestReceived,
    BrokerRebateCompensation,
    StrategyBotsProfitShare,
    DCDBrokerTransfer,
    DCDBrokerRebate,
    TransferOutTradingSubAccount,
    TransferInTradingSubAccount,
    TransferOutCustodyFundingAccount,
    TransferInCustodyFundingAccount,
    CustodyFundDelegation,
    CustodyFundUndelegation,
    /// wildcard
    Other(String),
}

impl_string_enum!(AssetBillType,
    Other,
    Deposit => "1",
    Withdrawal => "2",
    CanceledWithdrawal => "13",
    TransferToSubAccount => "20",
    TransferFromSubAccount => "21",
    TransferOutFromSubToMasterAccount => "22",
    TransferInFromMasterToSubAccount => "23",
    ManuallyClaimedAirdrop => "28",
    SystemReversal => "47",
    EventReward => "48",
    EventGiveaway => "49",
    ReceivedFromAppointments => "50",
    DeductedFromAppointments => "51",
    RedPacketSent => "52",
    RedPacketSnatched => "53",
    RedPacketRefunded => "54",
    Conversion => "61",
    ClaimRebateCard => "68",
    DistributeRebateCard => "69",
    TokenReceived => "72",
    TokenGivenAway => "73",
    TokenRefunded => "74",
    SubscriptionToSavings => "75",
    RedemptionToSavings => "76",
    Distribute => "77",
    LockUp => "78",
    NodeVoting => "79",
    DeFiStackingPurchase => "80",
    VoteRedemption => "81",
    DeFiStackingRedemption => "82",
    StakingYield => "83",
    ViolationFee => "84",
    PoWMiningYield => "85",
    CloudMiningPay => "86",
    CloudMiningYield => "87",
    Subsidy => "88",
    DeFiYield => "89",
    AddCollateral => "92",
    RedeemCollateral => "93",
    Investment => "94",
    BorrowerBorrows => "95",
    PrincipalTransferredIn => "96",
    BorrowerTransferredLoanOut => "97",
    BorrowerTransferredInterestOut => "98",
    InvestorTransferredInterestIn => "99",
    PrepaymentPenaltyTransferredIn => "102",
    PrepaymentPenaltyTransferredOut => "103",
    MortgageFeeTransferredIn => "104",
    MortgageFeeTransferredOut => "105",
    OverdueFeeTransferredIn => "106",
    OverdueFeeTransferredOut => "107",
    OverdueInterestTransferredOut => "108",
    OverdueInterestTransferredIn => "109",
    CollateralForClosedPositionTransferredIn => "110",
    CollateralForClosedPositionTransferredOut => "111",
    CollateralForLiquidationTransferredIn => "112",
    CollateralForLiquidationTransferredOut => "113",
    InsuranceFundTransferredIn => "114",
    InsuranceFundTransferredOut => "115",
    PlaceAnOrder => "116",
    FulfillAnOrder => "117",
    CancelAnOrder => "118",
    MerchantsUnlockDeposit => "119",
    MerchantsAddDeposit => "120",
    FiatGatewayPlaceAnOrder => "121",
    FiatGatewayCancelAnOrder => "122",
    FiatGatewayFulfillAnOrder => "123",
    JumpstartUnlocking => "124",
    ManualDeposit => "125",
    InterestDeposit => "126",
    InvestmentFeeTransferredIn => "127",
    InvestmentFeeTransferredOut => "128",
    RewardsTransferredIn => "129",
    TransferFromTradingAccount => "130",
    TransferToTradingAccount => "131",
    FrozenByCustomerService => "132",
    UnfrozenByCustomerService => "133",
    TransferredByCustomerService => "134",
    CrossChainExchange => "135",
    Convert => "136",
    ETH20Subscription => "137",
    ETH20Swapping => "138",
    ETH20Earnings => "139",
    SystemReverse => "143",
    TransferOutOfUnifiedAccountReserve => "144",
    RewardExpired => "145",
    CustomerFeedback => "146",
    VestedSushiRewards => "147",
    AffiliateCommission => "150",
    ReferralReward => "151",
    BrokerReward => "152",
    JoinerReward => "153",
    MysteryBoxReward => "154",
    RewardsWithdraw => "155",
    FeeRebate => "156",
    CollectedCrypto => "157",
    DualInvestmentSubscribe => "160",
    DualInvestmentCollection => "161",
    DualInvestmentProfit => "162",
    DualInvestmentRefund => "163",
    NewYearRewards => "169",
    SubAffiliateCommission => "172",
    Pay => "174",
    LockedCollateral => "175",
    Loan => "176",
    AddedCollateral => "177",
    ReturnedCollateral => "178",
    Repayment => "179",
    UnlockedCollateral => "180",
    AirdropPayment => "181",
    FeedbackBounty => "182",
    InviteFriendsRewards => "183",
    DivideTheRewardPool => "184",
    BrokerConvertReward => "185",
    FreeETH => "186",
    ConvertTransfer => "187",
    SlotAuctionConversion => "188",
    MysteryBoxBonus => "189",
    CardPaymentBuy => "193",
    UntradableAssetWithdrawal => "195",
    UntradableAssetWithdrawalRevoked => "196",
    UntradableAssetDeposit => "197",
    UntradableAssetCollectionReduce => "198",
    UntradableAssetCollectionIncrease => "199",
    Buy => "200",
    PriceLockSubscribe => "202",
    PriceLockCollection => "203",
    PriceLockProfit => "204",
    PriceLockRefund => "205",
    DualInvestmentLiteSubscribe => "207",
    DualInvestmentLiteCollection => "208",
    DualInvestmentLiteProfit => "209",
    DualInvestmentLiteRefund => "210",
    WinCryptoWithSatoshi => "211",
    MultiCollateralLoanCollateralLocked => "212",
    CollateralTransferedOutFromUserAccount => "213",
    CollateralReturnedToUsers => "214",
    MultiCollateralLoanCollateralReleased => "215",
    LoanTransferredToUsersAccount => "216",
    MultiCollateralLoanBorrowed => "217",
    MultiCollateralLoanRepaid => "218",
    RepaymentTransferredFromUsersAccount => "219",
    DelistedCrypto => "220",
    BlockchainsWithdrawalFee => "221",
    WithdrawalFeeRefund => "222",
    CopyTradingProfitShare => "223",
    ServiceFee => "224",
    SharkFinSubscribe => "225",
    SharkFinCollection => "226",
    SharkFinProfit => "227",
    SharkFinRefund => "228",
    Airdrop => "229",
    TokenMigrationCompleted => "230",
    SubsidizedInterestReceived => "232",
    BrokerRebateCompensation => "233",
    StrategyBotsProfitShare => "263",
    DCDBrokerTransfer => "270",
    DCDBrokerRebate => "271",
    TransferOutTradingSubAccount => "284",
    TransferInTradingSubAccount => "285",
    TransferOutCustodyFundingAccount => "286",
    TransferInCustodyFundingAccount => "287",
    CustodyFundDelegation => "288",
    CustodyFundUndelegation => "289",
);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetBill {
    /// Bill ID
    #[serde(default)]
    pub bill_id: MaybeString,
    /// Account balance currency
    #[serde(default)]
    pub ccy: MaybeString,
    /// Client-supplied ID for transfer or withdrawal
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub client_id: Option<f64>,
    /// Change in balance at the account level
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub bal_chg: Option<f64>,
    /// Balance at the account level
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub bal: Option<f64>,
    /// Bill type
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub r#type: Option<AssetBillType>,
    /// Creation time, Unix timestamp format in milliseconds, e.g.1597026383085
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ts: Option<u64>,
}

// gen test for get_asset_bills
#[cfg(test)]
mod tests_get_asset_bills {
    use crate::api::v5::funding_account::GetAssetBills;
    use crate::api::v5::testkit::with_env_private_client;

    #[tokio::test]
    #[ignore]
    async fn test_deser() {
        with_env_private_client(|client| async move {
            let resp = client
                .request(GetAssetBills::default())
                .await
                .expect("get asset bills");
            println!("{:?}", resp);
            assert!(!resp.is_empty());
        })
        .await;
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountBillType {
    InterestDeduction, // 7
    FundingFee,        // 8
    Other(String),
}
impl_string_enum!(AccountBillType,
    Other,
    InterestDeduction => "7",
    FundingFee => "8",
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountBillSubType {
    InterestDeductionForMarketLoans, // 9
    FundingFeeExpense,               // 173
    FundingFeeIncome,                // 174
    Other(String),
}

impl_string_enum!(AccountBillSubType,
    Other,
    InterestDeductionForMarketLoans => "9",
    FundingFeeExpense => "173",
    FundingFeeIncome => "174",
);
