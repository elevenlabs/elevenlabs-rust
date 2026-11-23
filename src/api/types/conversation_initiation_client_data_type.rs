pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationInitiationClientDataType {
    ConversationInitiationClientData,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationInitiationClientDataType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConversationInitiationClientData => {
                serializer.serialize_str("conversation_initiation_client_data")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationInitiationClientDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation_initiation_client_data" => Ok(Self::ConversationInitiationClientData),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationInitiationClientDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationInitiationClientData => {
                write!(f, "conversation_initiation_client_data")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
