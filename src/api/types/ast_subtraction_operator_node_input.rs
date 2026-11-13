pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstSubtractionOperatorNodeInput {
    pub left: Box<AstNodeInput>,
    pub right: Box<AstNodeInput>,
}

impl AstSubtractionOperatorNodeInput {
    pub fn builder() -> AstSubtractionOperatorNodeInputBuilder {
        <AstSubtractionOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstSubtractionOperatorNodeInputBuilder {
    left: Option<Box<AstNodeInput>>,
    right: Option<Box<AstNodeInput>>,
}

impl AstSubtractionOperatorNodeInputBuilder {
    pub fn left(mut self, value: Box<AstNodeInput>) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: Box<AstNodeInput>) -> Self {
        self.right = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstSubtractionOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`left`](AstSubtractionOperatorNodeInputBuilder::left)
    /// - [`right`](AstSubtractionOperatorNodeInputBuilder::right)
    pub fn build(self) -> Result<AstSubtractionOperatorNodeInput, BuildError> {
        Ok(AstSubtractionOperatorNodeInput {
            left: self.left.ok_or_else(|| BuildError::missing_field("left"))?,
            right: self
                .right
                .ok_or_else(|| BuildError::missing_field("right"))?,
        })
    }
}
