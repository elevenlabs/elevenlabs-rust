pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentToolRequest {
    pub agent_tool_request: AgentToolRequestAgentToolRequest,
}

impl AgentToolRequest {
    pub fn builder() -> AgentToolRequestBuilder {
        <AgentToolRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolRequestBuilder {
    agent_tool_request: Option<AgentToolRequestAgentToolRequest>,
}

impl AgentToolRequestBuilder {
    pub fn agent_tool_request(mut self, value: AgentToolRequestAgentToolRequest) -> Self {
        self.agent_tool_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_tool_request`](AgentToolRequestBuilder::agent_tool_request)
    pub fn build(self) -> Result<AgentToolRequest, BuildError> {
        Ok(AgentToolRequest {
            agent_tool_request: self
                .agent_tool_request
                .ok_or_else(|| BuildError::missing_field("agent_tool_request"))?,
        })
    }
}
