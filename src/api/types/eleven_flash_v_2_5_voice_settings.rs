pub use crate::prelude::*;

/// Overrides for the voice's saved settings, applied to one generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ElevenFlashV2_5VoiceSettings {
    /// How consistent the voice stays across generations. Lower values give more expressive, varied speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub stability: Option<f64>,
    /// How closely the output adheres to the original voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub similarity_boost: Option<f64>,
    /// The speed of the generated speech, where 1.0 is the voice's natural pace.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub speed: Option<f64>,
}

impl ElevenFlashV2_5VoiceSettings {
    pub fn builder() -> ElevenFlashV2_5VoiceSettingsBuilder {
        <ElevenFlashV2_5VoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElevenFlashV2_5VoiceSettingsBuilder {
    stability: Option<f64>,
    similarity_boost: Option<f64>,
    speed: Option<f64>,
}

impl ElevenFlashV2_5VoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn similarity_boost(mut self, value: f64) -> Self {
        self.similarity_boost = Some(value);
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ElevenFlashV2_5VoiceSettings`].
    pub fn build(self) -> Result<ElevenFlashV2_5VoiceSettings, BuildError> {
        Ok(ElevenFlashV2_5VoiceSettings {
            stability: self.stability,
            similarity_boost: self.similarity_boost,
            speed: self.speed,
        })
    }
}
