pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBookingSlugStatusParams {}

impl GetBookingSlugStatusParams {
    pub fn builder() -> GetBookingSlugStatusParamsBuilder {
        <GetBookingSlugStatusParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBookingSlugStatusParamsBuilder {}

impl GetBookingSlugStatusParamsBuilder {
    /// Consumes the builder and constructs a [`GetBookingSlugStatusParams`].
    pub fn build(self) -> Result<GetBookingSlugStatusParams, BuildError> {
        Ok(GetBookingSlugStatusParams {})
    }
}
