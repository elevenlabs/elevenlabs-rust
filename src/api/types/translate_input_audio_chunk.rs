pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateInputAudioChunk {
    /// Base64-encoded audio data. Recommended chunk size is around 100 ms of audio.
    #[serde(default)]
    pub audio_base_64: String,
}

impl TranslateInputAudioChunk {
    pub fn builder() -> TranslateInputAudioChunkBuilder {
        <TranslateInputAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateInputAudioChunkBuilder {
    audio_base_64: Option<String>,
}

impl TranslateInputAudioChunkBuilder {
    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateInputAudioChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](TranslateInputAudioChunkBuilder::audio_base_64)
    pub fn build(self) -> Result<TranslateInputAudioChunk, BuildError> {
        Ok(TranslateInputAudioChunk {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
        })
    }
}
