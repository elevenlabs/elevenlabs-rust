pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstLessThanOrEqualsOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstLessThanOrEqualsOperatorNodeOutput {
    pub fn builder() -> AstLessThanOrEqualsOperatorNodeOutputBuilder {
        <AstLessThanOrEqualsOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstLessThanOrEqualsOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstLessThanOrEqualsOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstLessThanOrEqualsOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstLessThanOrEqualsOperatorNodeOutputBuilder::left)
    /// - [`right`](AstLessThanOrEqualsOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstLessThanOrEqualsOperatorNodeOutput, BuildError> {
        Ok(AstLessThanOrEqualsOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
