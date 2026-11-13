pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GetPronunciationDictionaryWithRulesResponseModelRulesItem {
    #[serde(rename = "alias")]
    Alias {
        #[serde(flatten)]
        data: PronunciationDictionaryAliasRuleResponseModel,
    },

    #[serde(rename = "phoneme")]
    Phoneme {
        #[serde(flatten)]
        data: PronunciationDictionaryPhonemeRuleResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl GetPronunciationDictionaryWithRulesResponseModelRulesItem {
    pub fn is_alias(&self) -> bool {
        matches!(self, Self::Alias { .. })
    }

    pub fn as_alias(&self) -> Option<&PronunciationDictionaryAliasRuleResponseModel> {
        match self {
            Self::Alias { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_alias(self) -> Option<PronunciationDictionaryAliasRuleResponseModel> {
        match self {
            Self::Alias { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_phoneme(&self) -> bool {
        matches!(self, Self::Phoneme { .. })
    }

    pub fn as_phoneme(&self) -> Option<&PronunciationDictionaryPhonemeRuleResponseModel> {
        match self {
            Self::Phoneme { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phoneme(self) -> Option<PronunciationDictionaryPhonemeRuleResponseModel> {
        match self {
            Self::Phoneme { data } => Some(data),
            _ => None,
        }
    }
}
