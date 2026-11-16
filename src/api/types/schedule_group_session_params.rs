pub use crate::prelude::*;

/// Schedule a single instance of a group service.
///
/// The session's duration is derived from the parent service so the assistant
/// only has to pin start time, the (optional) instructor / room, and the
/// location. Participants register separately via
/// ``register_for_group_session``.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleGroupSessionParams {}

impl ScheduleGroupSessionParams {
    pub fn builder() -> ScheduleGroupSessionParamsBuilder {
        <ScheduleGroupSessionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleGroupSessionParamsBuilder {}

impl ScheduleGroupSessionParamsBuilder {
    /// Consumes the builder and constructs a [`ScheduleGroupSessionParams`].
    pub fn build(self) -> Result<ScheduleGroupSessionParams, BuildError> {
        Ok(ScheduleGroupSessionParams {})
    }
}
