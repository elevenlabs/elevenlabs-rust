pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponseComplete {
    pub agent_response_complete_event: AgentResponseCompleteEvent,
}

impl AgentResponseComplete {
    pub fn builder() -> AgentResponseCompleteBuilder {
        <AgentResponseCompleteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseCompleteBuilder {
    agent_response_complete_event: Option<AgentResponseCompleteEvent>,
}

impl AgentResponseCompleteBuilder {
    pub fn agent_response_complete_event(mut self, value: AgentResponseCompleteEvent) -> Self {
        self.agent_response_complete_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseComplete`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_response_complete_event`](AgentResponseCompleteBuilder::agent_response_complete_event)
    pub fn build(self) -> Result<AgentResponseComplete, BuildError> {
        Ok(AgentResponseComplete {
            agent_response_complete_event: self
                .agent_response_complete_event
                .ok_or_else(|| BuildError::missing_field("agent_response_complete_event"))?,
        })
    }
}
