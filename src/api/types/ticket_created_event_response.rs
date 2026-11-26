pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketCreatedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketCreatedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketCreatedEventResponseType,
    pub priority: Option<TicketPriority>,
}

impl TicketCreatedEventResponse {
    pub fn builder() -> TicketCreatedEventResponseBuilder {
        <TicketCreatedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketCreatedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketCreatedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketCreatedEventResponseType>,
    priority: Option<TicketPriority>,
}

impl TicketCreatedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketCreatedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketCreatedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn priority(mut self, value: TicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketCreatedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketCreatedEventResponseBuilder::id)
    /// - [`visibility`](TicketCreatedEventResponseBuilder::visibility)
    /// - [`author`](TicketCreatedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketCreatedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketCreatedEventResponseBuilder::type)
    pub fn build(self) -> Result<TicketCreatedEventResponse, BuildError> {
        Ok(TicketCreatedEventResponse {
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
            priority: self.priority,
        })
    }
}
