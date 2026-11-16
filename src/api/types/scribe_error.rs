pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeError {
    pub message_type: DubbingErrorPayloadMessageType,
    pub error: String,
}

impl ScribeError {
    pub fn builder() -> ScribeErrorBuilder {
        <ScribeErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeErrorBuilder {
    message_type: Option<DubbingErrorPayloadMessageType>,
    error: Option<String>,
}

impl ScribeErrorBuilder {
    pub fn message_type(mut self, value: DubbingErrorPayloadMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeErrorBuilder::message_type)
    /// - [`error`](ScribeErrorBuilder::error)
    pub fn build(self) -> Result<ScribeError, BuildError> {
        Ok(ScribeError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
