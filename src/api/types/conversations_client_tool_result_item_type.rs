pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientToolResultItemType {
    ClientToolResult,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClientToolResultItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ClientToolResult => serializer.serialize_str("client_tool_result"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClientToolResultItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "client_tool_result" => Ok(Self::ClientToolResult),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClientToolResultItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientToolResult => write!(f, "client_tool_result"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
