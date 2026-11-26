pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTicketMessageRequest {
    pub author: CreateTicketMessageRequestAuthor,
    #[serde(default)]
    pub body: String,
    pub visibility: TicketEventVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TicketStatus>,
}

impl CreateTicketMessageRequest {
    pub fn builder() -> CreateTicketMessageRequestBuilder {
        <CreateTicketMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTicketMessageRequestBuilder {
    author: Option<CreateTicketMessageRequestAuthor>,
    body: Option<String>,
    visibility: Option<TicketEventVisibility>,
    status: Option<TicketStatus>,
}

impl CreateTicketMessageRequestBuilder {
    pub fn author(mut self, value: CreateTicketMessageRequestAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: TicketEventVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn status(mut self, value: TicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTicketMessageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`author`](CreateTicketMessageRequestBuilder::author)
    /// - [`body`](CreateTicketMessageRequestBuilder::body)
    /// - [`visibility`](CreateTicketMessageRequestBuilder::visibility)
    pub fn build(self) -> Result<CreateTicketMessageRequest, BuildError> {
        Ok(CreateTicketMessageRequest {
            author: self
                .author
                .ok_or_else(|| BuildError::missing_field("author"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
            status: self.status,
        })
    }
}
