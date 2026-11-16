pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InteractionBudget {
    Realtime,
    Num5Minutes,
    Num10Minutes,
    Num1Hour,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InteractionBudget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Realtime => serializer.serialize_str("realtime"),
            Self::Num5Minutes => serializer.serialize_str("5_minutes"),
            Self::Num10Minutes => serializer.serialize_str("10_minutes"),
            Self::Num1Hour => serializer.serialize_str("1_hour"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InteractionBudget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "realtime" => Ok(Self::Realtime),
            "5_minutes" => Ok(Self::Num5Minutes),
            "10_minutes" => Ok(Self::Num10Minutes),
            "1_hour" => Ok(Self::Num1Hour),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InteractionBudget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Realtime => write!(f, "realtime"),
            Self::Num5Minutes => write!(f, "5_minutes"),
            Self::Num10Minutes => write!(f, "10_minutes"),
            Self::Num1Hour => write!(f, "1_hour"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
