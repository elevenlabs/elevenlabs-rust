pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketSubjectChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketSubjectChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketSubjectChangedEventResponseType,
    pub before: String,
    pub after: String,
}

impl TicketSubjectChangedEventResponse {
    pub fn builder() -> TicketSubjectChangedEventResponseBuilder {
        <TicketSubjectChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSubjectChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketSubjectChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketSubjectChangedEventResponseType>,
    before: Option<String>,
    after: Option<String>,
}

impl TicketSubjectChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketSubjectChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketSubjectChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketSubjectChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketSubjectChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketSubjectChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketSubjectChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketSubjectChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketSubjectChangedEventResponseBuilder::type)
    /// - [`before`](TicketSubjectChangedEventResponseBuilder::before)
    /// - [`after`](TicketSubjectChangedEventResponseBuilder::after)
    pub fn build(self) -> Result<TicketSubjectChangedEventResponse, BuildError> {
        Ok(TicketSubjectChangedEventResponse {
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
