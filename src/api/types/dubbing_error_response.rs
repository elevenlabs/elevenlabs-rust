pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingErrorResponse {
    /// Stable identifier for the failure, safe to branch on. New codes are added over time, so we recommend treating an unrecognized value as `internal_error`.
    #[serde(default)]
    pub code: String,
    /// Human-readable description of the failure, for display. The wording may change at any time, so we recommend branching on `code` instead.
    #[serde(default)]
    pub message: String,
    /// Whether resubmitting the same input could succeed. A value of false means the failure describes the input or the account, so an identical retry will fail the same way.
    #[serde(default)]
    pub retryable: bool,
}

impl DubbingErrorResponse {
    pub fn builder() -> DubbingErrorResponseBuilder {
        <DubbingErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingErrorResponseBuilder {
    code: Option<String>,
    message: Option<String>,
    retryable: Option<bool>,
}

impl DubbingErrorResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](DubbingErrorResponseBuilder::code)
    /// - [`message`](DubbingErrorResponseBuilder::message)
    /// - [`retryable`](DubbingErrorResponseBuilder::retryable)
    pub fn build(self) -> Result<DubbingErrorResponse, BuildError> {
        Ok(DubbingErrorResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            retryable: self
                .retryable
                .ok_or_else(|| BuildError::missing_field("retryable"))?,
        })
    }
}
