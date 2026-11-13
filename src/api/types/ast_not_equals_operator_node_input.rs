pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstNotEqualsOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstNotEqualsOperatorNodeInput {
    pub fn builder() -> AstNotEqualsOperatorNodeInputBuilder {
        <AstNotEqualsOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstNotEqualsOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstNotEqualsOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstNotEqualsOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstNotEqualsOperatorNodeInputBuilder::left)
    /// - [`right`](AstNotEqualsOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstNotEqualsOperatorNodeInput, BuildError> {
        Ok(AstNotEqualsOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
