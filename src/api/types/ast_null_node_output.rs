pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstNullNodeOutput {}

impl AstNullNodeOutput {
    pub fn builder() -> AstNullNodeOutputBuilder {
        <AstNullNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstNullNodeOutputBuilder {}

impl AstNullNodeOutputBuilder {
    /// Consumes the builder and constructs a [`AstNullNodeOutput`].
    pub fn build(self) -> Result<AstNullNodeOutput, BuildError> {
        Ok(AstNullNodeOutput {})
    }
}
