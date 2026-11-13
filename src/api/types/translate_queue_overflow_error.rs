pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateQueueOverflowError {
    #[serde(default)]
    pub error: String,
}

impl TranslateQueueOverflowError {
    pub fn builder() -> TranslateQueueOverflowErrorBuilder {
        <TranslateQueueOverflowErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateQueueOverflowErrorBuilder {
    error: Option<String>,
}

impl TranslateQueueOverflowErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateQueueOverflowError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateQueueOverflowErrorBuilder::error)
    pub fn build(self) -> Result<TranslateQueueOverflowError, BuildError> {
        Ok(TranslateQueueOverflowError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
