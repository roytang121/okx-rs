use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::Error;
use serde::{de, Deserialize, Deserializer, Serializer};
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    FromStr::from_str(&s).map_err(de::Error::custom)
}

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

/// Deserialize a string into a `DateTime<Utc>`.
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

/// Deserialize a string into an `Option<DateTime<Utc>>`.
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

#[macro_export]
macro_rules! impl_string_enum {
    ($name:ident, $wildcard:tt, $($variant:tt => $variant_str:expr,)+) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant => write!(f, $variant_str)
                    ),*,
                    Self::$wildcard(other) => write!(f, "{}", other)
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    $(
                        $variant_str => { return std::result::Result::Ok(Self::$variant) }
                    ),*,
                    other => { return std::result::Result::Ok(Self::$wildcard(other.to_string())) }
                }
            }
        }

        impl $name {
            #[allow(dead_code)]
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $variant_str
                    ),*,
                    /// dynamic string is not feasible for const fn 'static str
                    Self::$wildcard(other) => "unhandled_const_str"
                }
            }
        }

        $crate::impl_serde_from_str!($name);
    };

    ($name:ident, $($variant:tt => $variant_str:expr,)+) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant => write!(f, $variant_str)
                    ),*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    $(
                        $variant_str => { return std::result::Result::Ok(Self::$variant) }
                    ),*
                    other => { anyhow::bail!("unknown variant {}", other) }
                }
            }
        }

        impl $name {
            #[allow(dead_code)]
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $variant_str
                    ),*
                }
            }
        }

        $crate::impl_serde_from_str!($name);
    };
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

pub const fn none<T>() -> Option<T> {
    None
}
#[cfg(test)]
mod tests_maybe_string {
    use rust_decimal::Decimal;
    use serde::Deserialize;

    #[test]
    fn test_deser_empty_str() {
        use super::*;
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
            bar: Option<String>,
        }
        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());

        let s = r#"{
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());
    }

    #[test]
    fn test_deser_empty_decimal() {
        use super::*;
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(default = "none", deserialize_with = "deserialize_from_opt_str")]
            bar: Option<Decimal>,
        }
        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());

        let s = r#"{
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());

        let s = r#"{
            "bar": "100"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_some());
        assert_eq!(m.bar.unwrap(), Decimal::new(100, 0));
    }
}