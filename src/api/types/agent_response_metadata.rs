pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentResponseMetadata {
    pub agent_response_metadata_event: AgentResponseMetadataEvent,
}

impl AgentResponseMetadata {
    pub fn builder() -> AgentResponseMetadataBuilder {
        <AgentResponseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseMetadataBuilder {
    agent_response_metadata_event: Option<AgentResponseMetadataEvent>,
}

impl AgentResponseMetadataBuilder {
    pub fn agent_response_metadata_event(mut self, value: AgentResponseMetadataEvent) -> Self {
        self.agent_response_metadata_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_response_metadata_event`](AgentResponseMetadataBuilder::agent_response_metadata_event)
    pub fn build(self) -> Result<AgentResponseMetadata, BuildError> {
        Ok(AgentResponseMetadata {
            agent_response_metadata_event: self
                .agent_response_metadata_event
                .ok_or_else(|| BuildError::missing_field("agent_response_metadata_event"))?,
        })
    }
}
