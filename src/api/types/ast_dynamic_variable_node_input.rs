pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstDynamicVariableNodeInput {
    /// The name of the dynamic variable.
    #[serde(default)]
    pub name: String,
}

impl AstDynamicVariableNodeInput {
    pub fn builder() -> AstDynamicVariableNodeInputBuilder {
        <AstDynamicVariableNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstDynamicVariableNodeInputBuilder {
    name: Option<String>,
}

impl AstDynamicVariableNodeInputBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstDynamicVariableNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AstDynamicVariableNodeInputBuilder::name)
    pub fn build(self) -> Result<AstDynamicVariableNodeInput, BuildError> {
        Ok(AstDynamicVariableNodeInput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
