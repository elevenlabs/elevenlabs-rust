pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateTranscriberError {
    #[serde(default)]
    pub error: String,
}

impl TranslateTranscriberError {
    pub fn builder() -> TranslateTranscriberErrorBuilder {
        <TranslateTranscriberErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateTranscriberErrorBuilder {
    error: Option<String>,
}

impl TranslateTranscriberErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateTranscriberError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateTranscriberErrorBuilder::error)
    pub fn build(self) -> Result<TranslateTranscriberError, BuildError> {
        Ok(TranslateTranscriberError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
