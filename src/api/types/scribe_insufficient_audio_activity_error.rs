pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeInsufficientAudioActivityError {
    pub message_type: ScribeInsufficientAudioActivityErrorMessageType,
    pub error: String,
}

impl ScribeInsufficientAudioActivityError {
    pub fn builder() -> ScribeInsufficientAudioActivityErrorBuilder {
        <ScribeInsufficientAudioActivityErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeInsufficientAudioActivityErrorBuilder {
    message_type: Option<ScribeInsufficientAudioActivityErrorMessageType>,
    error: Option<String>,
}

impl ScribeInsufficientAudioActivityErrorBuilder {
    pub fn message_type(mut self, value: ScribeInsufficientAudioActivityErrorMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeInsufficientAudioActivityError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeInsufficientAudioActivityErrorBuilder::message_type)
    /// - [`error`](ScribeInsufficientAudioActivityErrorBuilder::error)
    pub fn build(self) -> Result<ScribeInsufficientAudioActivityError, BuildError> {
        Ok(ScribeInsufficientAudioActivityError {
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
