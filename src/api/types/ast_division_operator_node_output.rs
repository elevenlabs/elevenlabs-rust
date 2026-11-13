pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstDivisionOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstDivisionOperatorNodeOutput {
    pub fn builder() -> AstDivisionOperatorNodeOutputBuilder {
        <AstDivisionOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstDivisionOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstDivisionOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstDivisionOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstDivisionOperatorNodeOutputBuilder::left)
    /// - [`right`](AstDivisionOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstDivisionOperatorNodeOutput, BuildError> {
        Ok(AstDivisionOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
