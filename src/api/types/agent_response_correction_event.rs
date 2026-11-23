pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponseCorrectionEvent {
    #[serde(default)]
    pub original_agent_response: String,
    #[serde(default)]
    pub corrected_agent_response: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(default)]
    pub response_id: String,
}

impl AgentResponseCorrectionEvent {
    pub fn builder() -> AgentResponseCorrectionEventBuilder {
        <AgentResponseCorrectionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseCorrectionEventBuilder {
    original_agent_response: Option<String>,
    corrected_agent_response: Option<String>,
    event_id: Option<i64>,
    response_id: Option<String>,
}

impl AgentResponseCorrectionEventBuilder {
    pub fn original_agent_response(mut self, value: impl Into<String>) -> Self {
        self.original_agent_response = Some(value.into());
        self
    }

    pub fn corrected_agent_response(mut self, value: impl Into<String>) -> Self {
        self.corrected_agent_response = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn response_id(mut self, value: impl Into<String>) -> Self {
        self.response_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseCorrectionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`original_agent_response`](AgentResponseCorrectionEventBuilder::original_agent_response)
    /// - [`corrected_agent_response`](AgentResponseCorrectionEventBuilder::corrected_agent_response)
    /// - [`event_id`](AgentResponseCorrectionEventBuilder::event_id)
    /// - [`response_id`](AgentResponseCorrectionEventBuilder::response_id)
    pub fn build(self) -> Result<AgentResponseCorrectionEvent, BuildError> {
        Ok(AgentResponseCorrectionEvent {
            original_agent_response: self
                .original_agent_response
                .ok_or_else(|| BuildError::missing_field("original_agent_response"))?,
            corrected_agent_response: self
                .corrected_agent_response
                .ok_or_else(|| BuildError::missing_field("corrected_agent_response"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            response_id: self
                .response_id
                .ok_or_else(|| BuildError::missing_field("response_id"))?,
        })
    }
}
