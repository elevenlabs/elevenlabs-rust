pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingQueueOverflowError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingQueueOverflowError {
    pub fn builder() -> SubtitlingQueueOverflowErrorBuilder {
        <SubtitlingQueueOverflowErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingQueueOverflowErrorBuilder {
    error: Option<String>,
}

impl SubtitlingQueueOverflowErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingQueueOverflowError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingQueueOverflowErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingQueueOverflowError, BuildError> {
        Ok(SubtitlingQueueOverflowError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
