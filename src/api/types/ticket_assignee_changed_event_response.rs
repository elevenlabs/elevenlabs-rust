pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketAssigneeChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketAssigneeChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketAssigneeChangedEventResponseType,
    pub before: Option<TicketAssigneeChangedEventResponseBefore>,
    pub after: Option<TicketAssigneeChangedEventResponseAfter>,
}

impl TicketAssigneeChangedEventResponse {
    pub fn builder() -> TicketAssigneeChangedEventResponseBuilder {
        <TicketAssigneeChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketAssigneeChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketAssigneeChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketAssigneeChangedEventResponseType>,
    before: Option<TicketAssigneeChangedEventResponseBefore>,
    after: Option<TicketAssigneeChangedEventResponseAfter>,
}

impl TicketAssigneeChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketAssigneeChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketAssigneeChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: TicketAssigneeChangedEventResponseBefore) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: TicketAssigneeChangedEventResponseAfter) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketAssigneeChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketAssigneeChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketAssigneeChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketAssigneeChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketAssigneeChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketAssigneeChangedEventResponseBuilder::type)
    pub fn build(self) -> Result<TicketAssigneeChangedEventResponse, BuildError> {
        Ok(TicketAssigneeChangedEventResponse {
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
