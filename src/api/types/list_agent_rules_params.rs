pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentRulesParams {}

impl ListAgentRulesParams {
    pub fn builder() -> ListAgentRulesParamsBuilder {
        <ListAgentRulesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentRulesParamsBuilder {}

impl ListAgentRulesParamsBuilder {
    /// Consumes the builder and constructs a [`ListAgentRulesParams`].
    pub fn build(self) -> Result<ListAgentRulesParams, BuildError> {
        Ok(ListAgentRulesParams {})
    }
}
