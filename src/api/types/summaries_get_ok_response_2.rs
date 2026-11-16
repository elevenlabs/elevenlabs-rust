pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum SummariesGetOkResponse2 {
    #[serde(rename = "success")]
    Success {
        #[serde(flatten)]
        data: KnowledgeBaseSummaryBatchSuccessfulResponseModel,
    },

    #[serde(rename = "failure")]
    Failure {
        #[serde(flatten)]
        data: BatchFailureResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SummariesGetOkResponse2 {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    pub fn as_success(&self) -> Option<&KnowledgeBaseSummaryBatchSuccessfulResponseModel> {
        match self {
            Self::Success { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_success(self) -> Option<KnowledgeBaseSummaryBatchSuccessfulResponseModel> {
        match self {
            Self::Success { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure { .. })
    }

    pub fn as_failure(&self) -> Option<&BatchFailureResponseModel> {
        match self {
            Self::Failure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_failure(self) -> Option<BatchFailureResponseModel> {
        match self {
            Self::Failure { data } => Some(data),
            _ => None,
        }
    }
}
