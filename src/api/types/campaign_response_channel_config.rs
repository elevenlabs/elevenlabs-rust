pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "channel")]
#[non_exhaustive]
pub enum CampaignResponseChannelConfig {
    #[serde(rename = "phone")]
    Phone {
        #[serde(flatten)]
        data: PhoneChannelConfigOutput,
    },

    #[serde(rename = "whatsapp")]
    Whatsapp {
        #[serde(flatten)]
        data: WhatsAppChannelConfig,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CampaignResponseChannelConfig {
    pub fn is_phone(&self) -> bool {
        matches!(self, Self::Phone { .. })
    }

    pub fn as_phone(&self) -> Option<&PhoneChannelConfigOutput> {
        match self {
            Self::Phone { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone(self) -> Option<PhoneChannelConfigOutput> {
        match self {
            Self::Phone { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_whatsapp(&self) -> bool {
        matches!(self, Self::Whatsapp { .. })
    }

    pub fn as_whatsapp(&self) -> Option<&WhatsAppChannelConfig> {
        match self {
            Self::Whatsapp { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_whatsapp(self) -> Option<WhatsAppChannelConfig> {
        match self {
            Self::Whatsapp { data } => Some(data),
            _ => None,
        }
    }
}
