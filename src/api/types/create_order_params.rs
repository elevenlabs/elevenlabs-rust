pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderParams {}

impl CreateOrderParams {
    pub fn builder() -> CreateOrderParamsBuilder {
        <CreateOrderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderParamsBuilder {}

impl CreateOrderParamsBuilder {
    /// Consumes the builder and constructs a [`CreateOrderParams`].
    pub fn build(self) -> Result<CreateOrderParams, BuildError> {
        Ok(CreateOrderParams {})
    }
}
