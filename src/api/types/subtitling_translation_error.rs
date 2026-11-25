pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingTranslationError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingTranslationError {
    pub fn builder() -> SubtitlingTranslationErrorBuilder {
        <SubtitlingTranslationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingTranslationErrorBuilder {
    error: Option<String>,
}

impl SubtitlingTranslationErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingTranslationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingTranslationErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingTranslationError, BuildError> {
        Ok(SubtitlingTranslationError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
