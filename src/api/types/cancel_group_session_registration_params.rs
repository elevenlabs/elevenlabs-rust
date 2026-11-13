pub use crate::prelude::*;

/// Cancel a single client's registration for a group session.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelGroupSessionRegistrationParams {}

impl CancelGroupSessionRegistrationParams {
    pub fn builder() -> CancelGroupSessionRegistrationParamsBuilder {
        <CancelGroupSessionRegistrationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelGroupSessionRegistrationParamsBuilder {}

impl CancelGroupSessionRegistrationParamsBuilder {
    /// Consumes the builder and constructs a [`CancelGroupSessionRegistrationParams`].
    pub fn build(self) -> Result<CancelGroupSessionRegistrationParams, BuildError> {
        Ok(CancelGroupSessionRegistrationParams {})
    }
}
