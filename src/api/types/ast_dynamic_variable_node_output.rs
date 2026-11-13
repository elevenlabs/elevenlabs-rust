pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstDynamicVariableNodeOutput {
    /// The name of the dynamic variable.
    #[serde(default)]
    pub name: String,
}

impl AstDynamicVariableNodeOutput {
    pub fn builder() -> AstDynamicVariableNodeOutputBuilder {
        <AstDynamicVariableNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstDynamicVariableNodeOutputBuilder {
    name: Option<String>,
}

impl AstDynamicVariableNodeOutputBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstDynamicVariableNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AstDynamicVariableNodeOutputBuilder::name)
    pub fn build(self) -> Result<AstDynamicVariableNodeOutput, BuildError> {
        Ok(AstDynamicVariableNodeOutput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
