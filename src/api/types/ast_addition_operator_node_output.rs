pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstAdditionOperatorNodeOutput {
    pub left: Box<AstNodeOutput>,
    pub right: Box<AstNodeOutput>,
}

impl AstAdditionOperatorNodeOutput {
    pub fn builder() -> AstAdditionOperatorNodeOutputBuilder {
        <AstAdditionOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstAdditionOperatorNodeOutputBuilder {
    left: Option<Box<AstNodeOutput>>,
    right: Option<Box<AstNodeOutput>>,
}

impl AstAdditionOperatorNodeOutputBuilder {
    pub fn left(mut self, value: Box<AstNodeOutput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeOutput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstAdditionOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstAdditionOperatorNodeOutputBuilder::left)
    /// - [`right`](AstAdditionOperatorNodeOutputBuilder::right)
    pub fn build(self) -> Result<AstAdditionOperatorNodeOutput, BuildError> {
        Ok(AstAdditionOperatorNodeOutput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
