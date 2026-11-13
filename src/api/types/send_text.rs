pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendText {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_trigger_generation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<SendTextVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<SendTextGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush: Option<bool>,
}

impl SendText {
    pub fn builder() -> SendTextBuilder {
        <SendTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTextBuilder {
    text: Option<String>,
    try_trigger_generation: Option<bool>,
    voice_settings: Option<SendTextVoiceSettings>,
    generation_config: Option<SendTextGenerationConfig>,
    flush: Option<bool>,
}

impl SendTextBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn try_trigger_generation(mut self, value: bool) -> Self {
        self.try_trigger_generation = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: SendTextVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn generation_config(mut self, value: SendTextGenerationConfig) -> Self {
        self.generation_config = Some(value);
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SendTextBuilder::text)
    pub fn build(self) -> Result<SendText, BuildError> {
        Ok(SendText {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            try_trigger_generation: self.try_trigger_generation,
            voice_settings: self.voice_settings,
            generation_config: self.generation_config,
            flush: self.flush,
        })
    }
}
