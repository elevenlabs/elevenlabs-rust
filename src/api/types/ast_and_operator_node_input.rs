pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstAndOperatorNodeInput {
    /// Child nodes of the logical operator.
    #[serde(default)]
    pub children: Vec<AstNodeInput>,
}

impl AstAndOperatorNodeInput {
    pub fn builder() -> AstAndOperatorNodeInputBuilder {
        <AstAndOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstAndOperatorNodeInputBuilder {
    children: Option<Vec<AstNodeInput>>,
}

impl AstAndOperatorNodeInputBuilder {
    pub fn children(mut self, value: Vec<AstNodeInput>) -> Self {
        self.children = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstAndOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children`](AstAndOperatorNodeInputBuilder::children)
    pub fn build(self) -> Result<AstAndOperatorNodeInput, BuildError> {
        Ok(AstAndOperatorNodeInput {
            children: self
                .children
                .ok_or_else(|| BuildError::missing_field("children"))?,
        })
    }
}
