pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioOutput {
    pub audio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<AudioOutputNormalizedAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AudioOutputAlignment>,
}

impl AudioOutput {
    pub fn builder() -> AudioOutputBuilder {
        <AudioOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioOutputBuilder {
    audio: Option<String>,
    is_final: Option<bool>,
    normalized_alignment: Option<AudioOutputNormalizedAlignment>,
    alignment: Option<AudioOutputAlignment>,
}

impl AudioOutputBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: AudioOutputNormalizedAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    pub fn alignment(mut self, value: AudioOutputAlignment) -> Self {
        self.alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](AudioOutputBuilder::audio)
    pub fn build(self) -> Result<AudioOutput, BuildError> {
        Ok(AudioOutput {
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            is_final: self.is_final,
            normalized_alignment: self.normalized_alignment,
            alignment: self.alignment,
        })
    }
}
