pub use crate::prelude::*;

/// Create a new client in the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateClientParams {}

impl CreateClientParams {
    pub fn builder() -> CreateClientParamsBuilder {
        <CreateClientParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientParamsBuilder {}

impl CreateClientParamsBuilder {
    /// Consumes the builder and constructs a [`CreateClientParams`].
    pub fn build(self) -> Result<CreateClientParams, BuildError> {
        Ok(CreateClientParams {})
    }
}
