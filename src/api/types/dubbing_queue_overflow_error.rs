pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingQueueOverflowError {
    #[serde(default)]
    pub error: String,
}

impl DubbingQueueOverflowError {
    pub fn builder() -> DubbingQueueOverflowErrorBuilder {
        <DubbingQueueOverflowErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingQueueOverflowErrorBuilder {
    error: Option<String>,
}

impl DubbingQueueOverflowErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingQueueOverflowError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingQueueOverflowErrorBuilder::error)
    pub fn build(self) -> Result<DubbingQueueOverflowError, BuildError> {
        Ok(DubbingQueueOverflowError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
