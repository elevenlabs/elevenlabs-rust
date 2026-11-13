pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstMultiplicationOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstMultiplicationOperatorNodeInput {
    pub fn builder() -> AstMultiplicationOperatorNodeInputBuilder {
        <AstMultiplicationOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstMultiplicationOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstMultiplicationOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstMultiplicationOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstMultiplicationOperatorNodeInputBuilder::left)
    /// - [`right`](AstMultiplicationOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstMultiplicationOperatorNodeInput, BuildError> {
        Ok(AstMultiplicationOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
