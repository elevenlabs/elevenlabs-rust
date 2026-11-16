pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteTransferRuleParams {}

impl DeleteTransferRuleParams {
    pub fn builder() -> DeleteTransferRuleParamsBuilder {
        <DeleteTransferRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteTransferRuleParamsBuilder {}

impl DeleteTransferRuleParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteTransferRuleParams`].
    pub fn build(self) -> Result<DeleteTransferRuleParams, BuildError> {
        Ok(DeleteTransferRuleParams {})
    }
}
