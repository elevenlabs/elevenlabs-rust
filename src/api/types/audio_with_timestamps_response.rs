pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioWithTimestampsResponse {
    /// Base64 encoded audio data
    #[serde(rename = "audio_base64")]
    #[serde(default)]
    pub audio_base_64: String,
    /// Timestamp information for each character in the original text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<CharacterAlignmentResponseModel>,
    /// Timestamp information for each character in the normalized text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<CharacterAlignmentResponseModel>,
}

impl AudioWithTimestampsResponse {
    pub fn builder() -> AudioWithTimestampsResponseBuilder {
        <AudioWithTimestampsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioWithTimestampsResponseBuilder {
    audio_base_64: Option<String>,
    alignment: Option<CharacterAlignmentResponseModel>,
    normalized_alignment: Option<CharacterAlignmentResponseModel>,
}

impl AudioWithTimestampsResponseBuilder {
    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    pub fn alignment(mut self, value: CharacterAlignmentResponseModel) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: CharacterAlignmentResponseModel) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioWithTimestampsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](AudioWithTimestampsResponseBuilder::audio_base_64)
    pub fn build(self) -> Result<AudioWithTimestampsResponse, BuildError> {
        Ok(AudioWithTimestampsResponse {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
        })
    }
}
