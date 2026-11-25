pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingAuthError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingAuthError {
    pub fn builder() -> SubtitlingAuthErrorBuilder {
        <SubtitlingAuthErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingAuthErrorBuilder {
    error: Option<String>,
}

impl SubtitlingAuthErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingAuthError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingAuthErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingAuthError, BuildError> {
        Ok(SubtitlingAuthError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
