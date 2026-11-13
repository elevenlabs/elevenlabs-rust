pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SayNodeLiteralMessageInput {
    /// Exact text spoken by the agent. Supports `{{variable_name}}` dynamic variable placeholders.
    #[serde(default)]
    pub text: String,
    /// Translations of `text`, keyed by language code. When the conversation language has an entry, its value is spoken instead of `text`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_translations: Option<std::collections::HashMap<String, TranslatedString>>,
}

impl SayNodeLiteralMessageInput {
    pub fn builder() -> SayNodeLiteralMessageInputBuilder {
        <SayNodeLiteralMessageInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodeLiteralMessageInputBuilder {
    text: Option<String>,
    text_translations: Option<std::collections::HashMap<String, TranslatedString>>,
}

impl SayNodeLiteralMessageInputBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn text_translations(
        mut self,
        value: std::collections::HashMap<String, TranslatedString>,
    ) -> Self {
        self.text_translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SayNodeLiteralMessageInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SayNodeLiteralMessageInputBuilder::text)
    pub fn build(self) -> Result<SayNodeLiteralMessageInput, BuildError> {
        Ok(SayNodeLiteralMessageInput {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            text_translations: self.text_translations,
        })
    }
}
