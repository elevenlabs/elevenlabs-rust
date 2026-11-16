pub use crate::prelude::*;

/// Query parameters for Get Convai Workspace Secrets
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SecretsListQueryRequest {
    /// How many documents to return at maximum. Can not exceed 100. If not provided, returns all secrets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Maximum number of dependent resources (tools, agents, phone numbers) to return per secret. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_limit: Option<i64>,
    /// If specified, returns only secrets whose names start with this string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SecretsListQueryRequest {
    pub fn builder() -> SecretsListQueryRequestBuilder {
        <SecretsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecretsListQueryRequestBuilder {
    page_size: Option<i64>,
    dependency_limit: Option<i64>,
    search: Option<String>,
    cursor: Option<String>,
}

impl SecretsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn dependency_limit(mut self, value: i64) -> Self {
        self.dependency_limit = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SecretsListQueryRequest`].
    pub fn build(self) -> Result<SecretsListQueryRequest, BuildError> {
        Ok(SecretsListQueryRequest {
            page_size: self.page_size,
            dependency_limit: self.dependency_limit,
            search: self.search,
            cursor: self.cursor,
        })
    }
}
