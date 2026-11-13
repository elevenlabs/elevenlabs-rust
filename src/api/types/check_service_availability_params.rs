pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckServiceAvailabilityParams {}

impl CheckServiceAvailabilityParams {
    pub fn builder() -> CheckServiceAvailabilityParamsBuilder {
        <CheckServiceAvailabilityParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckServiceAvailabilityParamsBuilder {}

impl CheckServiceAvailabilityParamsBuilder {
    /// Consumes the builder and constructs a [`CheckServiceAvailabilityParams`].
    pub fn build(self) -> Result<CheckServiceAvailabilityParams, BuildError> {
        Ok(CheckServiceAvailabilityParams {})
    }
}
