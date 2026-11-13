pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum AnalysisItemVersionResponseModelDefinition {
    #[serde(rename = "data_collection")]
    DataCollection {
        #[serde(flatten)]
        data: UserDataCollectionAnalysisItemOutput,
    },

    #[serde(rename = "evaluation")]
    Evaluation {
        #[serde(flatten)]
        data: UserEvaluationAnalysisItem,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AnalysisItemVersionResponseModelDefinition {
    pub fn is_data_collection(&self) -> bool {
        matches!(self, Self::DataCollection { .. })
    }

    pub fn as_data_collection(&self) -> Option<&UserDataCollectionAnalysisItemOutput> {
        match self {
            Self::DataCollection { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_data_collection(self) -> Option<UserDataCollectionAnalysisItemOutput> {
        match self {
            Self::DataCollection { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_evaluation(&self) -> bool {
        matches!(self, Self::Evaluation { .. })
    }

    pub fn as_evaluation(&self) -> Option<&UserEvaluationAnalysisItem> {
        match self {
            Self::Evaluation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_evaluation(self) -> Option<UserEvaluationAnalysisItem> {
        match self {
            Self::Evaluation { data } => Some(data),
            _ => None,
        }
    }
}
