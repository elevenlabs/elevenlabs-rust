pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketDeadlineChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketDeadlineChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketDeadlineChangedEventResponseType,
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub before: Option<i64>,
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub after: Option<i64>,
}

impl TicketDeadlineChangedEventResponse {
    pub fn builder() -> TicketDeadlineChangedEventResponseBuilder {
        <TicketDeadlineChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketDeadlineChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketDeadlineChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketDeadlineChangedEventResponseType>,
    before: Option<i64>,
    after: Option<i64>,
}

impl TicketDeadlineChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketDeadlineChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketDeadlineChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: i64) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: i64) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketDeadlineChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketDeadlineChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketDeadlineChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketDeadlineChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketDeadlineChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketDeadlineChangedEventResponseBuilder::type)
    pub fn build(self) -> Result<TicketDeadlineChangedEventResponse, BuildError> {
        Ok(TicketDeadlineChangedEventResponse {
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
