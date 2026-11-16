pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeUnacceptedTermsError {
    pub message_type: ScribeUnacceptedTermsErrorMessageType,
    pub error: String,
}

impl ScribeUnacceptedTermsError {
    pub fn builder() -> ScribeUnacceptedTermsErrorBuilder {
        <ScribeUnacceptedTermsErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeUnacceptedTermsErrorBuilder {
    message_type: Option<ScribeUnacceptedTermsErrorMessageType>,
    error: Option<String>,
}

impl ScribeUnacceptedTermsErrorBuilder {
    pub fn message_type(mut self, value: ScribeUnacceptedTermsErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeUnacceptedTermsError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeUnacceptedTermsErrorBuilder::message_type)
    /// - [`error`](ScribeUnacceptedTermsErrorBuilder::error)
    pub fn build(self) -> Result<ScribeUnacceptedTermsError, BuildError> {
        Ok(ScribeUnacceptedTermsError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
