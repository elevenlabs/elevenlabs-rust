pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBusinessInfoParams {}

impl UpdateBusinessInfoParams {
    pub fn builder() -> UpdateBusinessInfoParamsBuilder {
        <UpdateBusinessInfoParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBusinessInfoParamsBuilder {}

impl UpdateBusinessInfoParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateBusinessInfoParams`].
    pub fn build(self) -> Result<UpdateBusinessInfoParams, BuildError> {
        Ok(UpdateBusinessInfoParams {})
    }
}
