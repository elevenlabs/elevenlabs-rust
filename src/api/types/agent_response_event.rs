pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponseEvent {
    #[serde(default)]
    pub agent_response: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(default)]
    pub response_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_response_to_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<AgentResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentsItem>>,
}

impl AgentResponseEvent {
    pub fn builder() -> AgentResponseEventBuilder {
        <AgentResponseEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseEventBuilder {
    agent_response: Option<String>,
    event_id: Option<i64>,
    response_id: Option<String>,
    in_response_to_ids: Option<Vec<String>>,
    response_type: Option<AgentResponseType>,
    attachments: Option<Vec<AttachmentsItem>>,
}

impl AgentResponseEventBuilder {
    pub fn agent_response(mut self, value: impl Into<String>) -> Self {
        self.agent_response = Some(value.into());
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

    pub fn in_response_to_ids(mut self, value: Vec<String>) -> Self {
        self.in_response_to_ids = Some(value);
        self
    }

    pub fn response_type(mut self, value: AgentResponseType) -> Self {
        self.response_type = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<AttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_response`](AgentResponseEventBuilder::agent_response)
    /// - [`event_id`](AgentResponseEventBuilder::event_id)
    /// - [`response_id`](AgentResponseEventBuilder::response_id)
    pub fn build(self) -> Result<AgentResponseEvent, BuildError> {
        Ok(AgentResponseEvent {
            agent_response: self
                .agent_response
                .ok_or_else(|| BuildError::missing_field("agent_response"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            response_id: self
                .response_id
                .ok_or_else(|| BuildError::missing_field("response_id"))?,
            in_response_to_ids: self.in_response_to_ids,
            response_type: self.response_type,
            attachments: self.attachments,
        })
    }
}
