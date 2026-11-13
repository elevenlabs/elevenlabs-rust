pub use crate::prelude::*;

/// Query parameters for Smart Search Conversation Messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessagesSearchQueryRequest {
    /// The search query text for semantic similarity matching
    pub text_query: String,
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter conversations by branch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Number of results per page. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl MessagesSearchQueryRequest {
    pub fn builder() -> MessagesSearchQueryRequestBuilder {
        <MessagesSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagesSearchQueryRequestBuilder {
    text_query: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl MessagesSearchQueryRequestBuilder {
    pub fn text_query(mut self, value: impl Into<String>) -> Self {
        self.text_query = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`MessagesSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text_query`](MessagesSearchQueryRequestBuilder::text_query)
    pub fn build(self) -> Result<MessagesSearchQueryRequest, BuildError> {
        Ok(MessagesSearchQueryRequest {
            text_query: self
                .text_query
                .ok_or_else(|| BuildError::missing_field("text_query"))?,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
