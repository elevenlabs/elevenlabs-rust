pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum PendingChange {
    #[serde(rename = "change")]
    Change {
        #[serde(flatten)]
        data: PendingSubscriptionSwitchResponseModel,
    },

    #[serde(rename = "cancellation")]
    Cancellation {
        #[serde(flatten)]
        data: PendingCancellationResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PendingChange {
    pub fn is_change(&self) -> bool {
        matches!(self, Self::Change { .. })
    }

    pub fn as_change(&self) -> Option<&PendingSubscriptionSwitchResponseModel> {
        match self {
            Self::Change { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_change(self) -> Option<PendingSubscriptionSwitchResponseModel> {
        match self {
            Self::Change { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_cancellation(&self) -> bool {
        matches!(self, Self::Cancellation { .. })
    }

    pub fn as_cancellation(&self) -> Option<&PendingCancellationResponseModel> {
        match self {
            Self::Cancellation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_cancellation(self) -> Option<PendingCancellationResponseModel> {
        match self {
            Self::Cancellation { data } => Some(data),
            _ => None,
        }
    }
}
