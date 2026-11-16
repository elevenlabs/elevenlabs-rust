pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteProductParams {}

impl DeleteProductParams {
    pub fn builder() -> DeleteProductParamsBuilder {
        <DeleteProductParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteProductParamsBuilder {}

impl DeleteProductParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteProductParams`].
    pub fn build(self) -> Result<DeleteProductParams, BuildError> {
        Ok(DeleteProductParams {})
    }
}
