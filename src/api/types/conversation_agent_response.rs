pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationAgentResponse {
    pub agent_response_event: AgentResponseEvent,
}

impl ConversationAgentResponse {
    pub fn builder() -> ConversationAgentResponseBuilder {
        <ConversationAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationAgentResponseBuilder {
    agent_response_event: Option<AgentResponseEvent>,
}

impl ConversationAgentResponseBuilder {
    pub fn agent_response_event(mut self, value: AgentResponseEvent) -> Self {
        self.agent_response_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_response_event`](ConversationAgentResponseBuilder::agent_response_event)
    pub fn build(self) -> Result<ConversationAgentResponse, BuildError> {
        Ok(ConversationAgentResponse {
            agent_response_event: self
                .agent_response_event
                .ok_or_else(|| BuildError::missing_field("agent_response_event"))?,
        })
    }
}
