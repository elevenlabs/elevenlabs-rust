pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketHumanAgentAssigneeResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

impl TicketHumanAgentAssigneeResponse {
    pub fn builder() -> TicketHumanAgentAssigneeResponseBuilder {
        <TicketHumanAgentAssigneeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketHumanAgentAssigneeResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

impl TicketHumanAgentAssigneeResponseBuilder {
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

    /// Consumes the builder and constructs a [`TicketHumanAgentAssigneeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketHumanAgentAssigneeResponseBuilder::id)
    pub fn build(self) -> Result<TicketHumanAgentAssigneeResponse, BuildError> {
        Ok(TicketHumanAgentAssigneeResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            email: self.email,
        })
    }
}
