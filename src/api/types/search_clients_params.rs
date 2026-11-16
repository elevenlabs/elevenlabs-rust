pub use crate::prelude::*;

/// Search for clients by name, phone number, or email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchClientsParams {}

impl SearchClientsParams {
    pub fn builder() -> SearchClientsParamsBuilder {
        <SearchClientsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchClientsParamsBuilder {}

impl SearchClientsParamsBuilder {
    /// Consumes the builder and constructs a [`SearchClientsParams`].
    pub fn build(self) -> Result<SearchClientsParams, BuildError> {
        Ok(SearchClientsParams {})
    }
}
