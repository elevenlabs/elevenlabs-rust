pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AudioChunkAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<AudioChunkNormalizedAlignment>,
}

impl AudioChunk {
    pub fn builder() -> AudioChunkBuilder {
        <AudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioChunkBuilder {
    audio: Option<String>,
    alignment: Option<AudioChunkAlignment>,
    normalized_alignment: Option<AudioChunkNormalizedAlignment>,
}

impl AudioChunkBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn alignment(mut self, value: AudioChunkAlignment) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: AudioChunkNormalizedAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioChunk`].
    pub fn build(self) -> Result<AudioChunk, BuildError> {
        Ok(AudioChunk {
            audio: self.audio,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
        })
    }
}
