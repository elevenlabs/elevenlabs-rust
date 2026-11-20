pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentToolRequestStreamedEvent {
    pub agent_tool_request: AgentToolRequestClientEventData,
}

impl AgentToolRequestStreamedEvent {
    pub fn builder() -> AgentToolRequestStreamedEventBuilder {
        <AgentToolRequestStreamedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolRequestStreamedEventBuilder {
    agent_tool_request: Option<AgentToolRequestClientEventData>,
}

impl AgentToolRequestStreamedEventBuilder {
    pub fn agent_tool_request(mut self, value: AgentToolRequestClientEventData) -> Self {
        self.agent_tool_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolRequestStreamedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_tool_request`](AgentToolRequestStreamedEventBuilder::agent_tool_request)
    pub fn build(self) -> Result<AgentToolRequestStreamedEvent, BuildError> {
        Ok(AgentToolRequestStreamedEvent {
            agent_tool_request: self
                .agent_tool_request
                .ok_or_else(|| BuildError::missing_field("agent_tool_request"))?,
        })
    }
}
