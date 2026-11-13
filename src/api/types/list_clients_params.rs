pub use crate::prelude::*;

/// List clients ordered by most recently updated, with an optional limit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientsParams {}

impl ListClientsParams {
    pub fn builder() -> ListClientsParamsBuilder {
        <ListClientsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientsParamsBuilder {}

impl ListClientsParamsBuilder {
    /// Consumes the builder and constructs a [`ListClientsParams`].
    pub fn build(self) -> Result<ListClientsParams, BuildError> {
        Ok(ListClientsParams {})
    }
}
