pub use crate::prelude::*;

/// Voice settings for the connection.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToDialogueMultiClientMessageVoiceSettings {
    /// Determines how stable the voice is and the randomness between each generation. Lower values introduce broader emotional range for the voice. Higher values can result in a monotonous voice with limited emotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub stability: Option<f64>,
}

impl TextToDialogueMultiClientMessageVoiceSettings {
    pub fn builder() -> TextToDialogueMultiClientMessageVoiceSettingsBuilder {
        <TextToDialogueMultiClientMessageVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueMultiClientMessageVoiceSettingsBuilder {
    stability: Option<f64>,
}

impl TextToDialogueMultiClientMessageVoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueMultiClientMessageVoiceSettings`].
    pub fn build(self) -> Result<TextToDialogueMultiClientMessageVoiceSettings, BuildError> {
        Ok(TextToDialogueMultiClientMessageVoiceSettings {
            stability: self.stability,
        })
    }
}
