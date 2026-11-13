pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderParams {}

impl UpdateOrderParams {
    pub fn builder() -> UpdateOrderParamsBuilder {
        <UpdateOrderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderParamsBuilder {}

impl UpdateOrderParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateOrderParams`].
    pub fn build(self) -> Result<UpdateOrderParams, BuildError> {
        Ok(UpdateOrderParams {})
    }
}
