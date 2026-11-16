pub use crate::prelude::*;

/// Query parameters for List Outbound Campaign Attention Items
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AttentionListQueryRequest {
    /// Only this campaign's items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// Include items that are already resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resolved: Option<bool>,
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AttentionListQueryRequest {
    pub fn builder() -> AttentionListQueryRequestBuilder {
        <AttentionListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttentionListQueryRequestBuilder {
    campaign_id: Option<String>,
    include_resolved: Option<bool>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AttentionListQueryRequestBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn include_resolved(mut self, value: bool) -> Self {
        self.include_resolved = Some(value);
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

    /// Consumes the builder and constructs a [`AttentionListQueryRequest`].
    pub fn build(self) -> Result<AttentionListQueryRequest, BuildError> {
        Ok(AttentionListQueryRequest {
            campaign_id: self.campaign_id,
            include_resolved: self.include_resolved,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
