pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ExpiresAtUnix {
    Integer(i64),
    AllowedIps2(AllowedIps2),
}

impl ExpiresAtUnix {
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

impl fmt::Display for ExpiresAtUnix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(
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

impl Default for ExpiresAtUnix {
    fn default() -> Self {
        ExpiresAtUnix::Integer(Default::default())
    }
}
