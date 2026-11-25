pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingQuotaExceededError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingQuotaExceededError {
    pub fn builder() -> SubtitlingQuotaExceededErrorBuilder {
        <SubtitlingQuotaExceededErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingQuotaExceededErrorBuilder {
    error: Option<String>,
}

impl SubtitlingQuotaExceededErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingQuotaExceededError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingQuotaExceededErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingQuotaExceededError, BuildError> {
        Ok(SubtitlingQuotaExceededError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
