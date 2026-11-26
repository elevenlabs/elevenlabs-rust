pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketPriorityChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketPriorityChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketPriorityChangedEventResponseType,
    pub before: Option<TicketPriority>,
    pub after: Option<TicketPriority>,
}

impl TicketPriorityChangedEventResponse {
    pub fn builder() -> TicketPriorityChangedEventResponseBuilder {
        <TicketPriorityChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketPriorityChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketPriorityChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketPriorityChangedEventResponseType>,
    before: Option<TicketPriority>,
    after: Option<TicketPriority>,
}

impl TicketPriorityChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketPriorityChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketPriorityChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: TicketPriority) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: TicketPriority) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketPriorityChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketPriorityChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketPriorityChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketPriorityChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketPriorityChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketPriorityChangedEventResponseBuilder::type)
    pub fn build(self) -> Result<TicketPriorityChangedEventResponse, BuildError> {
        Ok(TicketPriorityChangedEventResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
            author: self
                .author
                .ok_or_else(|| BuildError::missing_field("author"))?,
            created_at_unix_secs: self
                .created_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            before: self.before,
            after: self.after,
        })
    }
}
