pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingPaymentIssueError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingPaymentIssueError {
    pub fn builder() -> SubtitlingPaymentIssueErrorBuilder {
        <SubtitlingPaymentIssueErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingPaymentIssueErrorBuilder {
    error: Option<String>,
}

impl SubtitlingPaymentIssueErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingPaymentIssueError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingPaymentIssueErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingPaymentIssueError, BuildError> {
        Ok(SubtitlingPaymentIssueError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
