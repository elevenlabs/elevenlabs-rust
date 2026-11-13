pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source")]
#[non_exhaustive]
pub enum AgentAnalysisItemsOutputDataCollectionItem {
    #[serde(rename = "system")]
    System {
        #[serde(flatten)]
        data: AttachedSystemDataCollectionRef,
    },

    #[serde(rename = "user")]
    User {
        #[serde(flatten)]
        data: AttachedUserDataCollectionRef,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AgentAnalysisItemsOutputDataCollectionItem {
    pub fn is_system(&self) -> bool {
        matches!(self, Self::System { .. })
    }

    pub fn as_system(&self) -> Option<&AttachedSystemDataCollectionRef> {
        match self {
            Self::System { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_system(self) -> Option<AttachedSystemDataCollectionRef> {
        match self {
            Self::System { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_user(&self) -> bool {
        matches!(self, Self::User { .. })
    }

    pub fn as_user(&self) -> Option<&AttachedUserDataCollectionRef> {
        match self {
            Self::User { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_user(self) -> Option<AttachedUserDataCollectionRef> {
        match self {
            Self::User { data } => Some(data),
            _ => None,
        }
    }
}
