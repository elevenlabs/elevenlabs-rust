pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScribeSessionTimeLimitExceededErrorMessageType {
    SessionTimeLimitExceeded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScribeSessionTimeLimitExceededErrorMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SessionTimeLimitExceeded => {
                serializer.serialize_str("session_time_limit_exceeded")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScribeSessionTimeLimitExceededErrorMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "session_time_limit_exceeded" => Ok(Self::SessionTimeLimitExceeded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScribeSessionTimeLimitExceededErrorMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionTimeLimitExceeded => write!(f, "session_time_limit_exceeded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
