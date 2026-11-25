pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingRateLimitedError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingRateLimitedError {
    pub fn builder() -> SubtitlingRateLimitedErrorBuilder {
        <SubtitlingRateLimitedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingRateLimitedErrorBuilder {
    error: Option<String>,
}

impl SubtitlingRateLimitedErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingRateLimitedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingRateLimitedErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingRateLimitedError, BuildError> {
        Ok(SubtitlingRateLimitedError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
