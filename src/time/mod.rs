use ::chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use std::str::FromStr;

pub mod chrono {
    pub use chrono::*;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UTCDateTime(DateTime<Utc>);

pub trait TimeExt {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

impl TimeExt for UTCDateTime {}

impl UTCDateTime {
    pub fn as_dt(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Deref for UTCDateTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<DateTime<Utc>> for UTCDateTime {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl From<DateTime<Utc>> for UTCDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        UTCDateTime(value)
    }
}

impl<'de> Deserialize<'de> for UTCDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UTCDateTimeVisitor;
        impl<'de> serde::de::Visitor<'de> for UTCDateTimeVisitor {
            type Value = UTCDateTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("valid unix timestamp")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let time_ms = i64::from_str(&s)
                    .map_err(|err| E::custom(format!("invalid time_ms {s}. {err}")))?;
                let ndt = NaiveDateTime::from_timestamp_millis(time_ms).unwrap();
                Ok(ndt.and_local_timezone(Utc).unwrap().into())
            }
        }

        deserializer.deserialize_str(UTCDateTimeVisitor)
    }
}

impl Serialize for UTCDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.timestamp_millis().to_string())
    }
}

#[cfg(test)]
mod utcdatetime_tests {
    use crate::time::UTCDateTime;
    use chrono::{TimeZone, Utc};
    use serde::Deserialize;

    #[test]
    fn test_deser_milli_str() {
        #[derive(Deserialize)]
        struct Foo {
            bar: UTCDateTime,
        }

        let json_str = r#"{
            "bar": "1609459200000"
        }"#;
        let foo = serde_json::from_str::<Foo>(json_str).unwrap();
        assert_eq!(foo.bar.timestamp_millis(), 1609459200000);
    }

    #[test]
    fn test_deser_opt_str() {
        #[derive(Deserialize)]
        struct Foo {
            bar: Option<UTCDateTime>,
        }

        let json_str = r#"{
            "bar": "1609459200000"
        }"#;
        let foo = serde_json::from_str::<Foo>(json_str).unwrap();
        assert_eq!(foo.bar.unwrap().timestamp_millis(), 1609459200000);
    }

    #[test]
    fn test_ser_timestamp() {
        let dt = UTCDateTime::from(Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap());
        let json_str = serde_json::to_string(&dt).unwrap();
        assert_eq!(json_str, r#""1609459200000""#);
    }
}
