pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingError {
    pub fn builder() -> SubtitlingErrorBuilder {
        <SubtitlingErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingErrorBuilder {
    error: Option<String>,
}

impl SubtitlingErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingError, BuildError> {
        Ok(SubtitlingError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
