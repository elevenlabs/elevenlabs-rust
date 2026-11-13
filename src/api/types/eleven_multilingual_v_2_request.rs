pub use crate::prelude::*;

/// Request body for the ElevenLabs Multilingual v2 TTS model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElevenMultilingualV2Request {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// The text to synthesize into speech.
    #[serde(default)]
    pub text: String,
    /// The ID of the voice to speak with.
    #[serde(default)]
    pub voice: String,
    /// The audio encoding of the output, as `codec_sampleRateHz_bitrateKbps`. `mp3_44100_192` requires the Creator tier or above.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ElevenFlashV2_5RequestOutputFormat>,
    /// Pronunciation dictionaries to apply to the text, in order of precedence. Up to 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators:
        Option<Vec<ElevenFlashV2_5RequestPronunciationDictionaryLocators>>,
    /// Overrides for the voice's saved settings, applied to this generation only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<TtsVoiceSettings>,
}

impl ElevenMultilingualV2Request {
    pub fn builder() -> ElevenMultilingualV2RequestBuilder {
        <ElevenMultilingualV2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElevenMultilingualV2RequestBuilder {
    webhook: Option<WebhookTarget>,
    text: Option<String>,
    voice: Option<String>,
    output_format: Option<ElevenFlashV2_5RequestOutputFormat>,
    pronunciation_dictionary_locators:
        Option<Vec<ElevenFlashV2_5RequestPronunciationDictionaryLocators>>,
    voice_settings: Option<TtsVoiceSettings>,
}

impl ElevenMultilingualV2RequestBuilder {
    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice(mut self, value: impl Into<String>) -> Self {
        self.voice = Some(value.into());
        self
    }

    pub fn output_format(mut self, value: ElevenFlashV2_5RequestOutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(
        mut self,
        value: Vec<ElevenFlashV2_5RequestPronunciationDictionaryLocators>,
    ) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: TtsVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ElevenMultilingualV2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ElevenMultilingualV2RequestBuilder::text)
    /// - [`voice`](ElevenMultilingualV2RequestBuilder::voice)
    pub fn build(self) -> Result<ElevenMultilingualV2Request, BuildError> {
        Ok(ElevenMultilingualV2Request {
            webhook: self.webhook,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice: self
                .voice
                .ok_or_else(|| BuildError::missing_field("voice"))?,
            output_format: self.output_format,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            voice_settings: self.voice_settings,
        })
    }
}
