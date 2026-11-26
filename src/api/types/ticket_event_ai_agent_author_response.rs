pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketEventAiAgentAuthorResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

impl TicketEventAiAgentAuthorResponse {
    pub fn builder() -> TicketEventAiAgentAuthorResponseBuilder {
        <TicketEventAiAgentAuthorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketEventAiAgentAuthorResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl TicketEventAiAgentAuthorResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketEventAiAgentAuthorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketEventAiAgentAuthorResponseBuilder::id)
    pub fn build(self) -> Result<TicketEventAiAgentAuthorResponse, BuildError> {
        Ok(TicketEventAiAgentAuthorResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
