//! Tolerant numeric (de)serialization.
//!
//! Some APIs serialize numbers as JSON strings to avoid precision loss in
//! clients with 2^53-bounded integers. These adapters accept either a JSON
//! number or a JSON string, and always serialize back as a JSON number.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Either JSON representation a number may arrive in.
#[derive(Deserialize)]
#[serde(untagged)]
enum NumericRepr {
    Float(f64),
    Int(i64),
    Text(String),
}

impl NumericRepr {
    fn into_f64(self) -> Result<f64, String> {
        match self {
            Self::Float(value) => Ok(value),
            Self::Int(value) => Ok(value as f64),
            Self::Text(text) => text
                .trim()
                .parse::<f64>()
                .map_err(|err| format!("`{}` is not a number: {}", text, err)),
        }
    }

    fn into_i64(self) -> Result<i64, String> {
        match self {
            Self::Int(value) => Ok(value),
            // Reject a fractional float rather than truncating it: silently
            // dropping the fraction would corrupt the value.
            Self::Float(value) if value.fract() == 0.0 => Ok(value as i64),
            Self::Float(value) => Err(format!("`{}` is not an integer", value)),
            Self::Text(text) => text
                .trim()
                .parse::<i64>()
                .map_err(|err| format!("`{}` is not an integer: {}", text, err)),
        }
    }
}

/// `#[serde(with = "...::float")]` for a required `f64`.
pub mod float {
    use super::*;

    /// Serializes a whole-number float without a trailing decimal point: `24000.0` -> `24000`.
    ///
    /// Some APIs reject the decimal form for a value declared as a number, so this matches the
    /// numeric form the API expects. A value outside the `i64` range stays a float, because the
    /// cast would saturate and change it. `i64::MAX as f64` rounds up to 2^63, so the upper
    /// comparison is strict.
    pub fn serialize<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
        if value.fract() == 0.0
            && value.is_finite()
            && *value >= (i64::MIN as f64)
            && *value < (i64::MAX as f64)
        {
            (*value as i64).serialize(serializer)
        } else {
            value.serialize(serializer)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        NumericRepr::deserialize(deserializer)?
            .into_f64()
            .map_err(serde::de::Error::custom)
    }

    /// `#[serde(with = "...::float::option")]` for `Option<f64>`.
    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer>(
            value: &Option<f64>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(inner) => super::serialize(inner, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<f64>, D::Error> {
            match Option::<NumericRepr>::deserialize(deserializer)? {
                Some(repr) => repr.into_f64().map(Some).map_err(serde::de::Error::custom),
                None => Ok(None),
            }
        }
    }
}

/// `#[serde(with = "...::int")]` for a required `i64`.
pub mod int {
    use super::*;

    pub fn serialize<S: Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
        NumericRepr::deserialize(deserializer)?
            .into_i64()
            .map_err(serde::de::Error::custom)
    }

    /// `#[serde(with = "...::int::option")]` for `Option<i64>`.
    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer>(
            value: &Option<i64>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(inner) => inner.serialize(serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<i64>, D::Error> {
            match Option::<NumericRepr>::deserialize(deserializer)? {
                Some(repr) => repr.into_i64().map(Some).map_err(serde::de::Error::custom),
                None => Ok(None),
            }
        }
    }
}
