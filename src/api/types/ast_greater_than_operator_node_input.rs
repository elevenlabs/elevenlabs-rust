pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstGreaterThanOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstGreaterThanOperatorNodeInput {
    pub fn builder() -> AstGreaterThanOperatorNodeInputBuilder {
        <AstGreaterThanOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstGreaterThanOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstGreaterThanOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstGreaterThanOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstGreaterThanOperatorNodeInputBuilder::left)
    /// - [`right`](AstGreaterThanOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstGreaterThanOperatorNodeInput, BuildError> {
        Ok(AstGreaterThanOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
