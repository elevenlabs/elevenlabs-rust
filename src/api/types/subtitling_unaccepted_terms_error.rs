pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingUnacceptedTermsError {
    #[serde(default)]
    pub error: String,
}

impl SubtitlingUnacceptedTermsError {
    pub fn builder() -> SubtitlingUnacceptedTermsErrorBuilder {
        <SubtitlingUnacceptedTermsErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingUnacceptedTermsErrorBuilder {
    error: Option<String>,
}

impl SubtitlingUnacceptedTermsErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingUnacceptedTermsError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SubtitlingUnacceptedTermsErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingUnacceptedTermsError, BuildError> {
        Ok(SubtitlingUnacceptedTermsError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
