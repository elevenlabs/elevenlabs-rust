pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeQuotaExceededError {
    pub message_type: ScribeQuotaExceededErrorMessageType,
    pub error: String,
}

impl ScribeQuotaExceededError {
    pub fn builder() -> ScribeQuotaExceededErrorBuilder {
        <ScribeQuotaExceededErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeQuotaExceededErrorBuilder {
    message_type: Option<ScribeQuotaExceededErrorMessageType>,
    error: Option<String>,
}

impl ScribeQuotaExceededErrorBuilder {
    pub fn message_type(mut self, value: ScribeQuotaExceededErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeQuotaExceededError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeQuotaExceededErrorBuilder::message_type)
    /// - [`error`](ScribeQuotaExceededErrorBuilder::error)
    pub fn build(self) -> Result<ScribeQuotaExceededError, BuildError> {
        Ok(ScribeQuotaExceededError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
