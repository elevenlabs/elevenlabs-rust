pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeTranscriberError {
    pub message_type: DubbingTranscriberErrorMessageType,
    pub error: String,
}

impl ScribeTranscriberError {
    pub fn builder() -> ScribeTranscriberErrorBuilder {
        <ScribeTranscriberErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeTranscriberErrorBuilder {
    message_type: Option<DubbingTranscriberErrorMessageType>,
    error: Option<String>,
}

impl ScribeTranscriberErrorBuilder {
    pub fn message_type(mut self, value: DubbingTranscriberErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeTranscriberError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeTranscriberErrorBuilder::message_type)
    /// - [`error`](ScribeTranscriberErrorBuilder::error)
    pub fn build(self) -> Result<ScribeTranscriberError, BuildError> {
        Ok(ScribeTranscriberError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
