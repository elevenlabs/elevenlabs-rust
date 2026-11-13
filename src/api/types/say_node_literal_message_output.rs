pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SayNodeLiteralMessageOutput {
    /// Exact text spoken by the agent. Supports `{{variable_name}}` dynamic variable placeholders.
    #[serde(default)]
    pub text: String,
    /// Translations of `text`, keyed by language code. When the conversation language has an entry, its value is spoken instead of `text`.
    #[serde(default)]
    pub text_translations: std::collections::HashMap<String, TranslatedString>,
}

impl SayNodeLiteralMessageOutput {
    pub fn builder() -> SayNodeLiteralMessageOutputBuilder {
        <SayNodeLiteralMessageOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodeLiteralMessageOutputBuilder {
    text: Option<String>,
    text_translations: Option<std::collections::HashMap<String, TranslatedString>>,
}

impl SayNodeLiteralMessageOutputBuilder {
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

    /// Consumes the builder and constructs a [`SayNodeLiteralMessageOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SayNodeLiteralMessageOutputBuilder::text)
    /// - [`text_translations`](SayNodeLiteralMessageOutputBuilder::text_translations)
    pub fn build(self) -> Result<SayNodeLiteralMessageOutput, BuildError> {
        Ok(SayNodeLiteralMessageOutput {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            text_translations: self
                .text_translations
                .ok_or_else(|| BuildError::missing_field("text_translations"))?,
        })
    }
}
