pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextualUpdateInputItemType {
    ContextualUpdate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ContextualUpdateInputItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ContextualUpdate => serializer.serialize_str("contextual_update"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ContextualUpdateInputItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "contextual_update" => Ok(Self::ContextualUpdate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ContextualUpdateInputItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContextualUpdate => write!(f, "contextual_update"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
