pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketListItemResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub subject: String,
    pub status: TicketStatus,
    pub priority: Option<TicketPriority>,
    pub requester: TicketRequesterResponse,
    pub assignee: Option<TicketListItemResponseAssignee>,
    pub secondary_assignee: Option<TicketListItemResponseSecondaryAssignee>,
    #[serde(default)]
    pub subscribers: Vec<TicketSubscriberResponse>,
    #[serde(default)]
    pub tag_ids: Vec<String>,
    pub sla: Option<TicketSlaSummaryResponse>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub deadline_at_unix_secs: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub updated_at_unix_secs: i64,
}

impl TicketListItemResponse {
    pub fn builder() -> TicketListItemResponseBuilder {
        <TicketListItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketListItemResponseBuilder {
    id: Option<String>,
    subject: Option<String>,
    status: Option<TicketStatus>,
    priority: Option<TicketPriority>,
    requester: Option<TicketRequesterResponse>,
    assignee: Option<TicketListItemResponseAssignee>,
    secondary_assignee: Option<TicketListItemResponseSecondaryAssignee>,
    subscribers: Option<Vec<TicketSubscriberResponse>>,
    tag_ids: Option<Vec<String>>,
    sla: Option<TicketSlaSummaryResponse>,
    deadline_at_unix_secs: Option<i64>,
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
}

impl TicketListItemResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn status(mut self, value: TicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn priority(mut self, value: TicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn requester(mut self, value: TicketRequesterResponse) -> Self {
        self.requester = Some(value);
        self
    }

    pub fn assignee(mut self, value: TicketListItemResponseAssignee) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn secondary_assignee(mut self, value: TicketListItemResponseSecondaryAssignee) -> Self {
        self.secondary_assignee = Some(value);
        self
    }

    pub fn subscribers(mut self, value: Vec<TicketSubscriberResponse>) -> Self {
        self.subscribers = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn sla(mut self, value: TicketSlaSummaryResponse) -> Self {
        self.sla = Some(value);
        self
    }

    pub fn deadline_at_unix_secs(mut self, value: i64) -> Self {
        self.deadline_at_unix_secs = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketListItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketListItemResponseBuilder::id)
    /// - [`subject`](TicketListItemResponseBuilder::subject)
    /// - [`status`](TicketListItemResponseBuilder::status)
    /// - [`requester`](TicketListItemResponseBuilder::requester)
    /// - [`subscribers`](TicketListItemResponseBuilder::subscribers)
    /// - [`tag_ids`](TicketListItemResponseBuilder::tag_ids)
    /// - [`created_at_unix_secs`](TicketListItemResponseBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](TicketListItemResponseBuilder::updated_at_unix_secs)
    pub fn build(self) -> Result<TicketListItemResponse, BuildError> {
        Ok(TicketListItemResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            priority: self.priority,
            requester: self
                .requester
                .ok_or_else(|| BuildError::missing_field("requester"))?,
            assignee: self.assignee,
            secondary_assignee: self.secondary_assignee,
            subscribers: self
                .subscribers
                .ok_or_else(|| BuildError::missing_field("subscribers"))?,
            tag_ids: self
                .tag_ids
                .ok_or_else(|| BuildError::missing_field("tag_ids"))?,
            sla: self.sla,
            deadline_at_unix_secs: self.deadline_at_unix_secs,
            created_at_unix_secs: self
                .created_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self
                .updated_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
        })
    }
}
