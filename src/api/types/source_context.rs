pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source_type")]
#[non_exhaustive]
pub enum SourceContext {
    #[serde(rename = "music_explore_song")]
    MusicExploreSong {
        #[serde(flatten)]
        data: MusicExploreSongSourceContext,
    },

    #[serde(rename = "sfx")]
    Sfx {
        #[serde(flatten)]
        data: SfxSourceContext,
    },

    #[serde(rename = "song")]
    Song {
        #[serde(flatten)]
        data: SongSourceContext,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SourceContext {
    pub fn is_music_explore_song(&self) -> bool {
        matches!(self, Self::MusicExploreSong { .. })
    }

    pub fn as_music_explore_song(&self) -> Option<&MusicExploreSongSourceContext> {
        match self {
            Self::MusicExploreSong { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_music_explore_song(self) -> Option<MusicExploreSongSourceContext> {
        match self {
            Self::MusicExploreSong { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sfx(&self) -> bool {
        matches!(self, Self::Sfx { .. })
    }

    pub fn as_sfx(&self) -> Option<&SfxSourceContext> {
        match self {
            Self::Sfx { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sfx(self) -> Option<SfxSourceContext> {
        match self {
            Self::Sfx { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_song(&self) -> bool {
        matches!(self, Self::Song { .. })
    }

    pub fn as_song(&self) -> Option<&SongSourceContext> {
        match self {
            Self::Song { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_song(self) -> Option<SongSourceContext> {
        match self {
            Self::Song { data } => Some(data),
            _ => None,
        }
    }
}
