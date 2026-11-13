pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstLessThanOrEqualsOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstLessThanOrEqualsOperatorNodeInput {
    pub fn builder() -> AstLessThanOrEqualsOperatorNodeInputBuilder {
        <AstLessThanOrEqualsOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstLessThanOrEqualsOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstLessThanOrEqualsOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstLessThanOrEqualsOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstLessThanOrEqualsOperatorNodeInputBuilder::left)
    /// - [`right`](AstLessThanOrEqualsOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstLessThanOrEqualsOperatorNodeInput, BuildError> {
        Ok(AstLessThanOrEqualsOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
