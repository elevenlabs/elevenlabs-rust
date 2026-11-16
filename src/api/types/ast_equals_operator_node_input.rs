pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstEqualsOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstEqualsOperatorNodeInput {
    pub fn builder() -> AstEqualsOperatorNodeInputBuilder {
        <AstEqualsOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstEqualsOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstEqualsOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstEqualsOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstEqualsOperatorNodeInputBuilder::left)
    /// - [`right`](AstEqualsOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstEqualsOperatorNodeInput, BuildError> {
        Ok(AstEqualsOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
