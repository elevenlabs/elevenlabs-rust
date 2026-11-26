pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketStatusChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketStatusChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketStatusChangedEventResponseType,
    pub before: TicketStatus,
    pub after: TicketStatus,
}

impl TicketStatusChangedEventResponse {
    pub fn builder() -> TicketStatusChangedEventResponseBuilder {
        <TicketStatusChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketStatusChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketStatusChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketStatusChangedEventResponseType>,
    before: Option<TicketStatus>,
    after: Option<TicketStatus>,
}

impl TicketStatusChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketStatusChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketStatusChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: TicketStatus) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: TicketStatus) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketStatusChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketStatusChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketStatusChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketStatusChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketStatusChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketStatusChangedEventResponseBuilder::type)
    /// - [`before`](TicketStatusChangedEventResponseBuilder::before)
    /// - [`after`](TicketStatusChangedEventResponseBuilder::after)
    pub fn build(self) -> Result<TicketStatusChangedEventResponse, BuildError> {
        Ok(TicketStatusChangedEventResponse {
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
            before: self
                .before
                .ok_or_else(|| BuildError::missing_field("before"))?,
            after: self
                .after
                .ok_or_else(|| BuildError::missing_field("after"))?,
        })
    }
}
