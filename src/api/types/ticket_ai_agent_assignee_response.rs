pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketAiAgentAssigneeResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

impl TicketAiAgentAssigneeResponse {
    pub fn builder() -> TicketAiAgentAssigneeResponseBuilder {
        <TicketAiAgentAssigneeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketAiAgentAssigneeResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl TicketAiAgentAssigneeResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketAiAgentAssigneeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketAiAgentAssigneeResponseBuilder::id)
    pub fn build(self) -> Result<TicketAiAgentAssigneeResponse, BuildError> {
        Ok(TicketAiAgentAssigneeResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
