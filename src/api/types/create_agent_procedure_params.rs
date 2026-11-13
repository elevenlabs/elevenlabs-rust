pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentProcedureParams {}

impl CreateAgentProcedureParams {
    pub fn builder() -> CreateAgentProcedureParamsBuilder {
        <CreateAgentProcedureParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentProcedureParamsBuilder {}

impl CreateAgentProcedureParamsBuilder {
    /// Consumes the builder and constructs a [`CreateAgentProcedureParams`].
    pub fn build(self) -> Result<CreateAgentProcedureParams, BuildError> {
        Ok(CreateAgentProcedureParams {})
    }
}
