pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MusicExploreSongSourceContext {
    #[serde(default)]
    pub music_explore_song_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub bpm: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<String>,
}

impl MusicExploreSongSourceContext {
    pub fn builder() -> MusicExploreSongSourceContextBuilder {
        <MusicExploreSongSourceContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MusicExploreSongSourceContextBuilder {
    music_explore_song_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    bpm: Option<i64>,
    vocals: Option<String>,
    lyrics: Option<String>,
}

impl MusicExploreSongSourceContextBuilder {
    pub fn music_explore_song_id(mut self, value: impl Into<String>) -> Self {
        self.music_explore_song_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn bpm(mut self, value: i64) -> Self {
        self.bpm = Some(value);
        self
    }

    pub fn vocals(mut self, value: impl Into<String>) -> Self {
        self.vocals = Some(value.into());
        self
    }

    pub fn lyrics(mut self, value: impl Into<String>) -> Self {
        self.lyrics = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MusicExploreSongSourceContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`music_explore_song_id`](MusicExploreSongSourceContextBuilder::music_explore_song_id)
    pub fn build(self) -> Result<MusicExploreSongSourceContext, BuildError> {
        Ok(MusicExploreSongSourceContext {
            music_explore_song_id: self
                .music_explore_song_id
                .ok_or_else(|| BuildError::missing_field("music_explore_song_id"))?,
            title: self.title,
            description: self.description,
            bpm: self.bpm,
            vocals: self.vocals,
            lyrics: self.lyrics,
        })
    }
}
