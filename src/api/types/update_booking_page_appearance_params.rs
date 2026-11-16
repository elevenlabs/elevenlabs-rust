pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBookingPageAppearanceParams {}

impl UpdateBookingPageAppearanceParams {
    pub fn builder() -> UpdateBookingPageAppearanceParamsBuilder {
        <UpdateBookingPageAppearanceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBookingPageAppearanceParamsBuilder {}

impl UpdateBookingPageAppearanceParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateBookingPageAppearanceParams`].
    pub fn build(self) -> Result<UpdateBookingPageAppearanceParams, BuildError> {
        Ok(UpdateBookingPageAppearanceParams {})
    }
}
