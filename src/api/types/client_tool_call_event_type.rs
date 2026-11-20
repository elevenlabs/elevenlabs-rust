pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientToolCallEventType {
    ClientToolCall,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClientToolCallEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ClientToolCall => serializer.serialize_str("client_tool_call"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClientToolCallEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "client_tool_call" => Ok(Self::ClientToolCall),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClientToolCallEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientToolCall => write!(f, "client_tool_call"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
