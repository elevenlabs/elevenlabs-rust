pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentProcedureParams {}

impl UpdateAgentProcedureParams {
    pub fn builder() -> UpdateAgentProcedureParamsBuilder {
        <UpdateAgentProcedureParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentProcedureParamsBuilder {}

impl UpdateAgentProcedureParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateAgentProcedureParams`].
    pub fn build(self) -> Result<UpdateAgentProcedureParams, BuildError> {
        Ok(UpdateAgentProcedureParams {})
    }
}
