pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateUnacceptedTermsError {
    #[serde(default)]
    pub error: String,
}

impl TranslateUnacceptedTermsError {
    pub fn builder() -> TranslateUnacceptedTermsErrorBuilder {
        <TranslateUnacceptedTermsErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateUnacceptedTermsErrorBuilder {
    error: Option<String>,
}

impl TranslateUnacceptedTermsErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateUnacceptedTermsError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateUnacceptedTermsErrorBuilder::error)
    pub fn build(self) -> Result<TranslateUnacceptedTermsError, BuildError> {
        Ok(TranslateUnacceptedTermsError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
