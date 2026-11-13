pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingTranscriberError {
    #[serde(default)]
    pub error: String,
}

impl DubbingTranscriberError {
    pub fn builder() -> DubbingTranscriberErrorBuilder {
        <DubbingTranscriberErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriberErrorBuilder {
    error: Option<String>,
}

impl DubbingTranscriberErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriberError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingTranscriberErrorBuilder::error)
    pub fn build(self) -> Result<DubbingTranscriberError, BuildError> {
        Ok(DubbingTranscriberError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
