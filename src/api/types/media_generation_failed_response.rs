pub use crate::prelude::*;

/// A failed media generation and why it failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MediaGenerationFailedResponse {
    /// The unique identifier of the generation.
    #[serde(default)]
    pub id: String,
    /// The category of failure.
    pub failure_reason: FailureReason,
    /// A human-readable description of the failure. Failed generations are not charged.
    #[serde(default)]
    pub error_message: String,
}

impl MediaGenerationFailedResponse {
    pub fn builder() -> MediaGenerationFailedResponseBuilder {
        <MediaGenerationFailedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaGenerationFailedResponseBuilder {
    id: Option<String>,
    failure_reason: Option<FailureReason>,
    error_message: Option<String>,
}

impl MediaGenerationFailedResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn failure_reason(mut self, value: FailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaGenerationFailedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MediaGenerationFailedResponseBuilder::id)
    /// - [`failure_reason`](MediaGenerationFailedResponseBuilder::failure_reason)
    /// - [`error_message`](MediaGenerationFailedResponseBuilder::error_message)
    pub fn build(self) -> Result<MediaGenerationFailedResponse, BuildError> {
        Ok(MediaGenerationFailedResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            failure_reason: self
                .failure_reason
                .ok_or_else(|| BuildError::missing_field("failure_reason"))?,
            error_message: self
                .error_message
                .ok_or_else(|| BuildError::missing_field("error_message"))?,
        })
    }
}
