pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecretDependencyType {
    ConversationInitiationWebhook,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SecretDependencyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConversationInitiationWebhook => {
                serializer.serialize_str("conversation_initiation_webhook")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SecretDependencyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation_initiation_webhook" => Ok(Self::ConversationInitiationWebhook),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SecretDependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationInitiationWebhook => write!(f, "conversation_initiation_webhook"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
