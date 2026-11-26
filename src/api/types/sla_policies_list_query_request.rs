pub use crate::prelude::*;

/// Query parameters for List Ticket Sla Policies
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SlaPoliciesListQueryRequest {
    /// Whether to include archived SLA policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// Number of ticket SLA policies to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SlaPoliciesListQueryRequest {
    pub fn builder() -> SlaPoliciesListQueryRequestBuilder {
        <SlaPoliciesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlaPoliciesListQueryRequestBuilder {
    include_archived: Option<bool>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl SlaPoliciesListQueryRequestBuilder {
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SlaPoliciesListQueryRequest`].
    pub fn build(self) -> Result<SlaPoliciesListQueryRequest, BuildError> {
        Ok(SlaPoliciesListQueryRequest {
            include_archived: self.include_archived,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
