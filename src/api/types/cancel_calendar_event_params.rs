pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelCalendarEventParams {}

impl CancelCalendarEventParams {
    pub fn builder() -> CancelCalendarEventParamsBuilder {
        <CancelCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelCalendarEventParamsBuilder {}

impl CancelCalendarEventParamsBuilder {
    /// Consumes the builder and constructs a [`CancelCalendarEventParams`].
    pub fn build(self) -> Result<CancelCalendarEventParams, BuildError> {
        Ok(CancelCalendarEventParams {})
    }
}
