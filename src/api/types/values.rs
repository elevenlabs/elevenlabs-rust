pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Values {
    StringToStringMap(std::collections::HashMap<String, String>),
    Variant2(std::collections::HashMap<String, EnvironmentVariableSecretValue>),
    Variant3(std::collections::HashMap<String, EnvironmentVariableAuthConnectionValue>),
}

impl Values {
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

    pub fn is_variant2(&self) -> bool {
        matches!(self, Self::Variant2(_))
    }

    pub fn as_variant2(
        &self,
    ) -> Option<&std::collections::HashMap<String, EnvironmentVariableSecretValue>> {
        match self {
            Self::Variant2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_variant2(
        self,
    ) -> Option<std::collections::HashMap<String, EnvironmentVariableSecretValue>> {
        match self {
            Self::Variant2(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_variant3(&self) -> bool {
        matches!(self, Self::Variant3(_))
    }

    pub fn as_variant3(
        &self,
    ) -> Option<&std::collections::HashMap<String, EnvironmentVariableAuthConnectionValue>> {
        match self {
            Self::Variant3(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_variant3(
        self,
    ) -> Option<std::collections::HashMap<String, EnvironmentVariableAuthConnectionValue>> {
        match self {
            Self::Variant3(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StringToStringMap(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Variant2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Variant3(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for Values {
    fn default() -> Self {
        Values::StringToStringMap(Default::default())
    }
}
