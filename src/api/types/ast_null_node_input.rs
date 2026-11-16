pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AstNullNodeInput {}

impl AstNullNodeInput {
    pub fn builder() -> AstNullNodeInputBuilder {
        <AstNullNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstNullNodeInputBuilder {}

impl AstNullNodeInputBuilder {
    /// Consumes the builder and constructs a [`AstNullNodeInput`].
    pub fn build(self) -> Result<AstNullNodeInput, BuildError> {
        Ok(AstNullNodeInput {})
    }
}
