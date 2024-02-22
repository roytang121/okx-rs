use crate::api::error::{ApiError, Error};
use crate::api::v5::orderbook_trading::orders::websocket::OrdersChannel;
use crate::api::v5::{AccountChannel, BalanceAndPositionChannel, OrderOp, PositionsChannel};
use crate::{
    api::v5::Instruments,
    api::v5::MarkPrices,
    api::v5::WsResponse,
    websocket::conn::{BboTbt, Books, Books5, BooksL2Tbt},
    websocket::WebsocketChannel,
};
use const_format::concatcp;
use serde::Deserialize;
use std::fmt::Debug;

fn deser_from_str<'a, T>(s: &'a str) -> serde_json::Result<T>
where
    T: Deserialize<'a>,
{
    serde_json::from_str(s)
}

#[cfg(feature = "simd")]
fn deser_from_str_simd<'a, T>(s: &'a mut str) -> simd_json::Result<T>
where
    T: Deserialize<'a>,
{
    simd_json::from_slice(unsafe { s.as_bytes_mut() })
}

macro_rules! impl_channel_match {
    ($channel:ident) => {
        impl ChannelMatch for $channel {
            const CHANNEL_PATTERN: &'static str =
                concatcp!(r#""channel":""#, $channel::CHANNEL, r#"""#);
        }
    };
}
impl_channel_match!(Instruments);
impl_channel_match!(MarkPrices);
impl_channel_match!(Books);
impl_channel_match!(BooksL2Tbt);
impl_channel_match!(Books5);
impl_channel_match!(BboTbt);
impl_channel_match!(PositionsChannel);
impl_channel_match!(AccountChannel);
impl_channel_match!(BalanceAndPositionChannel);
impl_channel_match!(OrdersChannel);

impl ChannelMatch for OrderOp {
    const CHANNEL_PATTERN: &'static str = r#""op":"order""#;
}

#[cfg(test)]
mod test_channel_match {
    use crate::api::v5::ws_convert::ChannelMatch;
    use crate::api::v5::Instruments;
    use crate::websocket::conn::Books;

    #[test]
    fn test_channel_match_1() {
        assert_eq!(Books::CHANNEL_PATTERN, r#""channel":"books""#);
        assert_eq!(Instruments::CHANNEL_PATTERN, r#""channel":"instruments""#);
    }
}

trait ChannelMatch {
    const CHANNEL_PATTERN: &'static str;
}

pub trait TryParseEvent {
    type Value<'a>: Debug;
    type ErrorData: Debug;

    fn try_parse(msg: &str) -> Result<Option<Self::Value<'_>>, Error<Self::ErrorData>>;
}
impl<T> TryParseEvent for T
where
    T: WebsocketChannel + ChannelMatch,
{
    // type Value<'a> = <T as WebsocketChannel>::Response<'a>;
    type Value<'a> =
        WsResponse<'a, <T as WebsocketChannel>::ArgType<'a>, <T as WebsocketChannel>::Response<'a>>;
    type ErrorData = ();

    fn try_parse(msg: &str) -> Result<Option<Self::Value<'_>>, Error<Self::ErrorData>> {
        if msg.contains(T::CHANNEL_PATTERN) {
            let response: Self::Value<'_> = match deser_from_str(msg) {
                Ok(Some(res)) => res,
                Ok(None) => return Ok(None),
                Err(err) => {
                    log::error!("{}", msg);
                    log::error!("{:?}", err);
                    let highlight = &msg.as_bytes()[err.column() - 20..err.column() + 20];
                    log::error!(".. {:?} ..", String::from_utf8(highlight.to_vec()).unwrap());
                    return Err(err.into());
                }
            };
            if response.event == Some("error") {
                log::error!("{:?}", response);
                let WsResponse {
                    code, conn_id, msg, ..
                } = response;
                return Err(Error::Api(ApiError {
                    code,
                    msg: msg.to_owned().map(str::to_string),
                    data: Some(()),
                    conn_id: conn_id.to_owned().map(str::to_string),
                }));
            } else if response.event == Some("subscribe") || response.event == Some("unsubscribe") {
                log::info!("{:?}", response);
                // TODO: propagate subscribe/unsubscribe event
                return Ok(None);
            }
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

impl Books {
    #[cfg(not(feature = "simd"))]
    pub fn try_parse_books<'a>(
        msg: &'a str,
    ) -> Option<
        WsResponse<
            'a,
            <Self as WebsocketChannel>::ArgType<'a>,
            <Self as WebsocketChannel>::Response<'a>,
        >,
    > {
        if msg.contains(Books::CHANNEL_PATTERN) || msg.contains(BboTbt::CHANNEL_PATTERN) {
            let response: WsResponse<
                <Self as WebsocketChannel>::ArgType<'_>,
                <Self as WebsocketChannel>::Response<'_>,
            > = deser_from_str(msg).unwrap();
            if response.event == Some("error") {
                log::error!("{:?}", response);
                return None;
            } else if response.event == Some("subscribe") || response.event == Some("unsubscribe") {
                log::info!("{:?}", response);
                return None;
            }
            Some(response)
        } else {
            None
        }
    }

    #[cfg(feature = "simd")]
    pub fn try_parse_books<'a>(
        msg: &'a mut str,
    ) -> Option<
        WsResponse<
            'a,
            <Self as WebsocketChannel>::ArgType<'a>,
            <Self as WebsocketChannel>::Response<'a>,
        >,
    > {
        if msg.contains(Books::CHANNEL_PATTERN) || msg.contains(BboTbt::CHANNEL_PATTERN) {
            let response: WsResponse<
                <Self as WebsocketChannel>::ArgType<'_>,
                <Self as WebsocketChannel>::Response<'_>,
            > = deser_from_str_simd(msg).unwrap();
            if response.event == Some("error") {
                log::error!("{:?}", response);
                return None;
            } else if response.event == Some("subscribe") || response.event == Some("unsubscribe") {
                log::info!("{:?}", response);
                return None;
            }
            Some(response)
        } else {
            None
        }
    }
}
