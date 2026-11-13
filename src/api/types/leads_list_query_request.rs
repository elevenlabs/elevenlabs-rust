pub use crate::prelude::*;

/// Query parameters for List Outbound Campaign Leads
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LeadsListQueryRequest {
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only leads whose ID, user ID or phone number contains this text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// `in_progress` for leads still being called, `finished` for leads that won't be called again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListStatusQueryParameter2>,
    /// Only leads that finished in one of these ways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<Vec<TerminalDisposition>>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl LeadsListQueryRequest {
    pub fn builder() -> LeadsListQueryRequestBuilder {
        <LeadsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadsListQueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    status: Option<ListStatusQueryParameter2>,
    disposition: Option<Vec<TerminalDisposition>>,
    cursor: Option<String>,
}

impl LeadsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListStatusQueryParameter2) -> Self {
        self.status = Some(value);
        self
    }

    pub fn disposition(mut self, value: Vec<TerminalDisposition>) -> Self {
        self.disposition = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadsListQueryRequest`].
    pub fn build(self) -> Result<LeadsListQueryRequest, BuildError> {
        Ok(LeadsListQueryRequest {
            page_size: self.page_size,
            search: self.search,
            status: self.status,
            disposition: self.disposition,
            cursor: self.cursor,
        })
    }
}
