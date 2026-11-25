pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingInputAudioChunk {
    /// Base64-encoded audio data. Recommended chunk size is around 100 ms of audio.
    #[serde(default)]
    pub audio_base_64: String,
}

impl SubtitlingInputAudioChunk {
    pub fn builder() -> SubtitlingInputAudioChunkBuilder {
        <SubtitlingInputAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingInputAudioChunkBuilder {
    audio_base_64: Option<String>,
}

impl SubtitlingInputAudioChunkBuilder {
    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingInputAudioChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](SubtitlingInputAudioChunkBuilder::audio_base_64)
    pub fn build(self) -> Result<SubtitlingInputAudioChunk, BuildError> {
        Ok(SubtitlingInputAudioChunk {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
        })
    }
}
