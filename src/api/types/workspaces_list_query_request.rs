pub use crate::prelude::*;

/// Query parameters for List Reseller Workspaces
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspacesListQueryRequest {
    /// Pagination cursor from previous response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of workspaces per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl WorkspacesListQueryRequest {
    pub fn builder() -> WorkspacesListQueryRequestBuilder {
        <WorkspacesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspacesListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
}

impl WorkspacesListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspacesListQueryRequest`].
    pub fn build(self) -> Result<WorkspacesListQueryRequest, BuildError> {
        Ok(WorkspacesListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
        })
    }
}
