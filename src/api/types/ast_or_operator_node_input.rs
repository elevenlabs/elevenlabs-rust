pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstOrOperatorNodeInput {
    /// Child nodes of the logical operator.
    #[serde(default)]
    pub children: Vec<AstNodeInput>,
}

impl AstOrOperatorNodeInput {
    pub fn builder() -> AstOrOperatorNodeInputBuilder {
        <AstOrOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstOrOperatorNodeInputBuilder {
    children: Option<Vec<AstNodeInput>>,
}

impl AstOrOperatorNodeInputBuilder {
    pub fn children(mut self, value: Vec<AstNodeInput>) -> Self {
        self.children = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstOrOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children`](AstOrOperatorNodeInputBuilder::children)
    pub fn build(self) -> Result<AstOrOperatorNodeInput, BuildError> {
        Ok(AstOrOperatorNodeInput {
            children: self
                .children
                .ok_or_else(|| BuildError::missing_field("children"))?,
        })
    }
}
