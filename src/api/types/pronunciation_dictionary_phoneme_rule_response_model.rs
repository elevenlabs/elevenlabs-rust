pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryPhonemeRuleResponseModel {
    #[serde(default)]
    pub string_to_replace: String,
    /// Whether the rule matches case-sensitively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// Whether the rule only matches at word boundaries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_boundaries: Option<bool>,
    #[serde(default)]
    pub phoneme: String,
    #[serde(default)]
    pub alphabet: String,
}

impl PronunciationDictionaryPhonemeRuleResponseModel {
    pub fn builder() -> PronunciationDictionaryPhonemeRuleResponseModelBuilder {
        <PronunciationDictionaryPhonemeRuleResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryPhonemeRuleResponseModelBuilder {
    string_to_replace: Option<String>,
    case_sensitive: Option<bool>,
    word_boundaries: Option<bool>,
    phoneme: Option<String>,
    alphabet: Option<String>,
}

impl PronunciationDictionaryPhonemeRuleResponseModelBuilder {
    pub fn string_to_replace(mut self, value: impl Into<String>) -> Self {
        self.string_to_replace = Some(value.into());
        self
    }

    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = Some(value);
        self
    }

    pub fn word_boundaries(mut self, value: bool) -> Self {
        self.word_boundaries = Some(value);
        self
    }

    pub fn phoneme(mut self, value: impl Into<String>) -> Self {
        self.phoneme = Some(value.into());
        self
    }

    pub fn alphabet(mut self, value: impl Into<String>) -> Self {
        self.alphabet = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryPhonemeRuleResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`string_to_replace`](PronunciationDictionaryPhonemeRuleResponseModelBuilder::string_to_replace)
    /// - [`phoneme`](PronunciationDictionaryPhonemeRuleResponseModelBuilder::phoneme)
    /// - [`alphabet`](PronunciationDictionaryPhonemeRuleResponseModelBuilder::alphabet)
    pub fn build(self) -> Result<PronunciationDictionaryPhonemeRuleResponseModel, BuildError> {
        Ok(PronunciationDictionaryPhonemeRuleResponseModel {
            string_to_replace: self
                .string_to_replace
                .ok_or_else(|| BuildError::missing_field("string_to_replace"))?,
            case_sensitive: self.case_sensitive,
            word_boundaries: self.word_boundaries,
            phoneme: self
                .phoneme
                .ok_or_else(|| BuildError::missing_field("phoneme"))?,
            alphabet: self
                .alphabet
                .ok_or_else(|| BuildError::missing_field("alphabet"))?,
        })
    }
}
