pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBookingPageSettingsParams {}

impl UpdateBookingPageSettingsParams {
    pub fn builder() -> UpdateBookingPageSettingsParamsBuilder {
        <UpdateBookingPageSettingsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBookingPageSettingsParamsBuilder {}

impl UpdateBookingPageSettingsParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateBookingPageSettingsParams`].
    pub fn build(self) -> Result<UpdateBookingPageSettingsParams, BuildError> {
        Ok(UpdateBookingPageSettingsParams {})
    }
}
