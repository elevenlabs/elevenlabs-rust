pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeRateLimitedError {
    pub message_type: DubbingRateLimitedErrorMessageType,
    pub error: String,
}

impl ScribeRateLimitedError {
    pub fn builder() -> ScribeRateLimitedErrorBuilder {
        <ScribeRateLimitedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeRateLimitedErrorBuilder {
    message_type: Option<DubbingRateLimitedErrorMessageType>,
    error: Option<String>,
}

impl ScribeRateLimitedErrorBuilder {
    pub fn message_type(mut self, value: DubbingRateLimitedErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeRateLimitedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeRateLimitedErrorBuilder::message_type)
    /// - [`error`](ScribeRateLimitedErrorBuilder::error)
    pub fn build(self) -> Result<ScribeRateLimitedError, BuildError> {
        Ok(ScribeRateLimitedError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
