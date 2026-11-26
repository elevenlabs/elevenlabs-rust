pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketSubscribersChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketSubscribersChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketSubscribersChangedEventResponseType,
    pub before: Vec<TicketSubscriberResponse>,
    pub after: Vec<TicketSubscriberResponse>,
}

impl TicketSubscribersChangedEventResponse {
    pub fn builder() -> TicketSubscribersChangedEventResponseBuilder {
        <TicketSubscribersChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSubscribersChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketSubscribersChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketSubscribersChangedEventResponseType>,
    before: Option<Vec<TicketSubscriberResponse>>,
    after: Option<Vec<TicketSubscriberResponse>>,
}

impl TicketSubscribersChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketSubscribersChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketSubscribersChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: Vec<TicketSubscriberResponse>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: Vec<TicketSubscriberResponse>) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketSubscribersChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketSubscribersChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketSubscribersChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketSubscribersChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketSubscribersChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketSubscribersChangedEventResponseBuilder::type)
    /// - [`before`](TicketSubscribersChangedEventResponseBuilder::before)
    /// - [`after`](TicketSubscribersChangedEventResponseBuilder::after)
    pub fn build(self) -> Result<TicketSubscribersChangedEventResponse, BuildError> {
        Ok(TicketSubscribersChangedEventResponse {
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
