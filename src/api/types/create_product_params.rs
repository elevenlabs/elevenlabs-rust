pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductParams {}

impl CreateProductParams {
    pub fn builder() -> CreateProductParamsBuilder {
        <CreateProductParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductParamsBuilder {}

impl CreateProductParamsBuilder {
    /// Consumes the builder and constructs a [`CreateProductParams`].
    pub fn build(self) -> Result<CreateProductParams, BuildError> {
        Ok(CreateProductParams {})
    }
}
