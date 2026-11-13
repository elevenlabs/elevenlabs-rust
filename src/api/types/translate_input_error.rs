pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateInputError {
    #[serde(default)]
    pub error: String,
}

impl TranslateInputError {
    pub fn builder() -> TranslateInputErrorBuilder {
        <TranslateInputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateInputErrorBuilder {
    error: Option<String>,
}

impl TranslateInputErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateInputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateInputErrorBuilder::error)
    pub fn build(self) -> Result<TranslateInputError, BuildError> {
        Ok(TranslateInputError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
