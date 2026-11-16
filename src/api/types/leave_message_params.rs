pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaveMessageParams {}

impl LeaveMessageParams {
    pub fn builder() -> LeaveMessageParamsBuilder {
        <LeaveMessageParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaveMessageParamsBuilder {}

impl LeaveMessageParamsBuilder {
    /// Consumes the builder and constructs a [`LeaveMessageParams`].
    pub fn build(self) -> Result<LeaveMessageParams, BuildError> {
        Ok(LeaveMessageParams {})
    }
}
