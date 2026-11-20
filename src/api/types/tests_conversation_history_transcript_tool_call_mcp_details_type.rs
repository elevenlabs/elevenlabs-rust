pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationHistoryTranscriptToolCallMcpDetailsType {
    Mcp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationHistoryTranscriptToolCallMcpDetailsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationHistoryTranscriptToolCallMcpDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mcp" => Ok(Self::Mcp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationHistoryTranscriptToolCallMcpDetailsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mcp => write!(f, "mcp"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
