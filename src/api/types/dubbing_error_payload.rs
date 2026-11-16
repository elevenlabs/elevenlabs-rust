pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingErrorPayload {
    #[serde(default)]
    pub error: String,
}

impl DubbingErrorPayload {
    pub fn builder() -> DubbingErrorPayloadBuilder {
        <DubbingErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingErrorPayloadBuilder {
    error: Option<String>,
}

impl DubbingErrorPayloadBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](DubbingErrorPayloadBuilder::error)
    pub fn build(self) -> Result<DubbingErrorPayload, BuildError> {
        Ok(DubbingErrorPayload {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
