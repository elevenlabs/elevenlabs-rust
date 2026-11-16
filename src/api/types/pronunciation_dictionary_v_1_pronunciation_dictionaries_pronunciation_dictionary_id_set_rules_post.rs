pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPost {
    /// List of pronunciation rules. Rule can be either:
    /// an alias rule: {'string_to_replace': 'a', 'type': 'alias', 'alias': 'b', }
    /// or a phoneme rule: {'string_to_replace': 'a', 'type': 'phoneme', 'phoneme': 'b', 'alphabet': 'ipa' }
    #[serde(default)]
    pub rules:
        Vec<DictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostRulesItem>,
}

impl PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPost {
    pub fn builder() -> PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostBuilder{
        <PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostBuilder
{
    rules: Option<
        Vec<DictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostRulesItem>,
    >,
}

impl
    PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostBuilder
{
    pub fn rules(
        mut self,
        value: Vec<
            DictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostRulesItem,
        >,
    ) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostBuilder::rules)
    pub fn build(
        self,
    ) -> Result<
        PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPost,
        BuildError,
    > {
        Ok(PronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPost {
            rules: self.rules.ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}
