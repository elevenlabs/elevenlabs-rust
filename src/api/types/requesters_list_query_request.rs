pub use crate::prelude::*;

/// Query parameters for List Support Requesters
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestersListQueryRequest {
    /// Only return requesters with these IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Number of requesters to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl RequestersListQueryRequest {
    pub fn builder() -> RequestersListQueryRequestBuilder {
        <RequestersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestersListQueryRequestBuilder {
    ids: Option<Vec<String>>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl RequestersListQueryRequestBuilder {
    pub fn ids(mut self, value: Vec<String>) -> Self {
        self.ids = Some(value);
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

    /// Consumes the builder and constructs a [`RequestersListQueryRequest`].
    pub fn build(self) -> Result<RequestersListQueryRequest, BuildError> {
        Ok(RequestersListQueryRequest {
            ids: self.ids,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
