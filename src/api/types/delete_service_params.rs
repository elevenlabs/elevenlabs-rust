pub use crate::prelude::*;

/// Delete an existing service from the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteServiceParams {}

impl DeleteServiceParams {
    pub fn builder() -> DeleteServiceParamsBuilder {
        <DeleteServiceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteServiceParamsBuilder {}

impl DeleteServiceParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteServiceParams`].
    pub fn build(self) -> Result<DeleteServiceParams, BuildError> {
        Ok(DeleteServiceParams {})
    }
}
