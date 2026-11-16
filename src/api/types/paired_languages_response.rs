pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PairedLanguagesResponse {
    /// The list of available source-to-destination language mappings.
    #[serde(default)]
    pub language_pairs: Vec<LanguagePairInfo>,
}

impl PairedLanguagesResponse {
    pub fn builder() -> PairedLanguagesResponseBuilder {
        <PairedLanguagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PairedLanguagesResponseBuilder {
    language_pairs: Option<Vec<LanguagePairInfo>>,
}

impl PairedLanguagesResponseBuilder {
    pub fn language_pairs(mut self, value: Vec<LanguagePairInfo>) -> Self {
        self.language_pairs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PairedLanguagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_pairs`](PairedLanguagesResponseBuilder::language_pairs)
    pub fn build(self) -> Result<PairedLanguagesResponse, BuildError> {
        Ok(PairedLanguagesResponse {
            language_pairs: self
                .language_pairs
                .ok_or_else(|| BuildError::missing_field("language_pairs"))?,
        })
    }
}
