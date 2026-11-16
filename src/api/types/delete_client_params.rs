pub use crate::prelude::*;

/// Delete a client, cascading deletion of all their appointments first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteClientParams {}

impl DeleteClientParams {
    pub fn builder() -> DeleteClientParamsBuilder {
        <DeleteClientParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteClientParamsBuilder {}

impl DeleteClientParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteClientParams`].
    pub fn build(self) -> Result<DeleteClientParams, BuildError> {
        Ok(DeleteClientParams {})
    }
}
