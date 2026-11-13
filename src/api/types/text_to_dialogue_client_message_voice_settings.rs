pub use crate::prelude::*;

/// Voice settings for the connection.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToDialogueClientMessageVoiceSettings {
    /// Determines how stable the voice is and the randomness between each generation. Lower values introduce broader emotional range for the voice. Higher values can result in a monotonous voice with limited emotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub stability: Option<f64>,
}

impl TextToDialogueClientMessageVoiceSettings {
    pub fn builder() -> TextToDialogueClientMessageVoiceSettingsBuilder {
        <TextToDialogueClientMessageVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueClientMessageVoiceSettingsBuilder {
    stability: Option<f64>,
}

impl TextToDialogueClientMessageVoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueClientMessageVoiceSettings`].
    pub fn build(self) -> Result<TextToDialogueClientMessageVoiceSettings, BuildError> {
        Ok(TextToDialogueClientMessageVoiceSettings {
            stability: self.stability,
        })
    }
}
