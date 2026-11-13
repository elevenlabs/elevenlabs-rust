pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoicePreviewResponseModel {
    /// The base64 encoded audio of the preview.
    #[serde(default)]
    pub audio_base_64: String,
    /// The ID of the generated voice. Use it to create a voice from the preview.
    #[serde(default)]
    pub generated_voice_id: String,
    /// The media type of the preview.
    #[serde(default)]
    pub media_type: String,
    /// The duration of the preview in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub duration_secs: f64,
    /// The language of the preview.
    #[serde(default)]
    pub language: Option<String>,
}

impl VoicePreviewResponseModel {
    pub fn builder() -> VoicePreviewResponseModelBuilder {
        <VoicePreviewResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicePreviewResponseModelBuilder {
    audio_base_64: Option<String>,
    generated_voice_id: Option<String>,
    media_type: Option<String>,
    duration_secs: Option<f64>,
    language: Option<String>,
}

impl VoicePreviewResponseModelBuilder {
    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    pub fn generated_voice_id(mut self, value: impl Into<String>) -> Self {
        self.generated_voice_id = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn duration_secs(mut self, value: f64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoicePreviewResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](VoicePreviewResponseModelBuilder::audio_base_64)
    /// - [`generated_voice_id`](VoicePreviewResponseModelBuilder::generated_voice_id)
    /// - [`media_type`](VoicePreviewResponseModelBuilder::media_type)
    /// - [`duration_secs`](VoicePreviewResponseModelBuilder::duration_secs)
    pub fn build(self) -> Result<VoicePreviewResponseModel, BuildError> {
        Ok(VoicePreviewResponseModel {
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
            generated_voice_id: self
                .generated_voice_id
                .ok_or_else(|| BuildError::missing_field("generated_voice_id"))?,
            media_type: self
                .media_type
                .ok_or_else(|| BuildError::missing_field("media_type"))?,
            duration_secs: self
                .duration_secs
                .ok_or_else(|| BuildError::missing_field("duration_secs"))?,
            language: self.language,
        })
    }
}
