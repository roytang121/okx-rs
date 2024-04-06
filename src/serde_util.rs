use serde::de::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub fn deserialize_from_opt_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    match StringOrFloat::deserialize(deserializer)? {
        StringOrFloat::Str(s) => match s {
            "" => Ok(None),
            s => FromStr::from_str(s)
                .map_err(de::Error::custom)
                .map(Option::Some),
        },
        _ => Ok(None),
    }
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
    // serializer.serialize_str(&dt.to_string())
    serializer.collect_str(dt)
}

pub fn serialize_as_str_opt<S, T>(dt: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    match dt {
        Some(dt) => serializer.collect_str(dt),
        None => serializer.serialize_none(),
    }
}

pub mod str_opt {
    use super::{deserialize_from_opt_str, serialize_as_str_opt};
    use serde::{Deserializer, Serializer};
    use std::fmt::Display;
    use std::str::FromStr;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        deserialize_from_opt_str(deserializer)
    }

    pub fn serialize<S, T>(f: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serialize_as_str_opt(f, serializer)
    }
}

#[allow(dead_code)]
pub const fn none<T>() -> Option<T> {
    None
}
#[cfg(test)]
mod tests_maybe_string {

    use serde::Deserialize;

    #[test]
    fn test_deser_empty_str() {
        use super::*;
        use std::fmt::Debug;
        #[derive(Debug, Deserialize)]
        struct Foo {
            #[serde(default, deserialize_with = "deserialize_from_opt_str")]
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
            #[serde(default, deserialize_with = "deserialize_from_opt_str")]
            bar: Option<f64>,
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
        assert_eq!(m.bar.unwrap(), 100.0);

        let s = r#"{
            "bar": null
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());
    }
}

#[allow(dead_code)]
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
            StringOrFloat::String(s) => {
                let s = s.as_str();
                s.parse().map_err(D::Error::custom)
            }
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

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct FloatOpt(Option<f64>);
pub type MaybeFloat = Option<f64>;
pub type MaybeU64 = Option<u64>;
pub type MaybeI64 = Option<i64>;
pub type MaybeString = Option<String>;

#[derive(Clone)]
pub struct Maybe<T>(pub Option<T>);

impl<T: std::fmt::Debug> std::fmt::Debug for Maybe<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(v) => write!(f, "Maybe({:?})", v),
            None => write!(f, "None"),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Maybe<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(v) => write!(f, "{}", v),
            None => write!(f, ""),
        }
    }
}

impl<T> Default for Maybe<T> {
    fn default() -> Self {
        Maybe(None)
    }
}

impl<T> Deref for Maybe<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Maybe<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<Option<T>> for Maybe<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(val: Maybe<T>) -> Self {
        val.0
    }
}

impl<T> Maybe<T> {
    pub fn into_option(self) -> Option<T> {
        self.into()
    }
}

impl<T> Serialize for Maybe<T>
where
    T: std::fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Maybe<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Maybe<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::Str(s) => match s {
                "" => Ok(Maybe(None)),
                s => Ok(Maybe(Some(s.parse().map_err(D::Error::custom)?))),
            },
            StringOrFloat::Bool(_) => Ok(Maybe(None)),
            StringOrFloat::Float(v) => Ok(Maybe(Some(v))),
            StringOrFloat::Integer(v) => Ok(Maybe(Some(v as f64))),
            StringOrFloat::Null(()) => Ok(Maybe(None)),
        }
    }
}

impl<'de> Deserialize<'de> for Maybe<u64> {
    fn deserialize<D>(deserializer: D) -> Result<Maybe<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::Str(s) => match s {
                "" => Ok(Maybe(None)),
                s => Ok(Maybe(Some(s.parse().map_err(D::Error::custom)?))),
            },
            StringOrFloat::Bool(_) => Ok(Maybe(None)),
            StringOrFloat::Float(v) => Ok(Maybe(Some(v as u64))),
            StringOrFloat::Integer(v) => Ok(Maybe(Some(v as u64))),
            StringOrFloat::Null(()) => Ok(Maybe(None)),
        }
    }
}

impl<'de> Deserialize<'de> for Maybe<i64> {
    fn deserialize<D>(deserializer: D) -> Result<Maybe<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::Str(s) => match s {
                "" => Ok(Maybe(None)),
                s => Ok(Maybe(Some(s.parse().map_err(D::Error::custom)?))),
            },
            StringOrFloat::Bool(_) => Ok(Maybe(None)),
            StringOrFloat::Float(v) => Ok(Maybe(Some(v as i64))),
            StringOrFloat::Integer(v) => Ok(Maybe(Some(v))),
            StringOrFloat::Null(()) => Ok(Maybe(None)),
        }
    }
}

