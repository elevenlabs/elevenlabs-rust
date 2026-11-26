pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketEventHumanAgentAuthorResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

impl TicketEventHumanAgentAuthorResponse {
    pub fn builder() -> TicketEventHumanAgentAuthorResponseBuilder {
        <TicketEventHumanAgentAuthorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketEventHumanAgentAuthorResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

impl TicketEventHumanAgentAuthorResponseBuilder {
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

    /// Consumes the builder and constructs a [`TicketEventHumanAgentAuthorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketEventHumanAgentAuthorResponseBuilder::id)
    pub fn build(self) -> Result<TicketEventHumanAgentAuthorResponse, BuildError> {
        Ok(TicketEventHumanAgentAuthorResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            email: self.email,
        })
    }
}
