pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TriggerUserVerificationParams {}

impl TriggerUserVerificationParams {
    pub fn builder() -> TriggerUserVerificationParamsBuilder {
        <TriggerUserVerificationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerUserVerificationParamsBuilder {}

impl TriggerUserVerificationParamsBuilder {
    /// Consumes the builder and constructs a [`TriggerUserVerificationParams`].
    pub fn build(self) -> Result<TriggerUserVerificationParams, BuildError> {
        Ok(TriggerUserVerificationParams {})
    }
}
