pub use crate::prelude::*;

/// Register a client for a scheduled group session.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterForGroupSessionParams {}

impl RegisterForGroupSessionParams {
    pub fn builder() -> RegisterForGroupSessionParamsBuilder {
        <RegisterForGroupSessionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterForGroupSessionParamsBuilder {}

impl RegisterForGroupSessionParamsBuilder {
    /// Consumes the builder and constructs a [`RegisterForGroupSessionParams`].
    pub fn build(self) -> Result<RegisterForGroupSessionParams, BuildError> {
        Ok(RegisterForGroupSessionParams {})
    }
}
