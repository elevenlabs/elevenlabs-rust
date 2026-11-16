pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioChunkMulti {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AudioChunkMultiAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<AudioChunkMultiNormalizedAlignment>,
    /// Identifies which context this response belongs to. Only set on the multi-context endpoint; omitted (None) on the single-context endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl AudioChunkMulti {
    pub fn builder() -> AudioChunkMultiBuilder {
        <AudioChunkMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioChunkMultiBuilder {
    audio: Option<String>,
    alignment: Option<AudioChunkMultiAlignment>,
    normalized_alignment: Option<AudioChunkMultiNormalizedAlignment>,
    context_id: Option<String>,
}

impl AudioChunkMultiBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn alignment(mut self, value: AudioChunkMultiAlignment) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: AudioChunkMultiNormalizedAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioChunkMulti`].
    pub fn build(self) -> Result<AudioChunkMulti, BuildError> {
        Ok(AudioChunkMulti {
            audio: self.audio,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
            context_id: self.context_id,
        })
    }
}