impl<'de> Deserialize<'de> for Maybe<String> {
    fn deserialize<D>(deserializer: D) -> Result<Maybe<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::Str(s) => match s {
                "" => Ok(Maybe(None)),
                s => Ok(Maybe(Some(s.to_string()))),
            },
            StringOrFloat::Bool(v) => Ok(Maybe(Some(v.to_string()))),
            StringOrFloat::Float(v) => Ok(Maybe(Some(v.to_string()))),
            StringOrFloat::Integer(v) => Ok(Maybe(Some(v.to_string()))),
            StringOrFloat::Null(()) => Ok(Maybe(None)),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrFloat<'a> {
    Str(&'a str),
    Float(f64),
    Integer(i64),
    Bool(bool),
    Null(()),
}

#[cfg(test)]
mod tests_maybe_float {
    use super::{str_opt, Maybe, MaybeFloat};
    use serde::{Deserialize, Serialize};

    #[test]
    fn can_deser_maybe_float() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bar: Maybe<f64>,
        }

        let s = r#"{
            "bar": "1.23"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, Some(1.23));
    }

    #[test]
    fn can_deser_maybe_float_empty() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bar: Maybe<f64>,
        }
        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn can_deser_maybe_float_null() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bar: Maybe<f64>,
        }
        let s = r#"{
            "bar": null
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn can_deser_maybe_float_missing_key() {
        #[derive(Debug, Deserialize, Default)]
        struct Foo {
            #[serde(default)]
            bar: Maybe<f64>,
        }
        let s = r#"{ }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn test_ser_maybe_float() {
        #[derive(Debug, Serialize)]
        struct Foo {
            #[serde(default, with = "str_opt")]
            bar: MaybeFloat,
        }
        let f = Foo {
            bar: *Maybe(Some(1.23)),
        };
        let s = serde_json::to_string(&f).unwrap();
        assert_eq!(s, r#"{"bar":"1.23"}"#);

        let f = Foo { bar: *Maybe(None) };
        let s = serde_json::to_string(&f).unwrap();
        assert_eq!(s, r#"{"bar":null}"#);
    }
}

#[cfg(test)]
mod tests_maybe_u64 {
    use super::Maybe;
    use serde::Deserialize;

    #[test]
    fn can_deser_maybe_u64() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bar: Maybe<u64>,
        }

        let s = r#"{
            "bar": "123"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, Some(123));
    }

    #[test]
    fn can_deser_maybe_u64_empty() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bar: Maybe<u64>,
        }
        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn can_deser_maybe_u64_null() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bar: Maybe<u64>,
        }
        let s = r#"{
            "bar": null
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }

    #[test]
    fn can_deser_maybe_u64_missing_key() {
        #[derive(Debug, Deserialize, Default)]
        struct Foo {
            #[serde(default)]
            bar: Maybe<u64>,
        }
        let s = r#"{ }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(*m.bar, None);
    }
}

#[cfg(test)]
mod test_parse_maybe_enum {
    use super::*;
    use serde::Deserialize;

    enum Bar {
        Baz,
        Other(Box<str>),
    }
    impl<'de> Deserialize<'de> for Bar {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "baz" => Ok(Bar::Baz),
                other => Ok(Bar::Other(other.into())),
            }
        }
    }
    #[derive(Deserialize)]
    struct Foo {
        bar: Option<Bar>,
    }

    #[test]
    fn test_deser_maybe_string() {
        let s = r#"{
            "bar": "baz"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(matches!(m.bar, Some(Bar::Baz)));

        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        let s = Box::new("");
        assert!(matches!(m.bar, Some(Bar::Other(_))));
        match m.bar.unwrap() {
            Bar::Other(other) => assert_eq!(&*other, *s),
            _ => panic!("should be other"),
        }

        let s = r#"{
            "bar": null
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());

        let s = r#"{ }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert!(m.bar.is_none());
    }
}

#[cfg(test)]
mod test_serialise_fields_as_str {
    use super::str_opt;
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, skip_serializing_none};

    #[serde_as]
    #[skip_serializing_none]
    #[derive(Serialize, Deserialize)]
    struct Foo {
        #[serde(default, with = "str_opt")]
        bar: Option<f64>,
    }

    #[test]
    fn test_ser_fields_as_str() {
        let f = Foo { bar: Some(1.23) };
        let s = serde_json::to_string(&f).unwrap();
        assert_eq!(s, r#"{"bar":"1.23"}"#);

        let f = Foo { bar: None };
        let s = serde_json::to_string(&f).unwrap();
        assert_eq!(s, r#"{}"#);
    }

    #[test]
    fn test_deser_fields_from_str() {
        let s = r#"{
            "bar": "1.23"
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(m.bar, Some(1.23));

        let s = r#"{
            "bar": ""
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(m.bar, None);

        let s = r#"{
            "bar": null
        }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(m.bar, None);

        let s = r#"{ }"#;
        let m = serde_json::from_str::<Foo>(s).unwrap();
        assert_eq!(m.bar, None);
    }
}
