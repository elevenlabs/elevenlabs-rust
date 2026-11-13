pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignProgressResponse {
    pub leads: LeadsProgress,
    pub conversations: ConversationProgress,
    /// When the next call is due; unset when none is scheduled.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub next_call_time_unix: Option<i64>,
}

impl CampaignProgressResponse {
    pub fn builder() -> CampaignProgressResponseBuilder {
        <CampaignProgressResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignProgressResponseBuilder {
    leads: Option<LeadsProgress>,
    conversations: Option<ConversationProgress>,
    next_call_time_unix: Option<i64>,
}

impl CampaignProgressResponseBuilder {
    pub fn leads(mut self, value: LeadsProgress) -> Self {
        self.leads = Some(value);
        self
    }

    pub fn conversations(mut self, value: ConversationProgress) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn next_call_time_unix(mut self, value: i64) -> Self {
        self.next_call_time_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignProgressResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`leads`](CampaignProgressResponseBuilder::leads)
    /// - [`conversations`](CampaignProgressResponseBuilder::conversations)
    pub fn build(self) -> Result<CampaignProgressResponse, BuildError> {
        Ok(CampaignProgressResponse {
            leads: self
                .leads
                .ok_or_else(|| BuildError::missing_field("leads"))?,
            conversations: self
                .conversations
                .ok_or_else(|| BuildError::missing_field("conversations"))?,
            next_call_time_unix: self.next_call_time_unix,
        })
    }
}
