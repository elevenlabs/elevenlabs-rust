pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateAuthError {
    #[serde(default)]
    pub error: String,
}

impl TranslateAuthError {
    pub fn builder() -> TranslateAuthErrorBuilder {
        <TranslateAuthErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateAuthErrorBuilder {
    error: Option<String>,
}

impl TranslateAuthErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateAuthError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateAuthErrorBuilder::error)
    pub fn build(self) -> Result<TranslateAuthError, BuildError> {
        Ok(TranslateAuthError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
