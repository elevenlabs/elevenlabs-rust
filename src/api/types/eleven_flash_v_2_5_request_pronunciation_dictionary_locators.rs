pub use crate::prelude::*;

/// A pronunciation dictionary to apply during speech synthesis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ElevenFlashV2_5RequestPronunciationDictionaryLocators {
    /// The ID of a pronunciation dictionary created via `POST /v1/pronunciation-dictionaries/add-from-file` or `POST /v1/pronunciation-dictionaries/add-from-rules`.
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    /// The version of the dictionary to use. Omit to use the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl ElevenFlashV2_5RequestPronunciationDictionaryLocators {
    pub fn builder() -> ElevenFlashV2_5RequestPronunciationDictionaryLocatorsBuilder {
        <ElevenFlashV2_5RequestPronunciationDictionaryLocatorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElevenFlashV2_5RequestPronunciationDictionaryLocatorsBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl ElevenFlashV2_5RequestPronunciationDictionaryLocatorsBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ElevenFlashV2_5RequestPronunciationDictionaryLocators`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](ElevenFlashV2_5RequestPronunciationDictionaryLocatorsBuilder::pronunciation_dictionary_id)
    pub fn build(
        self,
    ) -> Result<ElevenFlashV2_5RequestPronunciationDictionaryLocators, BuildError> {
        Ok(ElevenFlashV2_5RequestPronunciationDictionaryLocators {
            pronunciation_dictionary_id: self
                .pronunciation_dictionary_id
                .ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self.version_id,
        })
    }
}
