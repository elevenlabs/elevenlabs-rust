pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderByConfirmationNumberParams {}

impl GetOrderByConfirmationNumberParams {
    pub fn builder() -> GetOrderByConfirmationNumberParamsBuilder {
        <GetOrderByConfirmationNumberParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderByConfirmationNumberParamsBuilder {}

impl GetOrderByConfirmationNumberParamsBuilder {
    /// Consumes the builder and constructs a [`GetOrderByConfirmationNumberParams`].
    pub fn build(self) -> Result<GetOrderByConfirmationNumberParams, BuildError> {
        Ok(GetOrderByConfirmationNumberParams {})
    }
}
