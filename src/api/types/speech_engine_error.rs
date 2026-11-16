pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpeechEngineError {
    /// Human-readable description of the error.
    #[serde(default)]
    pub message: String,
}

impl SpeechEngineError {
    pub fn builder() -> SpeechEngineErrorBuilder {
        <SpeechEngineErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineErrorBuilder {
    message: Option<String>,
}

impl SpeechEngineErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SpeechEngineErrorBuilder::message)
    pub fn build(self) -> Result<SpeechEngineError, BuildError> {
        Ok(SpeechEngineError {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
