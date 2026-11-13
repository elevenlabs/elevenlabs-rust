pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateError {
    #[serde(default)]
    pub error: String,
}

impl TranslateError {
    pub fn builder() -> TranslateErrorBuilder {
        <TranslateErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateErrorBuilder {
    error: Option<String>,
}

impl TranslateErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateErrorBuilder::error)
    pub fn build(self) -> Result<TranslateError, BuildError> {
        Ok(TranslateError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
