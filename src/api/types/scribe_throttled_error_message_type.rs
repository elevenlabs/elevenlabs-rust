pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScribeThrottledErrorMessageType {
    CommitThrottled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScribeThrottledErrorMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CommitThrottled => serializer.serialize_str("commit_throttled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScribeThrottledErrorMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "commit_throttled" => Ok(Self::CommitThrottled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScribeThrottledErrorMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommitThrottled => write!(f, "commit_throttled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
