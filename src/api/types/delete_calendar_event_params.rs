pub use crate::prelude::*;

/// Permanently remove a previously-cancelled calendar event.
///
/// This delete tool is the irreversible follow-up to cancel_calendar_event.
/// The backend rejects the call (422) if the event hasn't been
/// cancelled yet, so the only safe path is cancel-then-delete.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteCalendarEventParams {}

impl DeleteCalendarEventParams {
    pub fn builder() -> DeleteCalendarEventParamsBuilder {
        <DeleteCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteCalendarEventParamsBuilder {}

impl DeleteCalendarEventParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteCalendarEventParams`].
    pub fn build(self) -> Result<DeleteCalendarEventParams, BuildError> {
        Ok(DeleteCalendarEventParams {})
    }
}
