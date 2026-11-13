pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WebhookTarget {
    #[serde(rename = "all")]
    All {
        #[serde(flatten)]
        data: WebhookTargetAll,
    },

    #[serde(rename = "ids")]
    Ids {
        #[serde(flatten)]
        data: WebhookTargetIds,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WebhookTarget {
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All { .. })
    }

    pub fn as_all(&self) -> Option<&WebhookTargetAll> {
        match self {
            Self::All { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_all(self) -> Option<WebhookTargetAll> {
        match self {
            Self::All { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_ids(&self) -> bool {
        matches!(self, Self::Ids { .. })
    }

    pub fn as_ids(&self) -> Option<&WebhookTargetIds> {
        match self {
            Self::Ids { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_ids(self) -> Option<WebhookTargetIds> {
        match self {
            Self::Ids { data } => Some(data),
            _ => None,
        }
    }
}
