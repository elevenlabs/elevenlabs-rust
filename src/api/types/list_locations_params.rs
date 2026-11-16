pub use crate::prelude::*;

/// Params + Reception-proxied ``run()`` for list_locations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLocationsParams {}

impl ListLocationsParams {
    pub fn builder() -> ListLocationsParamsBuilder {
        <ListLocationsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLocationsParamsBuilder {}

impl ListLocationsParamsBuilder {
    /// Consumes the builder and constructs a [`ListLocationsParams`].
    pub fn build(self) -> Result<ListLocationsParams, BuildError> {
        Ok(ListLocationsParams {})
    }
}
