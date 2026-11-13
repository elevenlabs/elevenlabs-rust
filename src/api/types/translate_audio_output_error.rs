pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateAudioOutputError {
    #[serde(default)]
    pub error: String,
}

impl TranslateAudioOutputError {
    pub fn builder() -> TranslateAudioOutputErrorBuilder {
        <TranslateAudioOutputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateAudioOutputErrorBuilder {
    error: Option<String>,
}

impl TranslateAudioOutputErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateAudioOutputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslateAudioOutputErrorBuilder::error)
    pub fn build(self) -> Result<TranslateAudioOutputError, BuildError> {
        Ok(TranslateAudioOutputError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
