pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstGreaterThanOrEqualsOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstGreaterThanOrEqualsOperatorNodeInput {
    pub fn builder() -> AstGreaterThanOrEqualsOperatorNodeInputBuilder {
        <AstGreaterThanOrEqualsOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstGreaterThanOrEqualsOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstGreaterThanOrEqualsOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstGreaterThanOrEqualsOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstGreaterThanOrEqualsOperatorNodeInputBuilder::left)
    /// - [`right`](AstGreaterThanOrEqualsOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstGreaterThanOrEqualsOperatorNodeInput, BuildError> {
        Ok(AstGreaterThanOrEqualsOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
