pub use crate::prelude::*;

/// Every location, inactive ones included, with every owner-editable field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnerListLocationsParams {}

impl OwnerListLocationsParams {
    pub fn builder() -> OwnerListLocationsParamsBuilder {
        <OwnerListLocationsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerListLocationsParamsBuilder {}

impl OwnerListLocationsParamsBuilder {
    /// Consumes the builder and constructs a [`OwnerListLocationsParams`].
    pub fn build(self) -> Result<OwnerListLocationsParams, BuildError> {
        Ok(OwnerListLocationsParams {})
    }
}
