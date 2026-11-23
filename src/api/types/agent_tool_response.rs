pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentToolResponse {
    pub agent_tool_response: AgentToolResponseAgentToolResponse,
}

impl AgentToolResponse {
    pub fn builder() -> AgentToolResponseBuilder {
        <AgentToolResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolResponseBuilder {
    agent_tool_response: Option<AgentToolResponseAgentToolResponse>,
}

impl AgentToolResponseBuilder {
    pub fn agent_tool_response(mut self, value: AgentToolResponseAgentToolResponse) -> Self {
        self.agent_tool_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_tool_response`](AgentToolResponseBuilder::agent_tool_response)
    pub fn build(self) -> Result<AgentToolResponse, BuildError> {
        Ok(AgentToolResponse {
            agent_tool_response: self
                .agent_tool_response
                .ok_or_else(|| BuildError::missing_field("agent_tool_response"))?,
        })
    }
}
