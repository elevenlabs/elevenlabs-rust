pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderParams {}

impl CancelOrderParams {
    pub fn builder() -> CancelOrderParamsBuilder {
        <CancelOrderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderParamsBuilder {}

impl CancelOrderParamsBuilder {
    /// Consumes the builder and constructs a [`CancelOrderParams`].
    pub fn build(self) -> Result<CancelOrderParams, BuildError> {
        Ok(CancelOrderParams {})
    }
}
