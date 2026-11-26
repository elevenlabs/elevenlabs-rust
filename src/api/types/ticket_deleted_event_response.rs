pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketDeletedEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketDeletedEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: TicketDeletedEventResponseType,
}

impl TicketDeletedEventResponse {
    pub fn builder() -> TicketDeletedEventResponseBuilder {
        <TicketDeletedEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketDeletedEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketDeletedEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<TicketDeletedEventResponseType>,
}

impl TicketDeletedEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketDeletedEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TicketDeletedEventResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketDeletedEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketDeletedEventResponseBuilder::id)
    /// - [`visibility`](TicketDeletedEventResponseBuilder::visibility)
    /// - [`author`](TicketDeletedEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketDeletedEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketDeletedEventResponseBuilder::type)
    pub fn build(self) -> Result<TicketDeletedEventResponse, BuildError> {
        Ok(TicketDeletedEventResponse {
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
        })
    }
}
