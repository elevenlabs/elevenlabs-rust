pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioEvent {
    #[serde(default)]
    pub audio_base_64: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    /// Character-level timing data for the audio chunk, useful for synchronized text display or lip-syncing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AudioEventAlignment>,
    /// Whether this is the last audio chunk of the current agent response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
}

impl AudioEvent {
    pub fn builder() -> AudioEventBuilder {
        <AudioEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioEventBuilder {
    audio_base_64: Option<String>,
    event_id: Option<i64>,
    alignment: Option<AudioEventAlignment>,
    is_final: Option<bool>,
}

impl AudioEventBuilder {
    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn alignment(mut self, value: AudioEventAlignment) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](AudioEventBuilder::audio_base_64)
    /// - [`event_id`](AudioEventBuilder::event_id)
    pub fn build(self) -> Result<AudioEvent, BuildError> {
        Ok(AudioEvent {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            alignment: self.alignment,
            is_final: self.is_final,
        })
    }
}
