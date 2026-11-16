pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeSessionTimeLimitExceededError {
    pub message_type: ScribeSessionTimeLimitExceededErrorMessageType,
    pub error: String,
}

impl ScribeSessionTimeLimitExceededError {
    pub fn builder() -> ScribeSessionTimeLimitExceededErrorBuilder {
        <ScribeSessionTimeLimitExceededErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeSessionTimeLimitExceededErrorBuilder {
    message_type: Option<ScribeSessionTimeLimitExceededErrorMessageType>,
    error: Option<String>,
}

impl ScribeSessionTimeLimitExceededErrorBuilder {
    pub fn message_type(mut self, value: ScribeSessionTimeLimitExceededErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeSessionTimeLimitExceededError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeSessionTimeLimitExceededErrorBuilder::message_type)
    /// - [`error`](ScribeSessionTimeLimitExceededErrorBuilder::error)
    pub fn build(self) -> Result<ScribeSessionTimeLimitExceededError, BuildError> {
        Ok(ScribeSessionTimeLimitExceededError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
