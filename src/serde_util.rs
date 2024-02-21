use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
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
            let time_ms = i64::from_str(s)
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
                    other => { return std::result::Result::Ok(Self::$wildcard(other.into())) }
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
                    // dynamic string is not feasible for const fn 'static str
                    Self::$wildcard(_) => "unhandled_const_str"
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
        use std::fmt::Debug;
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

// serde util for time
#[derive(Debug)]
pub struct Timestamp(DateTime<Utc>);

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // TODO: detect microsecond timestmap
        let time_ms = i64::from_str(&s)
            .map_err(|err| D::Error::custom(format!("invalid time_ms {s}. {err}")))?;
        let ndt = NaiveDateTime::from_timestamp_millis(time_ms).unwrap();
        Ok(Timestamp(ndt.and_local_timezone(Utc).unwrap()))
    }
}

impl AsRef<DateTime<Utc>> for Timestamp {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

// generate test for test deser Timestamp from json
#[cfg(test)]
mod tests_timestamp {
    use super::Timestamp;
    use chrono::DateTime;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Foo {
        bar: Timestamp,
    }

    #[test]
    fn test_deser_timestamp() {
        let s = r#"{
            "bar": "1610000000000"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(
            m.bar.as_ref(),
            &DateTime::parse_from_rfc3339("2021-01-07T06:13:20Z").unwrap()
        );
    }
}

pub(crate) mod serde_float {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Number(f64),
            Null(()),
        }
        let s = StringOrFloat::deserialize(deserializer)?;
        match s {
            StringOrFloat::String(s) => match s.as_str() {
                s => s.parse().map_err(D::Error::custom),
            },
            StringOrFloat::Number(n) => Ok(n),
            StringOrFloat::Null(_) => Err(D::Error::custom("null is not a valid number")),
        }
    }

    pub fn serialize<S>(f: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&f.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatOpt(Option<f64>);
impl AsRef<Option<f64>> for FloatOpt {
    fn as_ref(&self) -> &Option<f64> {
        &self.0
    }
}

impl Deref for FloatOpt {
    type Target = Option<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FloatOpt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Option<f64>> for FloatOpt {
    fn from(opt: Option<f64>) -> Self {
        FloatOpt(opt)
    }
}

impl From<f64> for FloatOpt {
    fn from(f: f64) -> Self {
        FloatOpt(Some(f))
    }
}

impl<'de> Deserialize<'de> for FloatOpt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_float_opt::deserialize(deserializer)
            .map_err(D::Error::custom)
            .map(Into::into)
    }
}

impl Serialize for FloatOpt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde_float_opt::serialize(self, serializer)
    }
}

impl Default for FloatOpt {
    fn default() -> Self {
        FloatOpt(None)
    }
}

pub(crate) mod serde_float_opt {
    use crate::serde_util::FloatOpt;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FloatOpt, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Number(f64),
            Null(()),
        }
        let s = StringOrFloat::deserialize(deserializer)?;
        match s {
            StringOrFloat::String(s) => match s.as_str() {
                "" => Ok(None.into()),
                s => s
                    .parse()
                    .map_err(D::Error::custom)
                    .map(Some)
                    .map(Into::into),
            },
            StringOrFloat::Number(n) => Ok(Some(n).into()),
            StringOrFloat::Null(()) => Ok(None.into()),
        }
    }

    pub fn serialize<S>(f: &FloatOpt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match f.as_ref() {
            Some(f) => serializer.serialize_str(&f.to_string()),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests_parse_float {
    use super::{serde_float, FloatOpt};
    use serde::Deserialize;

    #[test]
    fn can_deser_float() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(with = "serde_float")]
            bar: f64,
        }

        let s = r#"{
            "bar": "1.23"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(m.bar, 1.23);
    }

    #[test]
    fn can_deser_float_with_exp() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(with = "serde_float")]
            bar: f64,
        }

        let s = r#"{
            "bar": "1.23e-3"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(m.bar, 1.23e-3);
    }

    #[test]
    fn can_deser_float_opt() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(default)]
            bar: FloatOpt,
        }
        let s = r#"{
            "bar": "1.23"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, Some(1.23));
    }

    #[test]
    fn can_deser_float_opt_empty() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(default)]
            bar: FloatOpt,
        }
        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn can_deser_float_opt_null() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(default)]
            bar: FloatOpt,
        }
        let s = r#"{
            "bar": null
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn can_deser_float_opt_missing_key() {
        #[derive(Debug, Deserialize, Default)]
        struct Foo {
            #[serde(default)]
            bar: FloatOpt,
        }
        let s = r#"{ }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }
}
