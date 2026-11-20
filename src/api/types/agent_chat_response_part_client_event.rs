pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentChatResponsePartClientEvent {
    pub text_response_part: AgentChatResponsePartClientEventData,
}

impl AgentChatResponsePartClientEvent {
    pub fn builder() -> AgentChatResponsePartClientEventBuilder {
        <AgentChatResponsePartClientEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentChatResponsePartClientEventBuilder {
    text_response_part: Option<AgentChatResponsePartClientEventData>,
}

impl AgentChatResponsePartClientEventBuilder {
    pub fn text_response_part(mut self, value: AgentChatResponsePartClientEventData) -> Self {
        self.text_response_part = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentChatResponsePartClientEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text_response_part`](AgentChatResponsePartClientEventBuilder::text_response_part)
    pub fn build(self) -> Result<AgentChatResponsePartClientEvent, BuildError> {
        Ok(AgentChatResponsePartClientEvent {
            text_response_part: self
                .text_response_part
                .ok_or_else(|| BuildError::missing_field("text_response_part"))?,
        })
    }
}
