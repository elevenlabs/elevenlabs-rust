pub use crate::prelude::*;

/// Client -> server message for the multi-context endpoint.
///
/// Adds a ``context_id`` so several independent dialogue streams can be
/// multiplexed over a single socket. ``voices``/``voice_settings``/
/// ``pronunciation_dictionary_locators`` apply to a context's first message
/// (not the connection).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToDialogueMultiClientMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<TextToDialogueMultiClientMessageInputsItem>>,
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
    pub voice_settings: Option<TextToDialogueMultiClientMessageVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators:
        Option<Vec<TextToDialogueMultiClientMessagePronunciationDictionaryLocatorsItem>>,
    /// Context ID for an independent dialogue stream within the socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// Flush and close this context, leaving other contexts open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_context: Option<bool>,
}

impl TextToDialogueMultiClientMessage {
    pub fn builder() -> TextToDialogueMultiClientMessageBuilder {
        <TextToDialogueMultiClientMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueMultiClientMessageBuilder {
    inputs: Option<Vec<TextToDialogueMultiClientMessageInputsItem>>,
    flush: Option<bool>,
    close_socket: Option<bool>,
    keep_alive: Option<bool>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    single_use_token: Option<String>,
    voices: Option<Vec<String>>,
    voice_settings: Option<TextToDialogueMultiClientMessageVoiceSettings>,
    pronunciation_dictionary_locators:
        Option<Vec<TextToDialogueMultiClientMessagePronunciationDictionaryLocatorsItem>>,
    context_id: Option<String>,
    close_context: Option<bool>,
}

impl TextToDialogueMultiClientMessageBuilder {
    pub fn inputs(mut self, value: Vec<TextToDialogueMultiClientMessageInputsItem>) -> Self {
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

    pub fn voice_settings(mut self, value: TextToDialogueMultiClientMessageVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(
        mut self,
        value: Vec<TextToDialogueMultiClientMessagePronunciationDictionaryLocatorsItem>,
    ) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn close_context(mut self, value: bool) -> Self {
        self.close_context = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueMultiClientMessage`].
    pub fn build(self) -> Result<TextToDialogueMultiClientMessage, BuildError> {
        Ok(TextToDialogueMultiClientMessage {
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
            context_id: self.context_id,
            close_context: self.close_context,
        })
    }
}
