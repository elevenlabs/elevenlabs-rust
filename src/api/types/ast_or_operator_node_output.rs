pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstOrOperatorNodeOutput {
    /// Child nodes of the logical operator.
    #[serde(default)]
    pub children: Vec<AstNodeOutput>,
}

impl AstOrOperatorNodeOutput {
    pub fn builder() -> AstOrOperatorNodeOutputBuilder {
        <AstOrOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstOrOperatorNodeOutputBuilder {
    children: Option<Vec<AstNodeOutput>>,
}

impl AstOrOperatorNodeOutputBuilder {
    pub fn children(mut self, value: Vec<AstNodeOutput>) -> Self {
        self.children = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstOrOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children`](AstOrOperatorNodeOutputBuilder::children)
    pub fn build(self) -> Result<AstOrOperatorNodeOutput, BuildError> {
        Ok(AstOrOperatorNodeOutput {
            children: self
                .children
                .ok_or_else(|| BuildError::missing_field("children"))?,
        })
    }
}
