pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstMultiplicationOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstMultiplicationOperatorNodeOutput {
    pub fn builder() -> AstMultiplicationOperatorNodeOutputBuilder {
        <AstMultiplicationOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstMultiplicationOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstMultiplicationOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstMultiplicationOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstMultiplicationOperatorNodeOutputBuilder::left)
    /// - [`right`](AstMultiplicationOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstMultiplicationOperatorNodeOutput, BuildError> {
        Ok(AstMultiplicationOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
