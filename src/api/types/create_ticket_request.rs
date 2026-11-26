pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTicketRequest {
    pub requester: Requester,
    pub author: CreateTicketRequestAuthor,
    #[serde(default)]
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TicketPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub deadline_at_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<CreateTicketRequestAssignee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_assignee: Option<CreateTicketRequestSecondaryAssignee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TicketSubscriberRequest>>,
}

impl CreateTicketRequest {
    pub fn builder() -> CreateTicketRequestBuilder {
        <CreateTicketRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTicketRequestBuilder {
    requester: Option<Requester>,
    author: Option<CreateTicketRequestAuthor>,
    subject: Option<String>,
    description: Option<String>,
    priority: Option<TicketPriority>,
    deadline_at_unix_secs: Option<i64>,
    tag_ids: Option<Vec<String>>,
    assignee: Option<CreateTicketRequestAssignee>,
    secondary_assignee: Option<CreateTicketRequestSecondaryAssignee>,
    subscribers: Option<Vec<TicketSubscriberRequest>>,
}

impl CreateTicketRequestBuilder {
    pub fn requester(mut self, value: Requester) -> Self {
        self.requester = Some(value);
        self
    }

    pub fn author(mut self, value: CreateTicketRequestAuthor) -> Self {
        self.author = Some(value);
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

    pub fn assignee(mut self, value: CreateTicketRequestAssignee) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn secondary_assignee(mut self, value: CreateTicketRequestSecondaryAssignee) -> Self {
        self.secondary_assignee = Some(value);
        self
    }

    pub fn subscribers(mut self, value: Vec<TicketSubscriberRequest>) -> Self {
        self.subscribers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTicketRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requester`](CreateTicketRequestBuilder::requester)
    /// - [`author`](CreateTicketRequestBuilder::author)
    /// - [`subject`](CreateTicketRequestBuilder::subject)
    pub fn build(self) -> Result<CreateTicketRequest, BuildError> {
        Ok(CreateTicketRequest {
            requester: self
                .requester
                .ok_or_else(|| BuildError::missing_field("requester"))?,
            author: self
                .author
                .ok_or_else(|| BuildError::missing_field("author"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            description: self.description,
            priority: self.priority,
            deadline_at_unix_secs: self.deadline_at_unix_secs,
            tag_ids: self.tag_ids,
            assignee: self.assignee,
            secondary_assignee: self.secondary_assignee,
            subscribers: self.subscribers,
        })
    }
}
