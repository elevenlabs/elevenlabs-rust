pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingRateLimitedError {
    #[serde(default)]
    pub error: String,
}

impl DubbingRateLimitedError {
    pub fn builder() -> DubbingRateLimitedErrorBuilder {
        <DubbingRateLimitedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingRateLimitedErrorBuilder {
    error: Option<String>,
}

impl DubbingRateLimitedErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingRateLimitedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingRateLimitedErrorBuilder::error)
    pub fn build(self) -> Result<DubbingRateLimitedError, BuildError> {
        Ok(DubbingRateLimitedError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
