pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DownloadBadRequestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DownloadBadRequestResponse {
    pub fn builder() -> DownloadBadRequestResponseBuilder {
        <DownloadBadRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DownloadBadRequestResponseBuilder {
    error: Option<String>,
    message: Option<String>,
}

impl DownloadBadRequestResponseBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DownloadBadRequestResponse`].
    pub fn build(self) -> Result<DownloadBadRequestResponse, BuildError> {
        Ok(DownloadBadRequestResponse {
            error: self.error,
            message: self.message,
        })
    }
}
