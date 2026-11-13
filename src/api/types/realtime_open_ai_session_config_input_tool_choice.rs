pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OpenAiSessionConfigInputToolChoice {
    String(String),
    OpenAiToolChoiceFunction(OpenAiToolChoiceFunction),
}

impl<'de> serde::Deserialize<'de> for OpenAiSessionConfigInputToolChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("function") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<OpenAiToolChoiceFunction>(value.clone()) {
                return Ok(Self::OpenAiToolChoiceFunction(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<String>(value.clone()) {
                return Ok(Self::String(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<OpenAiToolChoiceFunction>(value.clone()) {
                return Ok(Self::OpenAiToolChoiceFunction(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum OpenAiSessionConfigInputToolChoice",
        ))
    }
}

impl OpenAiSessionConfigInputToolChoice {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_open_ai_tool_choice_function(&self) -> bool {
        matches!(self, Self::OpenAiToolChoiceFunction(_))
    }

    pub fn as_open_ai_tool_choice_function(&self) -> Option<&OpenAiToolChoiceFunction> {
        match self {
            Self::OpenAiToolChoiceFunction(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_open_ai_tool_choice_function(self) -> Option<OpenAiToolChoiceFunction> {
        match self {
            Self::OpenAiToolChoiceFunction(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for OpenAiSessionConfigInputToolChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::OpenAiToolChoiceFunction(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for OpenAiSessionConfigInputToolChoice {
    fn default() -> Self {
        OpenAiSessionConfigInputToolChoice::String(Default::default())
    }
}
