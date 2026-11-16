pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentConfigParams {}

impl UpdateAgentConfigParams {
    pub fn builder() -> UpdateAgentConfigParamsBuilder {
        <UpdateAgentConfigParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentConfigParamsBuilder {}

impl UpdateAgentConfigParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateAgentConfigParams`].
    pub fn build(self) -> Result<UpdateAgentConfigParams, BuildError> {
        Ok(UpdateAgentConfigParams {})
    }
}
