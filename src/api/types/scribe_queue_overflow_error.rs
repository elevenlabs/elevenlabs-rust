pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeQueueOverflowError {
    pub message_type: DubbingQueueOverflowErrorMessageType,
    pub error: String,
}

impl ScribeQueueOverflowError {
    pub fn builder() -> ScribeQueueOverflowErrorBuilder {
        <ScribeQueueOverflowErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeQueueOverflowErrorBuilder {
    message_type: Option<DubbingQueueOverflowErrorMessageType>,
    error: Option<String>,
}

impl ScribeQueueOverflowErrorBuilder {
    pub fn message_type(mut self, value: DubbingQueueOverflowErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeQueueOverflowError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeQueueOverflowErrorBuilder::message_type)
    /// - [`error`](ScribeQueueOverflowErrorBuilder::error)
    pub fn build(self) -> Result<ScribeQueueOverflowError, BuildError> {
        Ok(ScribeQueueOverflowError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
