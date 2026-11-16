pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WebhookMetadata {
    String(String),
    Value(serde_json::Value),
}

impl WebhookMetadata {
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

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_value(&self) -> Option<&serde_json::Value> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_value(self) -> Option<serde_json::Value> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WebhookMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Value(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for WebhookMetadata {
    fn default() -> Self {
        WebhookMetadata::String(Default::default())
    }
}
