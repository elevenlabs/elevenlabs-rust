pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingInputAudioChunk {
    #[serde(default)]
    pub audio_base_64: String,
}

impl DubbingInputAudioChunk {
    pub fn builder() -> DubbingInputAudioChunkBuilder {
        <DubbingInputAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingInputAudioChunkBuilder {
    audio_base_64: Option<String>,
}

impl DubbingInputAudioChunkBuilder {
    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingInputAudioChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](DubbingInputAudioChunkBuilder::audio_base_64)
    pub fn build(self) -> Result<DubbingInputAudioChunk, BuildError> {
        Ok(DubbingInputAudioChunk {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
        })
    }
}
