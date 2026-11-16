pub use crate::prelude::*;

/// Query parameters for List Dubbing Projects
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectListQueryRequest {
    /// Pass the `next_cursor` from a previous response to fetch the page after it. Omit for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of projects per page. Clamped to between 1 and 100 rather than rejected, so a larger value returns a full page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter to projects in this status: `queued`, `preparing`, `ready`, or `failed`. Omit to return every status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Sort by creation time; newest first by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<ListSortDirectionQueryParameter2>,
}

impl ProjectListQueryRequest {
    pub fn builder() -> ProjectListQueryRequestBuilder {
        <ProjectListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<String>,
    sort_direction: Option<ListSortDirectionQueryParameter2>,
}

impl ProjectListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: ListSortDirectionQueryParameter2) -> Self {
        self.sort_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectListQueryRequest`].
    pub fn build(self) -> Result<ProjectListQueryRequest, BuildError> {
        Ok(ProjectListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
            sort_direction: self.sort_direction,
        })
    }
}
