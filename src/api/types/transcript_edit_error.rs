pub use crate::prelude::*;

/// An error returned in place of an edited transcript when the edit could not be produced.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranscriptEditError {
    /// edit_failed: the edit could not be produced.
    pub error_type: ErrorType,
    /// A short, user-facing explanation of the failure.
    #[serde(default)]
    pub message: String,
}

impl TranscriptEditError {
    pub fn builder() -> TranscriptEditErrorBuilder {
        <TranscriptEditErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptEditErrorBuilder {
    error_type: Option<ErrorType>,
    message: Option<String>,
}

impl TranscriptEditErrorBuilder {
    pub fn error_type(mut self, value: ErrorType) -> Self {
        self.error_type = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptEditError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error_type`](TranscriptEditErrorBuilder::error_type)
    /// - [`message`](TranscriptEditErrorBuilder::message)
    pub fn build(self) -> Result<TranscriptEditError, BuildError> {
        Ok(TranscriptEditError {
            error_type: self
                .error_type
                .ok_or_else(|| BuildError::missing_field("error_type"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
