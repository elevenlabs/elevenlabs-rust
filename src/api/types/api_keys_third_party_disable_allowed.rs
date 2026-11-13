pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ThirdPartyDisableAllowed {
    Boolean(bool),
    AllowedIps2(AllowedIps2),
}

impl ThirdPartyDisableAllowed {
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

impl fmt::Display for ThirdPartyDisableAllowed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(
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

impl Default for ThirdPartyDisableAllowed {
    fn default() -> Self {
        ThirdPartyDisableAllowed::Boolean(Default::default())
    }
}
