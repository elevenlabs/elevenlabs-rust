pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstDivisionOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstDivisionOperatorNodeInput {
    pub fn builder() -> AstDivisionOperatorNodeInputBuilder {
        <AstDivisionOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstDivisionOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstDivisionOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstDivisionOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstDivisionOperatorNodeInputBuilder::left)
    /// - [`right`](AstDivisionOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstDivisionOperatorNodeInput, BuildError> {
        Ok(AstDivisionOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
