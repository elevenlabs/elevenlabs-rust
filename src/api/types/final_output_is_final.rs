pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FinalOutputIsFinal {
    #[default]
    True_,
}

impl Serialize for FinalOutputIsFinal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::True_ => serializer.serialize_bool(true),
        }
    }
}

impl<'de> Deserialize<'de> for FinalOutputIsFinal {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <bool as serde::Deserialize>::deserialize(deserializer)?;
        if value {
            return Ok(Self::True_);
        }
        Err(serde::de::Error::custom(format!(
            "unknown FinalOutputIsFinal value: {value}"
        )))
    }
}

impl fmt::Display for FinalOutputIsFinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True_ => write!(f, "true"),
        }
    }
}
