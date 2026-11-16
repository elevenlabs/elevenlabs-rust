pub use crate::prelude::*;

/// Query parameters for List Outbound Campaign Calls
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CallsListQueryRequest {
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only this lead's calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_id: Option<String>,
    /// Only calls in this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CallStatus>,
    /// Only calls that ended in one of these results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_result: Option<Vec<CallResult>>,
    /// Only messages at this delivery stage. Message campaigns only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<ListStage>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl CallsListQueryRequest {
    pub fn builder() -> CallsListQueryRequestBuilder {
        <CallsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CallsListQueryRequestBuilder {
    page_size: Option<i64>,
    lead_id: Option<String>,
    status: Option<CallStatus>,
    call_result: Option<Vec<CallResult>>,
    stage: Option<ListStage>,
    cursor: Option<String>,
}

impl CallsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CallStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn call_result(mut self, value: Vec<CallResult>) -> Self {
        self.call_result = Some(value);
        self
    }

    pub fn stage(mut self, value: ListStage) -> Self {
        self.stage = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CallsListQueryRequest`].
    pub fn build(self) -> Result<CallsListQueryRequest, BuildError> {
        Ok(CallsListQueryRequest {
            page_size: self.page_size,
            lead_id: self.lead_id,
            status: self.status,
            call_result: self.call_result,
            stage: self.stage,
            cursor: self.cursor,
        })
    }
}
