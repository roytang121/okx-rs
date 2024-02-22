use crate::api::v5::model::{
    InstrumentType, InterestAccrued, InterestLimitResponse, MarginMode, PositionDetail,
    TradingBalanceDetail,
};
use crate::api::v5::Request;
use crate::time::UTCDateTime;
use crate::websocket::WebsocketChannel;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::ChannelArg;

pub mod rest {
    use super::*;
    /// https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-balance
    /// ## Get balance
    /// Retrieve a list of assets (with non-zero balance), remaining balance, and available amount in the trading account.
    ///
    ///  Interest-free quota and discount rates are public data and not displayed on the account interface.
    /// Rate Limit: 10 requests per 2 seconds
    /// Rate limit rule: UserID
    /// ## HTTP Requests
    /// GET /api/v5/account/balance
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetTradingBalances {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH.
        pub ccy: Option<String>,
    }

    impl Request for GetTradingBalances {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/account/balance";
        const AUTH: bool = true;
        type Response = Vec<TradingBalanceDetail>;
    }

    /// https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-positions
    /// ## Get positions
    /// Retrieve information on your positions. When the account is in net mode, net positions will be displayed, and when the account is in long/short mode, long or short positions will be displayed. Return in reverse chronological order using ctime.
    ///
    /// Rate Limit: 10 requests per 2 seconds
    /// Rate limit rule: UserID
    /// ## HTTP Request
    /// GET /api/v5/account/positions
    #[derive(Debug, Serialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetPositions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_type: Option<InstrumentType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pos_id: Option<String>,
    }

    impl Request for GetPositions {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/account/positions";
        const AUTH: bool = true;
        type Response = Vec<PositionDetail>;
    }

    /// https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-positions-history
    /// ## Get positions history
    /// Retrieve the updated position data for the last 3 months. Return in reverse chronological order using utime.
    ///
    /// Rate Limit: 1 request per 10 seconds
    /// Rate limit rule: UserID
    /// ### HTTP Request
    /// GET /api/v5/account/positions-history
    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GetPositionsHistory {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_type: Option<InstrumentType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mgn_mode: Option<MarginMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pos_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<UTCDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before: Option<UTCDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
    }

    /// https://www.okx.com/docs-v5/en/#rest-api-account-get-interest-accrued-data
    #[derive(Debug, Serialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetInterestAccrued {
        #[serde(skip_serializing_if = "Option::is_none")]
        after: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        before: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mgn_mode: Option<MarginMode>,
    }

    impl Request for GetInterestAccrued {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/account/interest-accrued";
        const AUTH: bool = true;
        type Response = Vec<InterestAccrued>;
    }

    /// https://www.okx.com/docs-v5/en/#rest-api-account-get-borrow-interest-and-limit
    #[derive(Debug, Serialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetInterestLimits {
        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ccy: Option<String>,
    }

    impl Request for GetInterestLimits {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/account/interest-limits";
        const AUTH: bool = true;
        type Response = Vec<InterestLimitResponse>;
    }
}

pub mod websocket {
    use super::*;
    use crate::api::v5::BalanceAndPositionDetail;
    #[derive(Debug, Deserialize)]
    pub struct AccountChannel;

    impl WebsocketChannel for AccountChannel {
        const CHANNEL: &'static str = "account";
        const AUTH: bool = true;
        type Response<'de> = Vec<TradingBalanceDetail>;
        type ArgType<'de> = ChannelArg<'de>;
        fn subscribe_message(&self) -> String {
            json!({
                "op": "subscribe",
                "args": [
                    {
                        "channel": Self::CHANNEL,
                        "extraParams": "
                        {
                          \"updateInterval\": \"1\"
                        }
                    "
                    }
                ]
            })
            .to_string()
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PositionsArg<'a> {
        channel: &'a str,
        /// Instrument type
        /// MARGIN
        /// SWAP
        /// FUTURES
        /// OPTION
        /// ANY
        #[serde(skip_serializing_if = "Option::is_none")]
        inst_type: Option<InstrumentType>,
        /// Instrument family
        /// Applicable to FUTURES/SWAP/OPTION
        #[serde(borrow, skip_serializing_if = "Option::is_none")]
        inst_family: Option<&'a str>,
        #[serde(borrow, skip_serializing_if = "Option::is_none")]
        inst_id: Option<&'a str>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PositionsChannel {
        /// Instrument type
        /// MARGIN
        /// SWAP
        /// FUTURES
        /// OPTION
        /// ANY
        // #[serde(default, skip_serializing_if = "Option::is_none")]
        pub inst_type: InstrumentType,
        /// Instrument family
        /// Applicable to FUTURES/SWAP/OPTION
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub inst_family: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub inst_id: Option<String>,
    }

    impl WebsocketChannel for PositionsChannel {
        const CHANNEL: &'static str = "positions";
        const AUTH: bool = true;
        type Response<'de> = Vec<PositionDetail>;
        type ArgType<'de> = PositionsArg<'de>;

        fn subscribe_message(&self) -> String {
            json!({
                "op": "subscribe",
                "args": [
                    {
                        "channel": Self::CHANNEL,
                        "instType": self.inst_type,
                        "instId": self.inst_id,
                        "instFamily": self.inst_family,
                    }
                ]
            })
            .to_string()
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BalanceAndPositionChannel;

    impl WebsocketChannel for BalanceAndPositionChannel {
        const CHANNEL: &'static str = "balance_and_position";
        const AUTH: bool = true;
        type Response<'de> = [BalanceAndPositionDetail; 1];
        type ArgType<'de> = ChannelArg<'de>;
    }
}
