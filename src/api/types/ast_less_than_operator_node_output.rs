pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstLessThanOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstLessThanOperatorNodeOutput {
    pub fn builder() -> AstLessThanOperatorNodeOutputBuilder {
        <AstLessThanOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstLessThanOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstLessThanOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstLessThanOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstLessThanOperatorNodeOutputBuilder::left)
    /// - [`right`](AstLessThanOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstLessThanOperatorNodeOutput, BuildError> {
        Ok(AstLessThanOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
