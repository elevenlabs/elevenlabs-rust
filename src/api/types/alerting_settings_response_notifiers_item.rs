pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AlertingSettingsResponseNotifiersItem {
    #[serde(rename = "webhook")]
    Webhook {
        #[serde(flatten)]
        data: AlertingWebhookNotifierResponse,
    },

    #[serde(rename = "integration")]
    Integration {
        #[serde(flatten)]
        data: AlertingIntegrationNotifierResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AlertingSettingsResponseNotifiersItem {
    pub fn is_webhook(&self) -> bool {
        matches!(self, Self::Webhook { .. })
    }

    pub fn as_webhook(&self) -> Option<&AlertingWebhookNotifierResponse> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_webhook(self) -> Option<AlertingWebhookNotifierResponse> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_integration(&self) -> bool {
        matches!(self, Self::Integration { .. })
    }

    pub fn as_integration(&self) -> Option<&AlertingIntegrationNotifierResponse> {
        match self {
            Self::Integration { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_integration(self) -> Option<AlertingIntegrationNotifierResponse> {
        match self {
            Self::Integration { data } => Some(data),
            _ => None,
        }
    }
}
