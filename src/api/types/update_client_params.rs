pub use crate::prelude::*;

/// Update an existing client's information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateClientParams {}

impl UpdateClientParams {
    pub fn builder() -> UpdateClientParamsBuilder {
        <UpdateClientParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateClientParamsBuilder {}

impl UpdateClientParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateClientParams`].
    pub fn build(self) -> Result<UpdateClientParams, BuildError> {
        Ok(UpdateClientParams {})
    }
}
