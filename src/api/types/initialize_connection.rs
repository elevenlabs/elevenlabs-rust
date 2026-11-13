pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeConnection {
    /// The initial text that must be sent is a blank space.
    pub text: InitializeConnectionMultiText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<InitializeConnectionVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<InitializeConnectionGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators:
        Option<Vec<InitializeConnectionPronunciationDictionaryLocatorsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
}

impl InitializeConnection {
    pub fn builder() -> InitializeConnectionBuilder {
        <InitializeConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionBuilder {
    text: Option<InitializeConnectionMultiText>,
    voice_settings: Option<InitializeConnectionVoiceSettings>,
    generation_config: Option<InitializeConnectionGenerationConfig>,
    pronunciation_dictionary_locators:
        Option<Vec<InitializeConnectionPronunciationDictionaryLocatorsItem>>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
}

impl InitializeConnectionBuilder {
    pub fn text(mut self, value: InitializeConnectionMultiText) -> Self {
        self.text = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: InitializeConnectionVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn generation_config(mut self, value: InitializeConnectionGenerationConfig) -> Self {
        self.generation_config = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(
        mut self,
        value: Vec<InitializeConnectionPronunciationDictionaryLocatorsItem>,
    ) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
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

    /// Consumes the builder and constructs a [`InitializeConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](InitializeConnectionBuilder::text)
    pub fn build(self) -> Result<InitializeConnection, BuildError> {
        Ok(InitializeConnection {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_settings: self.voice_settings,
            generation_config: self.generation_config,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
        })
    }
}
