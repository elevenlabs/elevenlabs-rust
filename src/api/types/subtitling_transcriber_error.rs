pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingTranscriberError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingTranscriberError {
    pub fn builder() -> SubtitlingTranscriberErrorBuilder {
        <SubtitlingTranscriberErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingTranscriberErrorBuilder {
    error: Option<String>,
}

impl SubtitlingTranscriberErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingTranscriberError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingTranscriberErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingTranscriberError, BuildError> {
        Ok(SubtitlingTranscriberError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
