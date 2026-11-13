pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitialiseContextPronunciationDictionaryLocatorsItem {
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    #[serde(default)]
    pub version_id: String,
}

impl InitialiseContextPronunciationDictionaryLocatorsItem {
    pub fn builder() -> InitialiseContextPronunciationDictionaryLocatorsItemBuilder {
        <InitialiseContextPronunciationDictionaryLocatorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialiseContextPronunciationDictionaryLocatorsItemBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl InitialiseContextPronunciationDictionaryLocatorsItemBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitialiseContextPronunciationDictionaryLocatorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](InitialiseContextPronunciationDictionaryLocatorsItemBuilder::pronunciation_dictionary_id)
    /// - [`version_id`](InitialiseContextPronunciationDictionaryLocatorsItemBuilder::version_id)
    pub fn build(self) -> Result<InitialiseContextPronunciationDictionaryLocatorsItem, BuildError> {
        Ok(InitialiseContextPronunciationDictionaryLocatorsItem {
            pronunciation_dictionary_id: self
                .pronunciation_dictionary_id
                .ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
