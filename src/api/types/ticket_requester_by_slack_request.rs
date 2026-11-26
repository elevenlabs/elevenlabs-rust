pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketRequesterBySlackRequest {
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

impl TicketRequesterBySlackRequest {
    pub fn builder() -> TicketRequesterBySlackRequestBuilder {
        <TicketRequesterBySlackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketRequesterBySlackRequestBuilder {
    team_id: Option<String>,
    user_id: Option<String>,
    full_name: Option<String>,
}

impl TicketRequesterBySlackRequestBuilder {
    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketRequesterBySlackRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](TicketRequesterBySlackRequestBuilder::team_id)
    /// - [`user_id`](TicketRequesterBySlackRequestBuilder::user_id)
    pub fn build(self) -> Result<TicketRequesterBySlackRequest, BuildError> {
        Ok(TicketRequesterBySlackRequest {
            team_id: self
                .team_id
                .ok_or_else(|| BuildError::missing_field("team_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            full_name: self.full_name,
        })
    }
}
