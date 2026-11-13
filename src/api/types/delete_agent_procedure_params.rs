pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAgentProcedureParams {}

impl DeleteAgentProcedureParams {
    pub fn builder() -> DeleteAgentProcedureParamsBuilder {
        <DeleteAgentProcedureParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAgentProcedureParamsBuilder {}

impl DeleteAgentProcedureParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteAgentProcedureParams`].
    pub fn build(self) -> Result<DeleteAgentProcedureParams, BuildError> {
        Ok(DeleteAgentProcedureParams {})
    }
}
