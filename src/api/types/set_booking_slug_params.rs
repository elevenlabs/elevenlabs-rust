pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetBookingSlugParams {}

impl SetBookingSlugParams {
    pub fn builder() -> SetBookingSlugParamsBuilder {
        <SetBookingSlugParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetBookingSlugParamsBuilder {}

impl SetBookingSlugParamsBuilder {
    /// Consumes the builder and constructs a [`SetBookingSlugParams`].
    pub fn build(self) -> Result<SetBookingSlugParams, BuildError> {
        Ok(SetBookingSlugParams {})
    }
}
