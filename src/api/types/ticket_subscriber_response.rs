pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketSubscriberResponse {
    pub r#type: TicketEventHumanAgentAuthorResponseType,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

impl TicketSubscriberResponse {
    pub fn builder() -> TicketSubscriberResponseBuilder {
        <TicketSubscriberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSubscriberResponseBuilder {
    r#type: Option<TicketEventHumanAgentAuthorResponseType>,
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

impl TicketSubscriberResponseBuilder {
    pub fn r#type(mut self, value: TicketEventHumanAgentAuthorResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketSubscriberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TicketSubscriberResponseBuilder::type)
    /// - [`id`](TicketSubscriberResponseBuilder::id)
    pub fn build(self) -> Result<TicketSubscriberResponse, BuildError> {
        Ok(TicketSubscriberResponse {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            email: self.email,
        })
    }
}
