pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAgentRuleParams {}

impl DeleteAgentRuleParams {
    pub fn builder() -> DeleteAgentRuleParamsBuilder {
        <DeleteAgentRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAgentRuleParamsBuilder {}

impl DeleteAgentRuleParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteAgentRuleParams`].
    pub fn build(self) -> Result<DeleteAgentRuleParams, BuildError> {
        Ok(DeleteAgentRuleParams {})
    }
}
