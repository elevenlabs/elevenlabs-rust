pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateRateLimitedError {
    #[serde(default)]
    pub error: String,
}

impl TranslateRateLimitedError {
    pub fn builder() -> TranslateRateLimitedErrorBuilder {
        <TranslateRateLimitedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateRateLimitedErrorBuilder {
    error: Option<String>,
}

impl TranslateRateLimitedErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateRateLimitedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateRateLimitedErrorBuilder::error)
    pub fn build(self) -> Result<TranslateRateLimitedError, BuildError> {
        Ok(TranslateRateLimitedError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
