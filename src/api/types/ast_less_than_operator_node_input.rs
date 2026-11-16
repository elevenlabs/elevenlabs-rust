pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstLessThanOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstLessThanOperatorNodeInput {
    pub fn builder() -> AstLessThanOperatorNodeInputBuilder {
        <AstLessThanOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstLessThanOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstLessThanOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstLessThanOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstLessThanOperatorNodeInputBuilder::left)
    /// - [`right`](AstLessThanOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstLessThanOperatorNodeInput, BuildError> {
        Ok(AstLessThanOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
