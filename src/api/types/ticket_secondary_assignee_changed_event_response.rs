pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketSecondaryAssigneeChangedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketSecondaryAssigneeChangedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketSecondaryAssigneeChangedEventResponseType,
    pub before: Option<TicketSecondaryAssigneeChangedEventResponseBefore>,
    pub after: Option<TicketSecondaryAssigneeChangedEventResponseAfter>,
}

impl TicketSecondaryAssigneeChangedEventResponse {
    pub fn builder() -> TicketSecondaryAssigneeChangedEventResponseBuilder {
        <TicketSecondaryAssigneeChangedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSecondaryAssigneeChangedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketSecondaryAssigneeChangedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketSecondaryAssigneeChangedEventResponseType>,
    before: Option<TicketSecondaryAssigneeChangedEventResponseBefore>,
    after: Option<TicketSecondaryAssigneeChangedEventResponseAfter>,
}

impl TicketSecondaryAssigneeChangedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketSecondaryAssigneeChangedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketSecondaryAssigneeChangedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before(mut self, value: TicketSecondaryAssigneeChangedEventResponseBefore) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: TicketSecondaryAssigneeChangedEventResponseAfter) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketSecondaryAssigneeChangedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketSecondaryAssigneeChangedEventResponseBuilder::id)
    /// - [`visibility`](TicketSecondaryAssigneeChangedEventResponseBuilder::visibility)
    /// - [`author`](TicketSecondaryAssigneeChangedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketSecondaryAssigneeChangedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketSecondaryAssigneeChangedEventResponseBuilder::type)
    pub fn build(self) -> Result<TicketSecondaryAssigneeChangedEventResponse, BuildError> {
        Ok(TicketSecondaryAssigneeChangedEventResponse {
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
