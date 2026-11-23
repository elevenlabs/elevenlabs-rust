pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueueStatusEventStatus {
    Waiting,
    Admitted,
    TimedOut,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueueStatusEventStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Waiting => serializer.serialize_str("waiting"),
            Self::Admitted => serializer.serialize_str("admitted"),
            Self::TimedOut => serializer.serialize_str("timed_out"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueueStatusEventStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "waiting" => Ok(Self::Waiting),
            "admitted" => Ok(Self::Admitted),
            "timed_out" => Ok(Self::TimedOut),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueueStatusEventStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Waiting => write!(f, "waiting"),
            Self::Admitted => write!(f, "admitted"),
            Self::TimedOut => write!(f, "timed_out"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
