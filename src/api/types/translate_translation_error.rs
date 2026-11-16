pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateTranslationError {
    #[serde(default)]
    pub error: String,
}

impl TranslateTranslationError {
    pub fn builder() -> TranslateTranslationErrorBuilder {
        <TranslateTranslationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateTranslationErrorBuilder {
    error: Option<String>,
}

impl TranslateTranslationErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateTranslationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateTranslationErrorBuilder::error)
    pub fn build(self) -> Result<TranslateTranslationError, BuildError> {
        Ok(TranslateTranslationError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
