pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PatchConvAiDashboardSettingsRequestChartsItem {
    #[serde(rename = "call_success")]
    CallSuccess {
        #[serde(flatten)]
        data: DashboardCallSuccessChartModel,
    },

    #[serde(rename = "criteria")]
    Criteria {
        #[serde(flatten)]
        data: DashboardCriteriaChartModel,
    },

    #[serde(rename = "data_collection")]
    DataCollection {
        #[serde(flatten)]
        data: DashboardDataCollectionChartModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PatchConvAiDashboardSettingsRequestChartsItem {
    pub fn is_call_success(&self) -> bool {
        matches!(self, Self::CallSuccess { .. })
    }

    pub fn as_call_success(&self) -> Option<&DashboardCallSuccessChartModel> {
        match self {
            Self::CallSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_call_success(self) -> Option<DashboardCallSuccessChartModel> {
        match self {
            Self::CallSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_criteria(&self) -> bool {
        matches!(self, Self::Criteria { .. })
    }

    pub fn as_criteria(&self) -> Option<&DashboardCriteriaChartModel> {
        match self {
            Self::Criteria { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_criteria(self) -> Option<DashboardCriteriaChartModel> {
        match self {
            Self::Criteria { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_data_collection(&self) -> bool {
        matches!(self, Self::DataCollection { .. })
    }

    pub fn as_data_collection(&self) -> Option<&DashboardDataCollectionChartModel> {
        match self {
            Self::DataCollection { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_data_collection(self) -> Option<DashboardDataCollectionChartModel> {
        match self {
            Self::DataCollection { data } => Some(data),
            _ => None,
        }
    }
}
