use std::ops::Deref;
use std::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(Decimal);

impl Deref for Fixed {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Decimal> for Fixed {
    fn as_ref(&self) -> &Decimal {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Fixed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct FixedVisitor;
        impl<'de> serde::de::Visitor<'de> for FixedVisitor {
            type Value = Fixed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("valid decimal")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                let dec = Decimal::from_str(&s)
                    .map_err(|err| E::custom(format!("invalid decimal {s}. {err}")))?;
                Ok(Fixed(dec))
            }
        }
        deserializer.deserialize_str(FixedVisitor)
    }
}

#[cfg(test)]
mod decimal_tests {
    use super::*;

    #[test]
    fn test_deser_str() {
        let dec: Fixed = serde_json::from_str("0.00000001").unwrap();
        assert_eq!(dec, Fixed(Decimal::new(1, 8)));
    }

    #[test]
    fn test_deser_opt_str() {
        let dec: Option<Fixed> = serde_json::from_str("0.00000001").unwrap();
        assert_eq!(dec, Some(Fixed(Decimal::new(1, 8))));
    }

    #[test]
    fn test_deser_empty_str() {
        let dec: Fixed = serde_json::from_str("").unwrap();
        assert_eq!(dec, Fixed(Decimal::new(0, 0)));
    }
}
