pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstAndOperatorNodeOutput {
    /// Child nodes of the logical operator.
    #[serde(default)]
    pub children: Vec<AstNodeOutput>,
}

impl AstAndOperatorNodeOutput {
    pub fn builder() -> AstAndOperatorNodeOutputBuilder {
        <AstAndOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstAndOperatorNodeOutputBuilder {
    children: Option<Vec<AstNodeOutput>>,
}

impl AstAndOperatorNodeOutputBuilder {
    pub fn children(mut self, value: Vec<AstNodeOutput>) -> Self {
        self.children = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstAndOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children`](AstAndOperatorNodeOutputBuilder::children)
    pub fn build(self) -> Result<AstAndOperatorNodeOutput, BuildError> {
        Ok(AstAndOperatorNodeOutput {
            children: self
                .children
                .ok_or_else(|| BuildError::missing_field("children"))?,
        })
    }
}
