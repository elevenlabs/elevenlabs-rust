pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstSubtractionOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstSubtractionOperatorNodeOutput {
    pub fn builder() -> AstSubtractionOperatorNodeOutputBuilder {
        <AstSubtractionOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstSubtractionOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstSubtractionOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstSubtractionOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstSubtractionOperatorNodeOutputBuilder::left)
    /// - [`right`](AstSubtractionOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstSubtractionOperatorNodeOutput, BuildError> {
        Ok(AstSubtractionOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
