pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteHolidayParams {}

impl DeleteHolidayParams {
    pub fn builder() -> DeleteHolidayParamsBuilder {
        <DeleteHolidayParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteHolidayParamsBuilder {}

impl DeleteHolidayParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteHolidayParams`].
    pub fn build(self) -> Result<DeleteHolidayParams, BuildError> {
        Ok(DeleteHolidayParams {})
    }
}
