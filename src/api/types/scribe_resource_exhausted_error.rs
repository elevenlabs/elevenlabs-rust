pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeResourceExhaustedError {
    pub message_type: ScribeResourceExhaustedErrorMessageType,
    pub error: String,
}

impl ScribeResourceExhaustedError {
    pub fn builder() -> ScribeResourceExhaustedErrorBuilder {
        <ScribeResourceExhaustedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeResourceExhaustedErrorBuilder {
    message_type: Option<ScribeResourceExhaustedErrorMessageType>,
    error: Option<String>,
}

impl ScribeResourceExhaustedErrorBuilder {
    pub fn message_type(mut self, value: ScribeResourceExhaustedErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeResourceExhaustedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeResourceExhaustedErrorBuilder::message_type)
    /// - [`error`](ScribeResourceExhaustedErrorBuilder::error)
    pub fn build(self) -> Result<ScribeResourceExhaustedError, BuildError> {
        Ok(ScribeResourceExhaustedError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
