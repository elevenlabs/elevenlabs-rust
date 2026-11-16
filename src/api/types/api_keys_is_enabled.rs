pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum IsEnabled {
    Boolean(bool),
    IsEnabled2(IsEnabled2),
}

impl IsEnabled {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_is_enabled2(&self) -> bool {
        matches!(self, Self::IsEnabled2(_))
    }

    pub fn as_is_enabled2(&self) -> Option<&IsEnabled2> {
        match self {
            Self::IsEnabled2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_is_enabled2(self) -> Option<IsEnabled2> {
        match self {
            Self::IsEnabled2(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for IsEnabled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::IsEnabled2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for IsEnabled {
    fn default() -> Self {
        IsEnabled::Boolean(Default::default())
    }
}
