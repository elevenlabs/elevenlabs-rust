pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstGreaterThanOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstGreaterThanOperatorNodeOutput {
    pub fn builder() -> AstGreaterThanOperatorNodeOutputBuilder {
        <AstGreaterThanOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstGreaterThanOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstGreaterThanOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstGreaterThanOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstGreaterThanOperatorNodeOutputBuilder::left)
    /// - [`right`](AstGreaterThanOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstGreaterThanOperatorNodeOutput, BuildError> {
        Ok(AstGreaterThanOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
