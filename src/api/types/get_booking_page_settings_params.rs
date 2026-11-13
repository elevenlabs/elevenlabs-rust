pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBookingPageSettingsParams {}

impl GetBookingPageSettingsParams {
    pub fn builder() -> GetBookingPageSettingsParamsBuilder {
        <GetBookingPageSettingsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBookingPageSettingsParamsBuilder {}

impl GetBookingPageSettingsParamsBuilder {
    /// Consumes the builder and constructs a [`GetBookingPageSettingsParams`].
    pub fn build(self) -> Result<GetBookingPageSettingsParams, BuildError> {
        Ok(GetBookingPageSettingsParams {})
    }
}
