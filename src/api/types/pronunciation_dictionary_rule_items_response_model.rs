pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryRuleItemsResponseModel {
    /// The ID of the pronunciation dictionary.
    #[serde(default)]
    pub id: String,
    /// The version ID of the pronunciation dictionary.
    #[serde(default)]
    pub version_id: String,
    /// The number of rules in the version of the pronunciation dictionary.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub version_rules_num: i64,
}

impl PronunciationDictionaryRuleItemsResponseModel {
    pub fn builder() -> PronunciationDictionaryRuleItemsResponseModelBuilder {
        <PronunciationDictionaryRuleItemsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryRuleItemsResponseModelBuilder {
    id: Option<String>,
    version_id: Option<String>,
    version_rules_num: Option<i64>,
}

impl PronunciationDictionaryRuleItemsResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn version_rules_num(mut self, value: i64) -> Self {
        self.version_rules_num = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryRuleItemsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PronunciationDictionaryRuleItemsResponseModelBuilder::id)
    /// - [`version_id`](PronunciationDictionaryRuleItemsResponseModelBuilder::version_id)
    /// - [`version_rules_num`](PronunciationDictionaryRuleItemsResponseModelBuilder::version_rules_num)
    pub fn build(self) -> Result<PronunciationDictionaryRuleItemsResponseModel, BuildError> {
        Ok(PronunciationDictionaryRuleItemsResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
            version_rules_num: self
                .version_rules_num
                .ok_or_else(|| BuildError::missing_field("version_rules_num"))?,
        })
    }
}
