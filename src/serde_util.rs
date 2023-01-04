use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::Error;
use serde::{de, Deserialize, Deserializer, Serializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn deserialize_from_opt_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "" => Ok(None),
        s => FromStr::from_str(s)
            .map_err(de::Error::custom)
            .map(Option::Some),
    }
}

pub fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let time_ms =
        i64::from_str(&s).map_err(|err| D::Error::custom(format!("invalid time_ms {s}. {err}")))?;
    let ndt = NaiveDateTime::from_timestamp_millis(time_ms).unwrap();
    Ok(ndt.and_local_timezone(Utc).unwrap())
}

pub fn deserialize_timestamp_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "" => Ok(None),
        s => {
            let time_ms = i64::from_str(&s)
                .map_err(|err| D::Error::custom(format!("invalid time_ms {s}. {err}")))?;
            let ndt = NaiveDateTime::from_timestamp_millis(time_ms).unwrap();
            Ok(Some(ndt.and_local_timezone(Utc).unwrap()))
        }
    }
}

pub fn serialize_timestamp<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.unwrap().timestamp_millis().to_string())
}

#[macro_export]
macro_rules! impl_serde_from_str {
    ($name:ident) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let data = <&str>::deserialize(deserializer)?;
                <$name as std::str::FromStr>::from_str(data).map_err(serde::de::Error::custom)
            }
        }

        impl serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
}

pub fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    FromStr::from_str(&s).map_err(de::Error::custom)
}

pub fn serialize_as_str_opt<S, T>(dt: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    use serde::ser::Error;

    if let Some(dt) = dt {
        serializer.serialize_str(&dt.to_string())
    } else {
        Err(S::Error::custom("Empty option"))
    }
}

pub fn serialize_as_str<S, T>(dt: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    serializer.serialize_str(&dt.to_string())
}
