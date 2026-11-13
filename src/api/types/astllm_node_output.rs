pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstllmNodeOutput {
    pub value_schema: LlmLiteralJsonSchemaProperty,
    /// The prompt to evaluate to a boolean value. Deprecated. Use a boolean schema instead.
    #[serde(default)]
    pub prompt: String,
}

impl AstllmNodeOutput {
    pub fn builder() -> AstllmNodeOutputBuilder {
        <AstllmNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstllmNodeOutputBuilder {
    value_schema: Option<LlmLiteralJsonSchemaProperty>,
    prompt: Option<String>,
}

impl AstllmNodeOutputBuilder {
    pub fn value_schema(mut self, value: LlmLiteralJsonSchemaProperty) -> Self {
        self.value_schema = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstllmNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value_schema`](AstllmNodeOutputBuilder::value_schema)
    /// - [`prompt`](AstllmNodeOutputBuilder::prompt)
    pub fn build(self) -> Result<AstllmNodeOutput, BuildError> {
        Ok(AstllmNodeOutput {
            value_schema: self
                .value_schema
                .ok_or_else(|| BuildError::missing_field("value_schema"))?,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
