pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetTicketResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: Option<TicketPriority>,
    pub requester: TicketRequesterResponse,
    #[serde(default)]
    pub tag_ids: Vec<String>,
    pub assignee: Option<GetTicketResponseAssignee>,
    pub secondary_assignee: Option<GetTicketResponseSecondaryAssignee>,
    #[serde(default)]
    pub subscribers: Vec<TicketSubscriberResponse>,
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
    #[serde(default)]
    pub events: Vec<EventsItem>,
}

impl GetTicketResponse {
    pub fn builder() -> GetTicketResponseBuilder {
        <GetTicketResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTicketResponseBuilder {
    id: Option<String>,
    subject: Option<String>,
    description: Option<String>,
    status: Option<TicketStatus>,
    priority: Option<TicketPriority>,
    requester: Option<TicketRequesterResponse>,
    tag_ids: Option<Vec<String>>,
    assignee: Option<GetTicketResponseAssignee>,
    secondary_assignee: Option<GetTicketResponseSecondaryAssignee>,
    subscribers: Option<Vec<TicketSubscriberResponse>>,
    sla: Option<TicketSlaSummaryResponse>,
    deadline_at_unix_secs: Option<i64>,
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
    events: Option<Vec<EventsItem>>,
}

impl GetTicketResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn assignee(mut self, value: GetTicketResponseAssignee) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn secondary_assignee(mut self, value: GetTicketResponseSecondaryAssignee) -> Self {
        self.secondary_assignee = Some(value);
        self
    }

    pub fn subscribers(mut self, value: Vec<TicketSubscriberResponse>) -> Self {
        self.subscribers = Some(value);
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

    pub fn events(mut self, value: Vec<EventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTicketResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetTicketResponseBuilder::id)
    /// - [`subject`](GetTicketResponseBuilder::subject)
    /// - [`status`](GetTicketResponseBuilder::status)
    /// - [`requester`](GetTicketResponseBuilder::requester)
    /// - [`tag_ids`](GetTicketResponseBuilder::tag_ids)
    /// - [`subscribers`](GetTicketResponseBuilder::subscribers)
    /// - [`created_at_unix_secs`](GetTicketResponseBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](GetTicketResponseBuilder::updated_at_unix_secs)
    /// - [`events`](GetTicketResponseBuilder::events)
    pub fn build(self) -> Result<GetTicketResponse, BuildError> {
        Ok(GetTicketResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            description: self.description,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            priority: self.priority,
            requester: self
                .requester
                .ok_or_else(|| BuildError::missing_field("requester"))?,
            tag_ids: self
                .tag_ids
                .ok_or_else(|| BuildError::missing_field("tag_ids"))?,
            assignee: self.assignee,
            secondary_assignee: self.secondary_assignee,
            subscribers: self
                .subscribers
                .ok_or_else(|| BuildError::missing_field("subscribers"))?,
            sla: self.sla,
            deadline_at_unix_secs: self.deadline_at_unix_secs,
            created_at_unix_secs: self
                .created_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self
                .updated_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}
