pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketTagIdsChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketTagIdsChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketTagIdsChangedEventResponseType,
    pub before: Vec<String>,
    pub after: Vec<String>,
}

impl TicketTagIdsChangedEventResponse {
    pub fn builder() -> TicketTagIdsChangedEventResponseBuilder {
        <TicketTagIdsChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketTagIdsChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketTagIdsChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketTagIdsChangedEventResponseType>,
    before: Option<Vec<String>>,
    after: Option<Vec<String>>,
}

impl TicketTagIdsChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketTagIdsChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketTagIdsChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: Vec<String>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: Vec<String>) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketTagIdsChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketTagIdsChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketTagIdsChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketTagIdsChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketTagIdsChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketTagIdsChangedEventResponseBuilder::type)
    /// - [`before`](TicketTagIdsChangedEventResponseBuilder::before)
    /// - [`after`](TicketTagIdsChangedEventResponseBuilder::after)
    pub fn build(self) -> Result<TicketTagIdsChangedEventResponse, BuildError> {
        Ok(TicketTagIdsChangedEventResponse {
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
