pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentRuleParams {}

impl CreateAgentRuleParams {
    pub fn builder() -> CreateAgentRuleParamsBuilder {
        <CreateAgentRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRuleParamsBuilder {}

impl CreateAgentRuleParamsBuilder {
    /// Consumes the builder and constructs a [`CreateAgentRuleParams`].
    pub fn build(self) -> Result<CreateAgentRuleParams, BuildError> {
        Ok(CreateAgentRuleParams {})
    }
}
