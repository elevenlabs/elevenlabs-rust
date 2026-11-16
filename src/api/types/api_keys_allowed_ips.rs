pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AllowedIps {
    StringList(Vec<String>),
    AllowedIps2(AllowedIps2),
}

impl AllowedIps {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_allowed_ips2(&self) -> bool {
        matches!(self, Self::AllowedIps2(_))
    }

    pub fn as_allowed_ips2(&self) -> Option<&AllowedIps2> {
        match self {
            Self::AllowedIps2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_allowed_ips2(self) -> Option<AllowedIps2> {
        match self {
            Self::AllowedIps2(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AllowedIps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StringList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AllowedIps2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for AllowedIps {
    fn default() -> Self {
        AllowedIps::StringList(Default::default())
    }
}
