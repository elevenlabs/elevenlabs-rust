pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstGreaterThanOrEqualsOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstGreaterThanOrEqualsOperatorNodeOutput {
    pub fn builder() -> AstGreaterThanOrEqualsOperatorNodeOutputBuilder {
        <AstGreaterThanOrEqualsOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstGreaterThanOrEqualsOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstGreaterThanOrEqualsOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstGreaterThanOrEqualsOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstGreaterThanOrEqualsOperatorNodeOutputBuilder::left)
    /// - [`right`](AstGreaterThanOrEqualsOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstGreaterThanOrEqualsOperatorNodeOutput, BuildError> {
        Ok(AstGreaterThanOrEqualsOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
