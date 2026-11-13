pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateProductParams {}

impl UpdateProductParams {
    pub fn builder() -> UpdateProductParamsBuilder {
        <UpdateProductParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProductParamsBuilder {}

impl UpdateProductParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateProductParams`].
    pub fn build(self) -> Result<UpdateProductParams, BuildError> {
        Ok(UpdateProductParams {})
    }
}
