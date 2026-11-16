pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RestoreCalendarEventParams {}

impl RestoreCalendarEventParams {
    pub fn builder() -> RestoreCalendarEventParamsBuilder {
        <RestoreCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreCalendarEventParamsBuilder {}

impl RestoreCalendarEventParamsBuilder {
    /// Consumes the builder and constructs a [`RestoreCalendarEventParams`].
    pub fn build(self) -> Result<RestoreCalendarEventParams, BuildError> {
        Ok(RestoreCalendarEventParams {})
    }
}
