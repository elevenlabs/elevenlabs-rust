pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstNotEqualsOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstNotEqualsOperatorNodeOutput {
    pub fn builder() -> AstNotEqualsOperatorNodeOutputBuilder {
        <AstNotEqualsOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstNotEqualsOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstNotEqualsOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstNotEqualsOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstNotEqualsOperatorNodeOutputBuilder::left)
    /// - [`right`](AstNotEqualsOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstNotEqualsOperatorNodeOutput, BuildError> {
        Ok(AstNotEqualsOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
