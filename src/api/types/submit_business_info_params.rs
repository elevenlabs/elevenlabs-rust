pub use crate::prelude::*;

/// Finalize the onboarding interview: create a text knowledge source and mark done.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitBusinessInfoParams {}

impl SubmitBusinessInfoParams {
    pub fn builder() -> SubmitBusinessInfoParamsBuilder {
        <SubmitBusinessInfoParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitBusinessInfoParamsBuilder {}

impl SubmitBusinessInfoParamsBuilder {
    /// Consumes the builder and constructs a [`SubmitBusinessInfoParams`].
    pub fn build(self) -> Result<SubmitBusinessInfoParams, BuildError> {
        Ok(SubmitBusinessInfoParams {})
    }
}
