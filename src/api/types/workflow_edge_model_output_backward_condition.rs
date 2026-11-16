pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowEdgeModelOutputBackwardCondition {
    #[serde(rename = "expression")]
    Expression {
        #[serde(flatten)]
        data: WorkflowExpressionConditionModelOutput,
    },

    #[serde(rename = "llm")]
    Llm {
        #[serde(flatten)]
        data: WorkflowLlmConditionModelOutput,
    },

    #[serde(rename = "result")]
    Result {
        #[serde(flatten)]
        data: WorkflowResultConditionModelOutput,
    },

    #[serde(rename = "unconditional")]
    Unconditional {
        #[serde(flatten)]
        data: WorkflowUnconditionalModelOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WorkflowEdgeModelOutputBackwardCondition {
    pub fn is_expression(&self) -> bool {
        matches!(self, Self::Expression { .. })
    }

    pub fn as_expression(&self) -> Option<&WorkflowExpressionConditionModelOutput> {
        match self {
            Self::Expression { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_expression(self) -> Option<WorkflowExpressionConditionModelOutput> {
        match self {
            Self::Expression { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_llm(&self) -> bool {
        matches!(self, Self::Llm { .. })
    }

    pub fn as_llm(&self) -> Option<&WorkflowLlmConditionModelOutput> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_llm(self) -> Option<WorkflowLlmConditionModelOutput> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_result(&self) -> bool {
        matches!(self, Self::Result { .. })
    }

    pub fn as_result(&self) -> Option<&WorkflowResultConditionModelOutput> {
        match self {
            Self::Result { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_result(self) -> Option<WorkflowResultConditionModelOutput> {
        match self {
            Self::Result { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_unconditional(&self) -> bool {
        matches!(self, Self::Unconditional { .. })
    }

    pub fn as_unconditional(&self) -> Option<&WorkflowUnconditionalModelOutput> {
        match self {
            Self::Unconditional { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_unconditional(self) -> Option<WorkflowUnconditionalModelOutput> {
        match self {
            Self::Unconditional { data } => Some(data),
            _ => None,
        }
    }
}
