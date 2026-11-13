pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialiseContext {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<InitialiseContextVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<InitialiseContextGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators:
        Option<Vec<InitialiseContextPronunciationDictionaryLocatorsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl InitialiseContext {
    pub fn builder() -> InitialiseContextBuilder {
        <InitialiseContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialiseContextBuilder {
    text: Option<String>,
    voice_settings: Option<InitialiseContextVoiceSettings>,
    generation_config: Option<InitialiseContextGenerationConfig>,
    pronunciation_dictionary_locators:
        Option<Vec<InitialiseContextPronunciationDictionaryLocatorsItem>>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    context_id: Option<String>,
}

impl InitialiseContextBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: InitialiseContextVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn generation_config(mut self, value: InitialiseContextGenerationConfig) -> Self {
        self.generation_config = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(
        mut self,
        value: Vec<InitialiseContextPronunciationDictionaryLocatorsItem>,
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

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitialiseContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](InitialiseContextBuilder::text)
    pub fn build(self) -> Result<InitialiseContext, BuildError> {
        Ok(InitialiseContext {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_settings: self.voice_settings,
            generation_config: self.generation_config,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
            context_id: self.context_id,
        })
    }
}
