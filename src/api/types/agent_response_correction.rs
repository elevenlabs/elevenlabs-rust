pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponseCorrection {
    pub agent_response_correction_event: AgentResponseCorrectionEvent,
}

impl AgentResponseCorrection {
    pub fn builder() -> AgentResponseCorrectionBuilder {
        <AgentResponseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseCorrectionBuilder {
    agent_response_correction_event: Option<AgentResponseCorrectionEvent>,
}

impl AgentResponseCorrectionBuilder {
    pub fn agent_response_correction_event(mut self, value: AgentResponseCorrectionEvent) -> Self {
        self.agent_response_correction_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_response_correction_event`](AgentResponseCorrectionBuilder::agent_response_correction_event)
    pub fn build(self) -> Result<AgentResponseCorrection, BuildError> {
        Ok(AgentResponseCorrection {
            agent_response_correction_event: self
                .agent_response_correction_event
                .ok_or_else(|| BuildError::missing_field("agent_response_correction_event"))?,
        })
    }
}
