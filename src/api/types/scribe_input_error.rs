pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeInputError {
    pub message_type: DubbingInputErrorMessageType,
    pub error: String,
}

impl ScribeInputError {
    pub fn builder() -> ScribeInputErrorBuilder {
        <ScribeInputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeInputErrorBuilder {
    message_type: Option<DubbingInputErrorMessageType>,
    error: Option<String>,
}

impl ScribeInputErrorBuilder {
    pub fn message_type(mut self, value: DubbingInputErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeInputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeInputErrorBuilder::message_type)
    /// - [`error`](ScribeInputErrorBuilder::error)
    pub fn build(self) -> Result<ScribeInputError, BuildError> {
        Ok(ScribeInputError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
