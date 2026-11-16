pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioOutputMulti {
    pub audio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<AudioOutputMultiNormalizedAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AudioOutputMultiAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl AudioOutputMulti {
    pub fn builder() -> AudioOutputMultiBuilder {
        <AudioOutputMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioOutputMultiBuilder {
    audio: Option<String>,
    is_final: Option<bool>,
    normalized_alignment: Option<AudioOutputMultiNormalizedAlignment>,
    alignment: Option<AudioOutputMultiAlignment>,
    context_id: Option<String>,
}

impl AudioOutputMultiBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: AudioOutputMultiNormalizedAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    pub fn alignment(mut self, value: AudioOutputMultiAlignment) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioOutputMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](AudioOutputMultiBuilder::audio)
    pub fn build(self) -> Result<AudioOutputMulti, BuildError> {
        Ok(AudioOutputMulti {
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            is_final: self.is_final,
            normalized_alignment: self.normalized_alignment,
            alignment: self.alignment,
            context_id: self.context_id,
        })
    }
}
