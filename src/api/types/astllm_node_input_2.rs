pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstllmNodeInput2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AstllmNodeOutputType>,
    /// The prompt to evaluate to a boolean value. Deprecated. Use a boolean schema instead.
    pub prompt: String,
}

impl AstllmNodeInput2 {
    pub fn builder() -> AstllmNodeInput2Builder {
        <AstllmNodeInput2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstllmNodeInput2Builder {
    r#type: Option<AstllmNodeOutputType>,
    prompt: Option<String>,
}

impl AstllmNodeInput2Builder {
    pub fn r#type(mut self, value: AstllmNodeOutputType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstllmNodeInput2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](AstllmNodeInput2Builder::prompt)
    pub fn build(self) -> Result<AstllmNodeInput2, BuildError> {
        Ok(AstllmNodeInput2 {
            r#type: self.r#type,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
