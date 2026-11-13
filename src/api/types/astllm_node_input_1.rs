pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstllmNodeInput1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AstllmNodeOutputType>,
    pub value_schema: LlmLiteralJsonSchemaProperty,
}

impl AstllmNodeInput1 {
    pub fn builder() -> AstllmNodeInput1Builder {
        <AstllmNodeInput1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstllmNodeInput1Builder {
    r#type: Option<AstllmNodeOutputType>,
    value_schema: Option<LlmLiteralJsonSchemaProperty>,
}

impl AstllmNodeInput1Builder {
    pub fn r#type(mut self, value: AstllmNodeOutputType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value_schema(mut self, value: LlmLiteralJsonSchemaProperty) -> Self {
        self.value_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstllmNodeInput1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value_schema`](AstllmNodeInput1Builder::value_schema)
    pub fn build(self) -> Result<AstllmNodeInput1, BuildError> {
        Ok(AstllmNodeInput1 {
            r#type: self.r#type,
            value_schema: self
                .value_schema
                .ok_or_else(|| BuildError::missing_field("value_schema"))?,
        })
    }
}
