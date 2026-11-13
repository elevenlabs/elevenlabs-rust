pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingTranslationError {
    #[serde(default)]
    pub error: String,
}

impl DubbingTranslationError {
    pub fn builder() -> DubbingTranslationErrorBuilder {
        <DubbingTranslationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranslationErrorBuilder {
    error: Option<String>,
}

impl DubbingTranslationErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranslationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingTranslationErrorBuilder::error)
    pub fn build(self) -> Result<DubbingTranslationError, BuildError> {
        Ok(DubbingTranslationError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
