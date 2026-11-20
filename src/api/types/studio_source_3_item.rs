pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Source3Item {
    PodcastTextSource(PodcastTextSource),
    PodcastUrlSource(PodcastUrlSource),
}

impl<'de> serde::Deserialize<'de> for Source3Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("text") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<PodcastTextSource>(value.clone()) {
                return Ok(Self::PodcastTextSource(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("url") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<PodcastUrlSource>(value.clone()) {
                return Ok(Self::PodcastUrlSource(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<PodcastTextSource>(value.clone()) {
                return Ok(Self::PodcastTextSource(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<PodcastUrlSource>(value.clone()) {
                return Ok(Self::PodcastUrlSource(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum Source3Item",
        ))
    }
}

impl Source3Item {
    pub fn is_podcast_text_source(&self) -> bool {
        matches!(self, Self::PodcastTextSource(_))
    }

    pub fn as_podcast_text_source(&self) -> Option<&PodcastTextSource> {
        match self {
            Self::PodcastTextSource(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_podcast_text_source(self) -> Option<PodcastTextSource> {
        match self {
            Self::PodcastTextSource(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_podcast_url_source(&self) -> bool {
        matches!(self, Self::PodcastUrlSource(_))
    }

    pub fn as_podcast_url_source(&self) -> Option<&PodcastUrlSource> {
        match self {
            Self::PodcastUrlSource(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_podcast_url_source(self) -> Option<PodcastUrlSource> {
        match self {
            Self::PodcastUrlSource(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for Source3Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PodcastTextSource(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PodcastUrlSource(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
