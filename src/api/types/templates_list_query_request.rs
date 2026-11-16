pub use crate::prelude::*;

/// Query parameters for List Templates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplatesListQueryRequest {
    /// Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many templates to return per page. Lower than the run list's ceiling because each row expands its versions' input and output schemas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// How many of each template's published versions to return, newest first. `has_more_versions` tells you when a template has more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions_per_template: Option<i64>,
    /// Only return templates whose name or description contains this text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl TemplatesListQueryRequest {
    pub fn builder() -> TemplatesListQueryRequestBuilder {
        <TemplatesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplatesListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    versions_per_template: Option<i64>,
    search: Option<String>,
}

impl TemplatesListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn versions_per_template(mut self, value: i64) -> Self {
        self.versions_per_template = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TemplatesListQueryRequest`].
    pub fn build(self) -> Result<TemplatesListQueryRequest, BuildError> {
        Ok(TemplatesListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            versions_per_template: self.versions_per_template,
            search: self.search,
        })
    }
}
