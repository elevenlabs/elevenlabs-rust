pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketSubscriberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TicketEventHumanAgentAuthorResponseType>,
    #[serde(default)]
    pub id: String,
}

impl TicketSubscriberRequest {
    pub fn builder() -> TicketSubscriberRequestBuilder {
        <TicketSubscriberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSubscriberRequestBuilder {
    r#type: Option<TicketEventHumanAgentAuthorResponseType>,
    id: Option<String>,
}

impl TicketSubscriberRequestBuilder {
    pub fn r#type(mut self, value: TicketEventHumanAgentAuthorResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketSubscriberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketSubscriberRequestBuilder::id)
    pub fn build(self) -> Result<TicketSubscriberRequest, BuildError> {
        Ok(TicketSubscriberRequest {
            r#type: self.r#type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
