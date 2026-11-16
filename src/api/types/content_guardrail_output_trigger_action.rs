pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContentGuardrailOutputTriggerAction {
    #[serde(rename = "end_call")]
    EndCall {
        #[serde(flatten)]
        data: EndCallTriggerAction,
    },

    #[serde(rename = "retry")]
    Retry {
        #[serde(flatten)]
        data: RetryTriggerAction,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ContentGuardrailOutputTriggerAction {
    pub fn is_end_call(&self) -> bool {
        matches!(self, Self::EndCall { .. })
    }

    pub fn as_end_call(&self) -> Option<&EndCallTriggerAction> {
        match self {
            Self::EndCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_call(self) -> Option<EndCallTriggerAction> {
        match self {
            Self::EndCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_retry(&self) -> bool {
        matches!(self, Self::Retry { .. })
    }

    pub fn as_retry(&self) -> Option<&RetryTriggerAction> {
        match self {
            Self::Retry { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_retry(self) -> Option<RetryTriggerAction> {
        match self {
            Self::Retry { data } => Some(data),
            _ => None,
        }
    }
}
