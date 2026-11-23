pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentChatResponsePart {
    pub text_response_part: TextResponsePart,
}

impl AgentChatResponsePart {
    pub fn builder() -> AgentChatResponsePartBuilder {
        <AgentChatResponsePartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentChatResponsePartBuilder {
    text_response_part: Option<TextResponsePart>,
}

impl AgentChatResponsePartBuilder {
    pub fn text_response_part(mut self, value: TextResponsePart) -> Self {
        self.text_response_part = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentChatResponsePart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text_response_part`](AgentChatResponsePartBuilder::text_response_part)
    pub fn build(self) -> Result<AgentChatResponsePart, BuildError> {
        Ok(AgentChatResponsePart {
            text_response_part: self
                .text_response_part
                .ok_or_else(|| BuildError::missing_field("text_response_part"))?,
        })
    }
}
