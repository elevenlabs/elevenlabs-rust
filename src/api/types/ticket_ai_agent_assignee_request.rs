pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketAiAgentAssigneeRequest {
    #[serde(default)]
    pub id: String,
}

impl TicketAiAgentAssigneeRequest {
    pub fn builder() -> TicketAiAgentAssigneeRequestBuilder {
        <TicketAiAgentAssigneeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketAiAgentAssigneeRequestBuilder {
    id: Option<String>,
}

impl TicketAiAgentAssigneeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketAiAgentAssigneeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketAiAgentAssigneeRequestBuilder::id)
    pub fn build(self) -> Result<TicketAiAgentAssigneeRequest, BuildError> {
        Ok(TicketAiAgentAssigneeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
