pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCalendarEventParams {}

impl UpdateCalendarEventParams {
    pub fn builder() -> UpdateCalendarEventParamsBuilder {
        <UpdateCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarEventParamsBuilder {}

impl UpdateCalendarEventParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateCalendarEventParams`].
    pub fn build(self) -> Result<UpdateCalendarEventParams, BuildError> {
        Ok(UpdateCalendarEventParams {})
    }
}
