pub use crate::prelude::*;

/// Create a new service (classic, rental, or group) in the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServiceParams {}

impl CreateServiceParams {
    pub fn builder() -> CreateServiceParamsBuilder {
        <CreateServiceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceParamsBuilder {}

impl CreateServiceParamsBuilder {
    /// Consumes the builder and constructs a [`CreateServiceParams`].
    pub fn build(self) -> Result<CreateServiceParams, BuildError> {
        Ok(CreateServiceParams {})
    }
}
