pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SingleLanguagesResponse {
    /// The list of available languages.
    #[serde(default)]
    pub languages: Vec<LanguageInfo>,
}

impl SingleLanguagesResponse {
    pub fn builder() -> SingleLanguagesResponseBuilder {
        <SingleLanguagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SingleLanguagesResponseBuilder {
    languages: Option<Vec<LanguageInfo>>,
}

impl SingleLanguagesResponseBuilder {
    pub fn languages(mut self, value: Vec<LanguageInfo>) -> Self {
        self.languages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SingleLanguagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`languages`](SingleLanguagesResponseBuilder::languages)
    pub fn build(self) -> Result<SingleLanguagesResponse, BuildError> {
        Ok(SingleLanguagesResponse {
            languages: self
                .languages
                .ok_or_else(|| BuildError::missing_field("languages"))?,
        })
    }
}
