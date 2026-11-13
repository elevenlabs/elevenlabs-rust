pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowSayNodeModelInputMessage {
    #[serde(rename = "literal")]
    Literal {
        #[serde(flatten)]
        data: SayNodeLiteralMessageInput,
    },

    #[serde(rename = "prompt")]
    Prompt {
        #[serde(flatten)]
        data: SayNodePromptMessageInput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WorkflowSayNodeModelInputMessage {
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal { .. })
    }

    pub fn as_literal(&self) -> Option<&SayNodeLiteralMessageInput> {
        match self {
            Self::Literal { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_literal(self) -> Option<SayNodeLiteralMessageInput> {
        match self {
            Self::Literal { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_prompt(&self) -> bool {
        matches!(self, Self::Prompt { .. })
    }

    pub fn as_prompt(&self) -> Option<&SayNodePromptMessageInput> {
        match self {
            Self::Prompt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_prompt(self) -> Option<SayNodePromptMessageInput> {
        match self {
            Self::Prompt { data } => Some(data),
            _ => None,
        }
    }
}
