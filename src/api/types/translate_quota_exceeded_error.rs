pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateQuotaExceededError {
    #[serde(default)]
    pub error: String,
}

impl TranslateQuotaExceededError {
    pub fn builder() -> TranslateQuotaExceededErrorBuilder {
        <TranslateQuotaExceededErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateQuotaExceededErrorBuilder {
    error: Option<String>,
}

impl TranslateQuotaExceededErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateQuotaExceededError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateQuotaExceededErrorBuilder::error)
    pub fn build(self) -> Result<TranslateQuotaExceededError, BuildError> {
        Ok(TranslateQuotaExceededError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
