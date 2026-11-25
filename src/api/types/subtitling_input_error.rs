pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingInputError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingInputError {
    pub fn builder() -> SubtitlingInputErrorBuilder {
        <SubtitlingInputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingInputErrorBuilder {
    error: Option<String>,
}

impl SubtitlingInputErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingInputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingInputErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingInputError, BuildError> {
        Ok(SubtitlingInputError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
