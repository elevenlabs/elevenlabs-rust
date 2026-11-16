pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateHolidayParams {}

impl CreateHolidayParams {
    pub fn builder() -> CreateHolidayParamsBuilder {
        <CreateHolidayParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateHolidayParamsBuilder {}

impl CreateHolidayParamsBuilder {
    /// Consumes the builder and constructs a [`CreateHolidayParams`].
    pub fn build(self) -> Result<CreateHolidayParams, BuildError> {
        Ok(CreateHolidayParams {})
    }
}
