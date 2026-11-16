pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioWithTimestampsAndVoiceSegmentsResponseModel {
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
    /// Voice segments for the audio
    #[serde(default)]
    pub voice_segments: Vec<VoiceSegment>,
}

impl AudioWithTimestampsAndVoiceSegmentsResponseModel {
    pub fn builder() -> AudioWithTimestampsAndVoiceSegmentsResponseModelBuilder {
        <AudioWithTimestampsAndVoiceSegmentsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioWithTimestampsAndVoiceSegmentsResponseModelBuilder {
    audio_base_64: Option<String>,
    alignment: Option<CharacterAlignmentResponseModel>,
    normalized_alignment: Option<CharacterAlignmentResponseModel>,
    voice_segments: Option<Vec<VoiceSegment>>,
}

impl AudioWithTimestampsAndVoiceSegmentsResponseModelBuilder {
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

    pub fn voice_segments(mut self, value: Vec<VoiceSegment>) -> Self {
        self.voice_segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioWithTimestampsAndVoiceSegmentsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](AudioWithTimestampsAndVoiceSegmentsResponseModelBuilder::audio_base_64)
    /// - [`voice_segments`](AudioWithTimestampsAndVoiceSegmentsResponseModelBuilder::voice_segments)
    pub fn build(self) -> Result<AudioWithTimestampsAndVoiceSegmentsResponseModel, BuildError> {
        Ok(AudioWithTimestampsAndVoiceSegmentsResponseModel {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
            voice_segments: self
                .voice_segments
                .ok_or_else(|| BuildError::missing_field("voice_segments"))?,
        })
    }
}
