pub use crate::prelude::*;

/// Update an existing service's information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateServiceParams {}

impl UpdateServiceParams {
    pub fn builder() -> UpdateServiceParamsBuilder {
        <UpdateServiceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateServiceParamsBuilder {}

impl UpdateServiceParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateServiceParams`].
    pub fn build(self) -> Result<UpdateServiceParams, BuildError> {
        Ok(UpdateServiceParams {})
    }
}
