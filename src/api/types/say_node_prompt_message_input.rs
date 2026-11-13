pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SayNodePromptMessageInput {
    /// Description of the message the LLM should generate. It is appended to the agent's system prompt while this node is active.
    #[serde(default)]
    pub prompt: String,
}

impl SayNodePromptMessageInput {
    pub fn builder() -> SayNodePromptMessageInputBuilder {
        <SayNodePromptMessageInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodePromptMessageInputBuilder {
    prompt: Option<String>,
}

impl SayNodePromptMessageInputBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SayNodePromptMessageInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](SayNodePromptMessageInputBuilder::prompt)
    pub fn build(self) -> Result<SayNodePromptMessageInput, BuildError> {
        Ok(SayNodePromptMessageInput {
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
