pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstEqualsOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstEqualsOperatorNodeOutput {
    pub fn builder() -> AstEqualsOperatorNodeOutputBuilder {
        <AstEqualsOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstEqualsOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstEqualsOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstEqualsOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstEqualsOperatorNodeOutputBuilder::left)
    /// - [`right`](AstEqualsOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstEqualsOperatorNodeOutput, BuildError> {
        Ok(AstEqualsOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
