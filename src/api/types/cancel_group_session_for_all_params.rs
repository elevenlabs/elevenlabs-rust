pub use crate::prelude::*;

/// Cancel an entire group session and notify every registered participant.
/// Destructive -- prefer ``cancel_group_session_registration`` for cancelling
/// a single attendee.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelGroupSessionForAllParams {}

impl CancelGroupSessionForAllParams {
    pub fn builder() -> CancelGroupSessionForAllParamsBuilder {
        <CancelGroupSessionForAllParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelGroupSessionForAllParamsBuilder {}

impl CancelGroupSessionForAllParamsBuilder {
    /// Consumes the builder and constructs a [`CancelGroupSessionForAllParams`].
    pub fn build(self) -> Result<CancelGroupSessionForAllParams, BuildError> {
        Ok(CancelGroupSessionForAllParams {})
    }
}
