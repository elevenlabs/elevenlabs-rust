pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateTicketRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TicketStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TicketPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub deadline_at_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UpdateTicketRequestAssignee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_assignee: Option<UpdateTicketRequestSecondaryAssignee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TicketSubscriberRequest>>,
}

impl UpdateTicketRequest {
    pub fn builder() -> UpdateTicketRequestBuilder {
        <UpdateTicketRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTicketRequestBuilder {
    subject: Option<String>,
    status: Option<TicketStatus>,
    priority: Option<TicketPriority>,
    deadline_at_unix_secs: Option<i64>,
    tag_ids: Option<Vec<String>>,
    assignee: Option<UpdateTicketRequestAssignee>,
    secondary_assignee: Option<UpdateTicketRequestSecondaryAssignee>,
    subscribers: Option<Vec<TicketSubscriberRequest>>,
}

impl UpdateTicketRequestBuilder {
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

    pub fn deadline_at_unix_secs(mut self, value: i64) -> Self {
        self.deadline_at_unix_secs = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn assignee(mut self, value: UpdateTicketRequestAssignee) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn secondary_assignee(mut self, value: UpdateTicketRequestSecondaryAssignee) -> Self {
        self.secondary_assignee = Some(value);
        self
    }

    pub fn subscribers(mut self, value: Vec<TicketSubscriberRequest>) -> Self {
        self.subscribers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTicketRequest`].
    pub fn build(self) -> Result<UpdateTicketRequest, BuildError> {
        Ok(UpdateTicketRequest {
            subject: self.subject,
            status: self.status,
            priority: self.priority,
            deadline_at_unix_secs: self.deadline_at_unix_secs,
            tag_ids: self.tag_ids,
            assignee: self.assignee,
            secondary_assignee: self.secondary_assignee,
            subscribers: self.subscribers,
        })
    }
}
