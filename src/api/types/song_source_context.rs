pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SongSourceContext {
    #[serde(default)]
    pub song_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_explicit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub bpm: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_settings: Option<serde_json::Value>,
}

impl SongSourceContext {
    pub fn builder() -> SongSourceContextBuilder {
        <SongSourceContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SongSourceContextBuilder {
    song_id: Option<String>,
    chat_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    genres: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    is_explicit: Option<bool>,
    bpm: Option<i64>,
    generation_settings: Option<serde_json::Value>,
}

impl SongSourceContextBuilder {
    pub fn song_id(mut self, value: impl Into<String>) -> Self {
        self.song_id = Some(value.into());
        self
    }

    pub fn chat_id(mut self, value: impl Into<String>) -> Self {
        self.chat_id = Some(value.into());
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

    pub fn genres(mut self, value: Vec<String>) -> Self {
        self.genres = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn is_explicit(mut self, value: bool) -> Self {
        self.is_explicit = Some(value);
        self
    }

    pub fn bpm(mut self, value: i64) -> Self {
        self.bpm = Some(value);
        self
    }

    pub fn generation_settings(mut self, value: serde_json::Value) -> Self {
        self.generation_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SongSourceContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`song_id`](SongSourceContextBuilder::song_id)
    pub fn build(self) -> Result<SongSourceContext, BuildError> {
        Ok(SongSourceContext {
            song_id: self
                .song_id
                .ok_or_else(|| BuildError::missing_field("song_id"))?,
            chat_id: self.chat_id,
            title: self.title,
            description: self.description,
            genres: self.genres,
            languages: self.languages,
            is_explicit: self.is_explicit,
            bpm: self.bpm,
            generation_settings: self.generation_settings,
        })
    }
}
