pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Source3Item {
    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        data: PodcastTextSource,
    },

    #[serde(rename = "url")]
    Url {
        #[serde(flatten)]
        data: PodcastUrlSource,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl Source3Item {
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn as_text(&self) -> Option<&PodcastTextSource> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<PodcastTextSource> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    pub fn as_url(&self) -> Option<&PodcastUrlSource> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_url(self) -> Option<PodcastUrlSource> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }
}
