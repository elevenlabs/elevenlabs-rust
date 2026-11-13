pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AddVoiceRequestLabels {
    StringToStringMap(std::collections::HashMap<String, String>),
    String(String),
}

impl AddVoiceRequestLabels {
    pub fn is_string_to_string_map(&self) -> bool {
        matches!(self, Self::StringToStringMap(_))
    }

    pub fn as_string_to_string_map(&self) -> Option<&std::collections::HashMap<String, String>> {
        match self {
            Self::StringToStringMap(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_to_string_map(self) -> Option<std::collections::HashMap<String, String>> {
        match self {
            Self::StringToStringMap(value) => Some(value),
            _ => None,
        }
    }

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
}

impl fmt::Display for AddVoiceRequestLabels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StringToStringMap(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::String(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for AddVoiceRequestLabels {
    fn default() -> Self {
        AddVoiceRequestLabels::StringToStringMap(Default::default())
    }
}
