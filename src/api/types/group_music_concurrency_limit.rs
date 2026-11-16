pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum GroupMusicConcurrencyLimit {
    Integer(i64),
    GroupConcurrencyLimit2(GroupConcurrencyLimit2),
}

impl GroupMusicConcurrencyLimit {
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

    pub fn is_group_concurrency_limit2(&self) -> bool {
        matches!(self, Self::GroupConcurrencyLimit2(_))
    }

    pub fn as_group_concurrency_limit2(&self) -> Option<&GroupConcurrencyLimit2> {
        match self {
            Self::GroupConcurrencyLimit2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_group_concurrency_limit2(self) -> Option<GroupConcurrencyLimit2> {
        match self {
            Self::GroupConcurrencyLimit2(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GroupMusicConcurrencyLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::GroupConcurrencyLimit2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for GroupMusicConcurrencyLimit {
    fn default() -> Self {
        GroupMusicConcurrencyLimit::Integer(Default::default())
    }
}
