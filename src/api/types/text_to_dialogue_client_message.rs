pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToDialogueClientMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<TextToDialogueClientMessageInputsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_socket: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use_token: Option<String>,
    /// List of voice IDs to register for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voices: Option<Vec<String>>,
    /// Voice settings for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<TextToDialogueClientMessageVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators:
        Option<Vec<TextToDialogueClientMessagePronunciationDictionaryLocatorsItem>>,
}

impl TextToDialogueClientMessage {
    pub fn builder() -> TextToDialogueClientMessageBuilder {
        <TextToDialogueClientMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueClientMessageBuilder {
    inputs: Option<Vec<TextToDialogueClientMessageInputsItem>>,
    flush: Option<bool>,
    close_socket: Option<bool>,
    keep_alive: Option<bool>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    single_use_token: Option<String>,
    voices: Option<Vec<String>>,
    voice_settings: Option<TextToDialogueClientMessageVoiceSettings>,
    pronunciation_dictionary_locators:
        Option<Vec<TextToDialogueClientMessagePronunciationDictionaryLocatorsItem>>,
}

impl TextToDialogueClientMessageBuilder {
    pub fn inputs(mut self, value: Vec<TextToDialogueClientMessageInputsItem>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    pub fn close_socket(mut self, value: bool) -> Self {
        self.close_socket = Some(value);
        self
    }

    pub fn keep_alive(mut self, value: bool) -> Self {
        self.keep_alive = Some(value);
        self
    }

    pub fn xi_api_key(mut self, value: impl Into<String>) -> Self {
        self.xi_api_key = Some(value.into());
        self
    }

    pub fn authorization(mut self, value: impl Into<String>) -> Self {
        self.authorization = Some(value.into());
        self
    }

    pub fn single_use_token(mut self, value: impl Into<String>) -> Self {
        self.single_use_token = Some(value.into());
        self
    }

    pub fn voices(mut self, value: Vec<String>) -> Self {
        self.voices = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: TextToDialogueClientMessageVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(
        mut self,
        value: Vec<TextToDialogueClientMessagePronunciationDictionaryLocatorsItem>,
    ) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueClientMessage`].
    pub fn build(self) -> Result<TextToDialogueClientMessage, BuildError> {
        Ok(TextToDialogueClientMessage {
            inputs: self.inputs,
            flush: self.flush,
            close_socket: self.close_socket,
            keep_alive: self.keep_alive,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
            single_use_token: self.single_use_token,
            voices: self.voices,
            voice_settings: self.voice_settings,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
        })
    }
}
