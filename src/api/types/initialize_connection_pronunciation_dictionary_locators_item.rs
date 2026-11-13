pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitializeConnectionPronunciationDictionaryLocatorsItem {
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    #[serde(default)]
    pub version_id: String,
}

impl InitializeConnectionPronunciationDictionaryLocatorsItem {
    pub fn builder() -> InitializeConnectionPronunciationDictionaryLocatorsItemBuilder {
        <InitializeConnectionPronunciationDictionaryLocatorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionPronunciationDictionaryLocatorsItemBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl InitializeConnectionPronunciationDictionaryLocatorsItemBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitializeConnectionPronunciationDictionaryLocatorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](InitializeConnectionPronunciationDictionaryLocatorsItemBuilder::pronunciation_dictionary_id)
    /// - [`version_id`](InitializeConnectionPronunciationDictionaryLocatorsItemBuilder::version_id)
    pub fn build(
        self,
    ) -> Result<InitializeConnectionPronunciationDictionaryLocatorsItem, BuildError> {
        Ok(InitializeConnectionPronunciationDictionaryLocatorsItem {
            pronunciation_dictionary_id: self
                .pronunciation_dictionary_id
                .ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
