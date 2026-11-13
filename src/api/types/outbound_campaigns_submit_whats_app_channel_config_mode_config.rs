pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode")]
#[non_exhaustive]
pub enum SubmitWhatsAppChannelConfigModeConfig {
    #[serde(rename = "message")]
    Message {
        #[serde(flatten)]
        data: SubmitWhatsAppMessageConfig,
    },

    #[serde(rename = "voice")]
    Voice {
        #[serde(flatten)]
        data: WhatsAppVoiceConfig,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SubmitWhatsAppChannelConfigModeConfig {
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message { .. })
    }

    pub fn as_message(&self) -> Option<&SubmitWhatsAppMessageConfig> {
        match self {
            Self::Message { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_message(self) -> Option<SubmitWhatsAppMessageConfig> {
        match self {
            Self::Message { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_voice(&self) -> bool {
        matches!(self, Self::Voice { .. })
    }

    pub fn as_voice(&self) -> Option<&WhatsAppVoiceConfig> {
        match self {
            Self::Voice { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_voice(self) -> Option<WhatsAppVoiceConfig> {
        match self {
            Self::Voice { data } => Some(data),
            _ => None,
        }
    }
}
