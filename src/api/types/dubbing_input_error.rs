pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingInputError {
    #[serde(default)]
    pub error: String,
}

impl DubbingInputError {
    pub fn builder() -> DubbingInputErrorBuilder {
        <DubbingInputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingInputErrorBuilder {
    error: Option<String>,
}

impl DubbingInputErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingInputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingInputErrorBuilder::error)
    pub fn build(self) -> Result<DubbingInputError, BuildError> {
        Ok(DubbingInputError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
