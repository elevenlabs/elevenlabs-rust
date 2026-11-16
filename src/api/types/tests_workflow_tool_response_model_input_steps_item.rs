pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowToolResponseModelInputStepsItem {
    #[serde(rename = "edge")]
    Edge {
        #[serde(flatten)]
        data: WorkflowToolEdgeStepModel,
    },

    #[serde(rename = "max_iterations_exceeded")]
    MaxIterationsExceeded {
        #[serde(flatten)]
        data: WorkflowToolMaxIterationsExceededStepModel,
    },

    #[serde(rename = "nested_tools")]
    NestedTools {
        #[serde(flatten)]
        data: WorkflowToolNestedToolsStepModelInput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WorkflowToolResponseModelInputStepsItem {
    pub fn is_edge(&self) -> bool {
        matches!(self, Self::Edge { .. })
    }

    pub fn as_edge(&self) -> Option<&WorkflowToolEdgeStepModel> {
        match self {
            Self::Edge { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_edge(self) -> Option<WorkflowToolEdgeStepModel> {
        match self {
            Self::Edge { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_max_iterations_exceeded(&self) -> bool {
        matches!(self, Self::MaxIterationsExceeded { .. })
    }

    pub fn as_max_iterations_exceeded(
        &self,
    ) -> Option<&WorkflowToolMaxIterationsExceededStepModel> {
        match self {
            Self::MaxIterationsExceeded { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_max_iterations_exceeded(
        self,
    ) -> Option<WorkflowToolMaxIterationsExceededStepModel> {
        match self {
            Self::MaxIterationsExceeded { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_nested_tools(&self) -> bool {
        matches!(self, Self::NestedTools { .. })
    }

    pub fn as_nested_tools(&self) -> Option<&WorkflowToolNestedToolsStepModelInput> {
        match self {
            Self::NestedTools { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_nested_tools(self) -> Option<WorkflowToolNestedToolsStepModelInput> {
        match self {
            Self::NestedTools { data } => Some(data),
            _ => None,
        }
    }
}
