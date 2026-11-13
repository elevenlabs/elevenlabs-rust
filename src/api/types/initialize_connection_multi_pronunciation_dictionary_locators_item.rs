pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitializeConnectionMultiPronunciationDictionaryLocatorsItem {
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    #[serde(default)]
    pub version_id: String,
}

impl InitializeConnectionMultiPronunciationDictionaryLocatorsItem {
    pub fn builder() -> InitializeConnectionMultiPronunciationDictionaryLocatorsItemBuilder {
        <InitializeConnectionMultiPronunciationDictionaryLocatorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionMultiPronunciationDictionaryLocatorsItemBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl InitializeConnectionMultiPronunciationDictionaryLocatorsItemBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitializeConnectionMultiPronunciationDictionaryLocatorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](InitializeConnectionMultiPronunciationDictionaryLocatorsItemBuilder::pronunciation_dictionary_id)
    /// - [`version_id`](InitializeConnectionMultiPronunciationDictionaryLocatorsItemBuilder::version_id)
    pub fn build(
        self,
    ) -> Result<InitializeConnectionMultiPronunciationDictionaryLocatorsItem, BuildError> {
        Ok(
            InitializeConnectionMultiPronunciationDictionaryLocatorsItem {
                pronunciation_dictionary_id: self
                    .pronunciation_dictionary_id
                    .ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
                version_id: self
                    .version_id
                    .ok_or_else(|| BuildError::missing_field("version_id"))?,
            },
        )
    }
}
