pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AnalysisItemsUpdateRequest {
    UserEvaluationAnalysisItem(UserEvaluationAnalysisItem),
    UserDataCollectionAnalysisItemInput(UserDataCollectionAnalysisItemInput),
}

impl<'de> serde::Deserialize<'de> for AnalysisItemsUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("source").and_then(serde_json::Value::as_str) == Some("user")
            && value.get("kind").and_then(serde_json::Value::as_str) == Some("evaluation")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<UserEvaluationAnalysisItem>(value.clone())
            {
                return Ok(Self::UserEvaluationAnalysisItem(parsed));
            }
        }

        if value.get("source").and_then(serde_json::Value::as_str) == Some("user")
            && value.get("kind").and_then(serde_json::Value::as_str) == Some("data_collection")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<UserDataCollectionAnalysisItemInput>(value.clone())
            {
                return Ok(Self::UserDataCollectionAnalysisItemInput(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<UserEvaluationAnalysisItem>(value.clone())
            {
                return Ok(Self::UserEvaluationAnalysisItem(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<UserDataCollectionAnalysisItemInput>(value.clone())
            {
                return Ok(Self::UserDataCollectionAnalysisItemInput(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum AnalysisItemsUpdateRequest",
        ))
    }
}

impl AnalysisItemsUpdateRequest {
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

    pub fn is_user_data_collection_analysis_item_input(&self) -> bool {
        matches!(self, Self::UserDataCollectionAnalysisItemInput(_))
    }

    pub fn as_user_data_collection_analysis_item_input(
        &self,
    ) -> Option<&UserDataCollectionAnalysisItemInput> {
        match self {
            Self::UserDataCollectionAnalysisItemInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_data_collection_analysis_item_input(
        self,
    ) -> Option<UserDataCollectionAnalysisItemInput> {
        match self {
            Self::UserDataCollectionAnalysisItemInput(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AnalysisItemsUpdateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserEvaluationAnalysisItem(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::UserDataCollectionAnalysisItemInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
