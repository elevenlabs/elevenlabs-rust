pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateTranslation {
    #[serde(default)]
    pub text: String,
}

impl TranslateTranslation {
    pub fn builder() -> TranslateTranslationBuilder {
        <TranslateTranslationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateTranslationBuilder {
    text: Option<String>,
}

impl TranslateTranslationBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateTranslation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TranslateTranslationBuilder::text)
    pub fn build(self) -> Result<TranslateTranslation, BuildError> {
        Ok(TranslateTranslation {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
