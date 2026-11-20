pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentReasoningResponsePartClientEvent {
    pub reasoning_response_part: AgentReasoningResponsePartClientEventData,
}

impl AgentReasoningResponsePartClientEvent {
    pub fn builder() -> AgentReasoningResponsePartClientEventBuilder {
        <AgentReasoningResponsePartClientEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentReasoningResponsePartClientEventBuilder {
    reasoning_response_part: Option<AgentReasoningResponsePartClientEventData>,
}

impl AgentReasoningResponsePartClientEventBuilder {
    pub fn reasoning_response_part(
        mut self,
        value: AgentReasoningResponsePartClientEventData,
    ) -> Self {
        self.reasoning_response_part = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentReasoningResponsePartClientEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reasoning_response_part`](AgentReasoningResponsePartClientEventBuilder::reasoning_response_part)
    pub fn build(self) -> Result<AgentReasoningResponsePartClientEvent, BuildError> {
        Ok(AgentReasoningResponsePartClientEvent {
            reasoning_response_part: self
                .reasoning_response_part
                .ok_or_else(|| BuildError::missing_field("reasoning_response_part"))?,
        })
    }
}
