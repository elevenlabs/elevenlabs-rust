pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketMessageEventResponse {
    pub id: String,
    pub visibility: TicketEventVisibility,
    pub author: TicketMessageEventResponseAuthor,
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    pub r#type: SubmitWhatsAppMessageConfigMode,
    pub body: String,
}

impl TicketMessageEventResponse {
    pub fn builder() -> TicketMessageEventResponseBuilder {
        <TicketMessageEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketMessageEventResponseBuilder {
    id: Option<String>,
    visibility: Option<TicketEventVisibility>,
    author: Option<TicketMessageEventResponseAuthor>,
    created_at_unix_secs: Option<i64>,
    r#type: Option<SubmitWhatsAppMessageConfigMode>,
    body: Option<String>,
}

impl TicketMessageEventResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn author(mut self, value: TicketMessageEventResponseAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: SubmitWhatsAppMessageConfigMode) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketMessageEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketMessageEventResponseBuilder::id)
    /// - [`visibility`](TicketMessageEventResponseBuilder::visibility)
    /// - [`author`](TicketMessageEventResponseBuilder::author)
    /// - [`created_at_unix_secs`](TicketMessageEventResponseBuilder::created_at_unix_secs)
    /// - [`r#type`](TicketMessageEventResponseBuilder::type)
    /// - [`body`](TicketMessageEventResponseBuilder::body)
    pub fn build(self) -> Result<TicketMessageEventResponse, BuildError> {
        Ok(TicketMessageEventResponse {
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
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
