pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstAdditionOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstAdditionOperatorNodeInput {
    pub fn builder() -> AstAdditionOperatorNodeInputBuilder {
        <AstAdditionOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstAdditionOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstAdditionOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstAdditionOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstAdditionOperatorNodeInputBuilder::left)
    /// - [`right`](AstAdditionOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstAdditionOperatorNodeInput, BuildError> {
        Ok(AstAdditionOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
