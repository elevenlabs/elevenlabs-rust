pub use crate::prelude::*;

/// Permanently remove a previously-cancelled group session.
///
/// Group analogue of ``delete_calendar_event``: cancel
/// (``cancel_group_session_for_all``) is the soft, history-preserving step;
/// this tool is the irreversible follow-up that drops the row from Mongo
/// and the staff Google Calendar entirely. The backend rejects the call
/// (422) if the session hasn't been cancelled yet, so the only safe path
/// is cancel-then-delete.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteGroupSessionParams {}

impl DeleteGroupSessionParams {
    pub fn builder() -> DeleteGroupSessionParamsBuilder {
        <DeleteGroupSessionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteGroupSessionParamsBuilder {}

impl DeleteGroupSessionParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteGroupSessionParams`].
    pub fn build(self) -> Result<DeleteGroupSessionParams, BuildError> {
        Ok(DeleteGroupSessionParams {})
    }
}
