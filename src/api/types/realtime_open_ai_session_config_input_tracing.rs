pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OpenAiSessionConfigInputTracing {
    OpenAiSessionConfigInputTracing1(OpenAiSessionConfigInputTracing1),
    OpenAiTracingConfiguration(OpenAiTracingConfiguration),
}

impl OpenAiSessionConfigInputTracing {
    pub fn is_open_ai_session_config_input_tracing1(&self) -> bool {
        matches!(self, Self::OpenAiSessionConfigInputTracing1(_))
    }

    pub fn as_open_ai_session_config_input_tracing1(
        &self,
    ) -> Option<&OpenAiSessionConfigInputTracing1> {
        match self {
            Self::OpenAiSessionConfigInputTracing1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_open_ai_session_config_input_tracing1(
        self,
    ) -> Option<OpenAiSessionConfigInputTracing1> {
        match self {
            Self::OpenAiSessionConfigInputTracing1(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_open_ai_tracing_configuration(&self) -> bool {
        matches!(self, Self::OpenAiTracingConfiguration(_))
    }

    pub fn as_open_ai_tracing_configuration(&self) -> Option<&OpenAiTracingConfiguration> {
        match self {
            Self::OpenAiTracingConfiguration(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_open_ai_tracing_configuration(self) -> Option<OpenAiTracingConfiguration> {
        match self {
            Self::OpenAiTracingConfiguration(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for OpenAiSessionConfigInputTracing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenAiSessionConfigInputTracing1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::OpenAiTracingConfiguration(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for OpenAiSessionConfigInputTracing {
    fn default() -> Self {
        OpenAiSessionConfigInputTracing::OpenAiTracingConfiguration(Default::default())
    }
}
