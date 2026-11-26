pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTicketMessageResponse {
    #[serde(default)]
    pub ticket_id: String,
    pub event: TicketMessageEventResponse,
}

impl CreateTicketMessageResponse {
    pub fn builder() -> CreateTicketMessageResponseBuilder {
        <CreateTicketMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTicketMessageResponseBuilder {
    ticket_id: Option<String>,
    event: Option<TicketMessageEventResponse>,
}

impl CreateTicketMessageResponseBuilder {
    pub fn ticket_id(mut self, value: impl Into<String>) -> Self {
        self.ticket_id = Some(value.into());
        self
    }

    pub fn event(mut self, value: TicketMessageEventResponse) -> Self {
        self.event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTicketMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ticket_id`](CreateTicketMessageResponseBuilder::ticket_id)
    /// - [`event`](CreateTicketMessageResponseBuilder::event)
    pub fn build(self) -> Result<CreateTicketMessageResponse, BuildError> {
        Ok(CreateTicketMessageResponse {
            ticket_id: self
                .ticket_id
                .ok_or_else(|| BuildError::missing_field("ticket_id"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
