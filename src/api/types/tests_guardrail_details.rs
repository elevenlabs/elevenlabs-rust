pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GuardrailDetails {
    #[serde(rename = "llm")]
    Llm {
        #[serde(flatten)]
        data: GuardrailLlmVerdict,
    },

    #[serde(rename = "classifier")]
    Classifier {
        #[serde(flatten)]
        data: GuardrailClassifierVerdict,
    },

    #[serde(rename = "unknown")]
    Unknown {
        #[serde(flatten)]
        data: GuardrailUnknownVerdict,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl GuardrailDetails {
    pub fn is_llm(&self) -> bool {
        matches!(self, Self::Llm { .. })
    }

    pub fn as_llm(&self) -> Option<&GuardrailLlmVerdict> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_llm(self) -> Option<GuardrailLlmVerdict> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_classifier(&self) -> bool {
        matches!(self, Self::Classifier { .. })
    }

    pub fn as_classifier(&self) -> Option<&GuardrailClassifierVerdict> {
        match self {
            Self::Classifier { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_classifier(self) -> Option<GuardrailClassifierVerdict> {
        match self {
            Self::Classifier { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }

    pub fn as_unknown(&self) -> Option<&GuardrailUnknownVerdict> {
        match self {
            Self::Unknown { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_unknown(self) -> Option<GuardrailUnknownVerdict> {
        match self {
            Self::Unknown { data } => Some(data),
            _ => None,
        }
    }
}
