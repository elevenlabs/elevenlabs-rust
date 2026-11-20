pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponseClientEvent {
    pub agent_response_event: AgentResponseClientEventData,
}

impl AgentResponseClientEvent {
    pub fn builder() -> AgentResponseClientEventBuilder {
        <AgentResponseClientEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseClientEventBuilder {
    agent_response_event: Option<AgentResponseClientEventData>,
}

impl AgentResponseClientEventBuilder {
    pub fn agent_response_event(mut self, value: AgentResponseClientEventData) -> Self {
        self.agent_response_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseClientEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_response_event`](AgentResponseClientEventBuilder::agent_response_event)
    pub fn build(self) -> Result<AgentResponseClientEvent, BuildError> {
        Ok(AgentResponseClientEvent {
            agent_response_event: self
                .agent_response_event
                .ok_or_else(|| BuildError::missing_field("agent_response_event"))?,
        })
    }
}
