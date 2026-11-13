pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateHolidayParams {}

impl UpdateHolidayParams {
    pub fn builder() -> UpdateHolidayParamsBuilder {
        <UpdateHolidayParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateHolidayParamsBuilder {}

impl UpdateHolidayParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateHolidayParams`].
    pub fn build(self) -> Result<UpdateHolidayParams, BuildError> {
        Ok(UpdateHolidayParams {})
    }
}
