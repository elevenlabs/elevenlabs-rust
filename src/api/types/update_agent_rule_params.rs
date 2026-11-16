pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentRuleParams {}

impl UpdateAgentRuleParams {
    pub fn builder() -> UpdateAgentRuleParamsBuilder {
        <UpdateAgentRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentRuleParamsBuilder {}

impl UpdateAgentRuleParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateAgentRuleParams`].
    pub fn build(self) -> Result<UpdateAgentRuleParams, BuildError> {
        Ok(UpdateAgentRuleParams {})
    }
}
