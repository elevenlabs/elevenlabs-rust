pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BodyCreatePodcastV1StudioPodcastsPostMode {
    #[serde(rename = "conversation")]
    Conversation {
        #[serde(flatten)]
        data: PodcastConversationMode,
    },

    #[serde(rename = "bulletin")]
    Bulletin {
        #[serde(flatten)]
        data: PodcastBulletinMode,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl BodyCreatePodcastV1StudioPodcastsPostMode {
    pub fn is_conversation(&self) -> bool {
        matches!(self, Self::Conversation { .. })
    }

    pub fn as_conversation(&self) -> Option<&PodcastConversationMode> {
        match self {
            Self::Conversation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_conversation(self) -> Option<PodcastConversationMode> {
        match self {
            Self::Conversation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bulletin(&self) -> bool {
        matches!(self, Self::Bulletin { .. })
    }

    pub fn as_bulletin(&self) -> Option<&PodcastBulletinMode> {
        match self {
            Self::Bulletin { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bulletin(self) -> Option<PodcastBulletinMode> {
        match self {
            Self::Bulletin { data } => Some(data),
            _ => None,
        }
    }
}
