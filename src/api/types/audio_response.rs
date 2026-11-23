pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioResponse {
    pub audio_event: AudioEvent,
}

impl AudioResponse {
    pub fn builder() -> AudioResponseBuilder {
        <AudioResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioResponseBuilder {
    audio_event: Option<AudioEvent>,
}

impl AudioResponseBuilder {
    pub fn audio_event(mut self, value: AudioEvent) -> Self {
        self.audio_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_event`](AudioResponseBuilder::audio_event)
    pub fn build(self) -> Result<AudioResponse, BuildError> {
        Ok(AudioResponse {
            audio_event: self
                .audio_event
                .ok_or_else(|| BuildError::missing_field("audio_event"))?,
        })
    }
}
