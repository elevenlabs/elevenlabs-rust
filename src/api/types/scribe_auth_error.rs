pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeAuthError {
    pub message_type: DubbingAuthErrorMessageType,
    pub error: String,
}

impl ScribeAuthError {
    pub fn builder() -> ScribeAuthErrorBuilder {
        <ScribeAuthErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeAuthErrorBuilder {
    message_type: Option<DubbingAuthErrorMessageType>,
    error: Option<String>,
}

impl ScribeAuthErrorBuilder {
    pub fn message_type(mut self, value: DubbingAuthErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeAuthError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeAuthErrorBuilder::message_type)
    /// - [`error`](ScribeAuthErrorBuilder::error)
    pub fn build(self) -> Result<ScribeAuthError, BuildError> {
        Ok(ScribeAuthError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
