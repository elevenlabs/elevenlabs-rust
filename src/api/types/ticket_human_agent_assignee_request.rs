pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketHumanAgentAssigneeRequest {
    #[serde(default)]
    pub id: String,
}

impl TicketHumanAgentAssigneeRequest {
    pub fn builder() -> TicketHumanAgentAssigneeRequestBuilder {
        <TicketHumanAgentAssigneeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketHumanAgentAssigneeRequestBuilder {
    id: Option<String>,
}

impl TicketHumanAgentAssigneeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketHumanAgentAssigneeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketHumanAgentAssigneeRequestBuilder::id)
    pub fn build(self) -> Result<TicketHumanAgentAssigneeRequest, BuildError> {
        Ok(TicketHumanAgentAssigneeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
