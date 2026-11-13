pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "conversation_initiation_source")]
#[non_exhaustive]
pub enum PlatformMetadata {
    #[serde(rename = "plain_integration")]
    PlainIntegration {
        #[serde(flatten)]
        data: PlainPlatformMetadata,
    },

    #[serde(rename = "zendesk_integration")]
    ZendeskIntegration {
        #[serde(flatten)]
        data: ZendeskPlatformMetadata,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PlatformMetadata {
    pub fn is_plain_integration(&self) -> bool {
        matches!(self, Self::PlainIntegration { .. })
    }

    pub fn as_plain_integration(&self) -> Option<&PlainPlatformMetadata> {
        match self {
            Self::PlainIntegration { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_plain_integration(self) -> Option<PlainPlatformMetadata> {
        match self {
            Self::PlainIntegration { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_zendesk_integration(&self) -> bool {
        matches!(self, Self::ZendeskIntegration { .. })
    }

    pub fn as_zendesk_integration(&self) -> Option<&ZendeskPlatformMetadata> {
        match self {
            Self::ZendeskIntegration { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_zendesk_integration(self) -> Option<ZendeskPlatformMetadata> {
        match self {
            Self::ZendeskIntegration { data } => Some(data),
            _ => None,
        }
    }
}
