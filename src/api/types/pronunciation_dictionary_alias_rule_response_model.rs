pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryAliasRuleResponseModel {
    #[serde(default)]
    pub string_to_replace: String,
    /// Whether the rule matches case-sensitively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// Whether the rule only matches at word boundaries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_boundaries: Option<bool>,
    #[serde(default)]
    pub alias: String,
}

impl PronunciationDictionaryAliasRuleResponseModel {
    pub fn builder() -> PronunciationDictionaryAliasRuleResponseModelBuilder {
        <PronunciationDictionaryAliasRuleResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryAliasRuleResponseModelBuilder {
    string_to_replace: Option<String>,
    case_sensitive: Option<bool>,
    word_boundaries: Option<bool>,
    alias: Option<String>,
}

impl PronunciationDictionaryAliasRuleResponseModelBuilder {
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

    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryAliasRuleResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`string_to_replace`](PronunciationDictionaryAliasRuleResponseModelBuilder::string_to_replace)
    /// - [`alias`](PronunciationDictionaryAliasRuleResponseModelBuilder::alias)
    pub fn build(self) -> Result<PronunciationDictionaryAliasRuleResponseModel, BuildError> {
        Ok(PronunciationDictionaryAliasRuleResponseModel {
            string_to_replace: self
                .string_to_replace
                .ok_or_else(|| BuildError::missing_field("string_to_replace"))?,
            case_sensitive: self.case_sensitive,
            word_boundaries: self.word_boundaries,
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
        })
    }
}
