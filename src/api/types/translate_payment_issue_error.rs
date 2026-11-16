pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslatePaymentIssueError {
    #[serde(default)]
    pub error: String,
}

impl TranslatePaymentIssueError {
    pub fn builder() -> TranslatePaymentIssueErrorBuilder {
        <TranslatePaymentIssueErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslatePaymentIssueErrorBuilder {
    error: Option<String>,
}

impl TranslatePaymentIssueErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslatePaymentIssueError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TranslatePaymentIssueErrorBuilder::error)
    pub fn build(self) -> Result<TranslatePaymentIssueError, BuildError> {
        Ok(TranslatePaymentIssueError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
