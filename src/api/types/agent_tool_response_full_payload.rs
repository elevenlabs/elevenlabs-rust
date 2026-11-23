pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentToolResponseFullPayload {
    pub agent_tool_response_full_payload: AgentToolResponseFullPayloadAgentToolResponseFullPayload,
}

impl AgentToolResponseFullPayload {
    pub fn builder() -> AgentToolResponseFullPayloadBuilder {
        <AgentToolResponseFullPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolResponseFullPayloadBuilder {
    agent_tool_response_full_payload:
        Option<AgentToolResponseFullPayloadAgentToolResponseFullPayload>,
}

impl AgentToolResponseFullPayloadBuilder {
    pub fn agent_tool_response_full_payload(
        mut self,
        value: AgentToolResponseFullPayloadAgentToolResponseFullPayload,
    ) -> Self {
        self.agent_tool_response_full_payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolResponseFullPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_tool_response_full_payload`](AgentToolResponseFullPayloadBuilder::agent_tool_response_full_payload)
    pub fn build(self) -> Result<AgentToolResponseFullPayload, BuildError> {
        Ok(AgentToolResponseFullPayload {
            agent_tool_response_full_payload: self
                .agent_tool_response_full_payload
                .ok_or_else(|| BuildError::missing_field("agent_tool_response_full_payload"))?,
        })
    }
}
