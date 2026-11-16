pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SayNodePromptMessageOutput {
    /// Description of the message the LLM should generate. It is appended to the agent's system prompt while this node is active.
    #[serde(default)]
    pub prompt: String,
}

impl SayNodePromptMessageOutput {
    pub fn builder() -> SayNodePromptMessageOutputBuilder {
        <SayNodePromptMessageOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodePromptMessageOutputBuilder {
    prompt: Option<String>,
}

impl SayNodePromptMessageOutputBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SayNodePromptMessageOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](SayNodePromptMessageOutputBuilder::prompt)
    pub fn build(self) -> Result<SayNodePromptMessageOutput, BuildError> {
        Ok(SayNodePromptMessageOutput {
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
