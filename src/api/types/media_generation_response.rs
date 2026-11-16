pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum MediaGenerationResponse {
    #[serde(rename = "completed")]
    Completed {
        #[serde(flatten)]
        data: MediaGenerationCompletedResponse,
    },

    #[serde(rename = "failed")]
    Failed {
        #[serde(flatten)]
        data: MediaGenerationFailedResponse,
    },

    #[serde(rename = "generating")]
    Generating {
        #[serde(flatten)]
        data: MediaGenerationInProgressResponse,
    },

    #[serde(rename = "pending")]
    Pending {
        #[serde(flatten)]
        data: MediaGenerationInProgressResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl MediaGenerationResponse {
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    pub fn as_completed(&self) -> Option<&MediaGenerationCompletedResponse> {
        match self {
            Self::Completed { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_completed(self) -> Option<MediaGenerationCompletedResponse> {
        match self {
            Self::Completed { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    pub fn as_failed(&self) -> Option<&MediaGenerationFailedResponse> {
        match self {
            Self::Failed { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_failed(self) -> Option<MediaGenerationFailedResponse> {
        match self {
            Self::Failed { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_generating(&self) -> bool {
        matches!(self, Self::Generating { .. })
    }

    pub fn as_generating(&self) -> Option<&MediaGenerationInProgressResponse> {
        match self {
            Self::Generating { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_generating(self) -> Option<MediaGenerationInProgressResponse> {
        match self {
            Self::Generating { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending { .. })
    }

    pub fn as_pending(&self) -> Option<&MediaGenerationInProgressResponse> {
        match self {
            Self::Pending { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_pending(self) -> Option<MediaGenerationInProgressResponse> {
        match self {
            Self::Pending { data } => Some(data),
            _ => None,
        }
    }
}
