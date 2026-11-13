pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OpenAiSessionConfigOutputMaxResponseOutputTokens {
    Integer(i64),
    MaxResponseOutputTokens2_1(MaxResponseOutputTokens2_1),
}

impl OpenAiSessionConfigOutputMaxResponseOutputTokens {
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_max_response_output_tokens2_1(&self) -> bool {
        matches!(self, Self::MaxResponseOutputTokens2_1(_))
    }

    pub fn as_max_response_output_tokens2_1(&self) -> Option<&MaxResponseOutputTokens2_1> {
        match self {
            Self::MaxResponseOutputTokens2_1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_max_response_output_tokens2_1(self) -> Option<MaxResponseOutputTokens2_1> {
        match self {
            Self::MaxResponseOutputTokens2_1(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for OpenAiSessionConfigOutputMaxResponseOutputTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::MaxResponseOutputTokens2_1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for OpenAiSessionConfigOutputMaxResponseOutputTokens {
    fn default() -> Self {
        OpenAiSessionConfigOutputMaxResponseOutputTokens::Integer(Default::default())
    }
}
