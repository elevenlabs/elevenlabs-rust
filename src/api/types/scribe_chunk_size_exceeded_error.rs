pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeChunkSizeExceededError {
    pub message_type: ScribeChunkSizeExceededErrorMessageType,
    pub error: String,
}

impl ScribeChunkSizeExceededError {
    pub fn builder() -> ScribeChunkSizeExceededErrorBuilder {
        <ScribeChunkSizeExceededErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeChunkSizeExceededErrorBuilder {
    message_type: Option<ScribeChunkSizeExceededErrorMessageType>,
    error: Option<String>,
}

impl ScribeChunkSizeExceededErrorBuilder {
    pub fn message_type(mut self, value: ScribeChunkSizeExceededErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeChunkSizeExceededError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeChunkSizeExceededErrorBuilder::message_type)
    /// - [`error`](ScribeChunkSizeExceededErrorBuilder::error)
    pub fn build(self) -> Result<ScribeChunkSizeExceededError, BuildError> {
        Ok(ScribeChunkSizeExceededError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
