pub use crate::prelude::*;

/// Query parameters for List Outbound Campaigns
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OutboundCampaignsListQueryRequest {
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only campaigns in one of these statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<CampaignStatus>>,
    /// Only campaigns run by this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Only campaigns on this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<OutboundChannel>,
    /// Only voice or only message campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ListMode>,
    /// Only campaigns whose name contains this text, or whose ID is exactly this text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl OutboundCampaignsListQueryRequest {
    pub fn builder() -> OutboundCampaignsListQueryRequestBuilder {
        <OutboundCampaignsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundCampaignsListQueryRequestBuilder {
    page_size: Option<i64>,
    status: Option<Vec<CampaignStatus>>,
    agent_id: Option<String>,
    channel: Option<OutboundChannel>,
    mode: Option<ListMode>,
    search: Option<String>,
    cursor: Option<String>,
}

impl OutboundCampaignsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<CampaignStatus>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn channel(mut self, value: OutboundChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn mode(mut self, value: ListMode) -> Self {
        self.mode = Some(value);
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

    /// Consumes the builder and constructs a [`OutboundCampaignsListQueryRequest`].
    pub fn build(self) -> Result<OutboundCampaignsListQueryRequest, BuildError> {
        Ok(OutboundCampaignsListQueryRequest {
            page_size: self.page_size,
            status: self.status,
            agent_id: self.agent_id,
            channel: self.channel,
            mode: self.mode,
            search: self.search,
            cursor: self.cursor,
        })
    }
}
