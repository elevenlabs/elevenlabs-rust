pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AnalysisItemResponseModelDefinition {
    UserEvaluationAnalysisItem(UserEvaluationAnalysisItem),
    UserDataCollectionAnalysisItemOutput(UserDataCollectionAnalysisItemOutput),

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    __Unknown(serde_json::Value),
}

impl<'de> serde::Deserialize<'de> for AnalysisItemResponseModelDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("kind").and_then(serde_json::Value::as_str) == Some("evaluation") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<UserEvaluationAnalysisItem>(value.clone())
            {
                return Ok(Self::UserEvaluationAnalysisItem(parsed));
            }
        }

        if value.get("kind").and_then(serde_json::Value::as_str) == Some("data_collection") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<UserDataCollectionAnalysisItemOutput>(value.clone())
            {
                return Ok(Self::UserDataCollectionAnalysisItemOutput(parsed));
            }
        }

        if !matched_literal {
            if value.get("kind").is_some_and(serde_json::Value::is_string) {
                return Ok(Self::__Unknown(value));
            }

            if let Ok(parsed) = serde_json::from_value::<UserEvaluationAnalysisItem>(value.clone())
            {
                return Ok(Self::UserEvaluationAnalysisItem(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<UserDataCollectionAnalysisItemOutput>(value.clone())
            {
                return Ok(Self::UserDataCollectionAnalysisItemOutput(parsed));
            }

            return Ok(Self::__Unknown(value));
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum AnalysisItemResponseModelDefinition",
        ))
    }
}

impl AnalysisItemResponseModelDefinition {
    pub fn is_user_evaluation_analysis_item(&self) -> bool {
        matches!(self, Self::UserEvaluationAnalysisItem(_))
    }

    pub fn as_user_evaluation_analysis_item(&self) -> Option<&UserEvaluationAnalysisItem> {
        match self {
            Self::UserEvaluationAnalysisItem(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_evaluation_analysis_item(self) -> Option<UserEvaluationAnalysisItem> {
        match self {
            Self::UserEvaluationAnalysisItem(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_user_data_collection_analysis_item_output(&self) -> bool {
        matches!(self, Self::UserDataCollectionAnalysisItemOutput(_))
    }

    pub fn as_user_data_collection_analysis_item_output(
        &self,
    ) -> Option<&UserDataCollectionAnalysisItemOutput> {
        match self {
            Self::UserDataCollectionAnalysisItemOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_data_collection_analysis_item_output(
        self,
    ) -> Option<UserDataCollectionAnalysisItemOutput> {
        match self {
            Self::UserDataCollectionAnalysisItemOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::__Unknown(_))
    }

    pub fn as_unknown(&self) -> Option<&serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_unknown(self) -> Option<serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AnalysisItemResponseModelDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserEvaluationAnalysisItem(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::UserDataCollectionAnalysisItemOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::__Unknown(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
