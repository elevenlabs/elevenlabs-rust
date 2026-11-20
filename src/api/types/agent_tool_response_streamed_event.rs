pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentToolResponseStreamedEvent {
    pub agent_tool_response: AgentToolResponseClientEventData,
}

impl AgentToolResponseStreamedEvent {
    pub fn builder() -> AgentToolResponseStreamedEventBuilder {
        <AgentToolResponseStreamedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolResponseStreamedEventBuilder {
    agent_tool_response: Option<AgentToolResponseClientEventData>,
}

impl AgentToolResponseStreamedEventBuilder {
    pub fn agent_tool_response(mut self, value: AgentToolResponseClientEventData) -> Self {
        self.agent_tool_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolResponseStreamedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_tool_response`](AgentToolResponseStreamedEventBuilder::agent_tool_response)
    pub fn build(self) -> Result<AgentToolResponseStreamedEvent, BuildError> {
        Ok(AgentToolResponseStreamedEvent {
            agent_tool_response: self
                .agent_tool_response
                .ok_or_else(|| BuildError::missing_field("agent_tool_response"))?,
        })
    }
}
