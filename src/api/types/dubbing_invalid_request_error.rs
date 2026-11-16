pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingInvalidRequestError {
    #[serde(default)]
    pub error: String,
}

impl DubbingInvalidRequestError {
    pub fn builder() -> DubbingInvalidRequestErrorBuilder {
        <DubbingInvalidRequestErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingInvalidRequestErrorBuilder {
    error: Option<String>,
}

impl DubbingInvalidRequestErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingInvalidRequestError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingInvalidRequestErrorBuilder::error)
    pub fn build(self) -> Result<DubbingInvalidRequestError, BuildError> {
        Ok(DubbingInvalidRequestError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
