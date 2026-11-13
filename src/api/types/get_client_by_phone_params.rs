pub use crate::prelude::*;

/// Look up a client by their exact phone number.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetClientByPhoneParams {}

impl GetClientByPhoneParams {
    pub fn builder() -> GetClientByPhoneParamsBuilder {
        <GetClientByPhoneParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetClientByPhoneParamsBuilder {}

impl GetClientByPhoneParamsBuilder {
    /// Consumes the builder and constructs a [`GetClientByPhoneParams`].
    pub fn build(self) -> Result<GetClientByPhoneParams, BuildError> {
        Ok(GetClientByPhoneParams {})
    }
}
