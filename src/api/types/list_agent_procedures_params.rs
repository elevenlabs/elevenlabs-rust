pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentProceduresParams {}

impl ListAgentProceduresParams {
    pub fn builder() -> ListAgentProceduresParamsBuilder {
        <ListAgentProceduresParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentProceduresParamsBuilder {}

impl ListAgentProceduresParamsBuilder {
    /// Consumes the builder and constructs a [`ListAgentProceduresParams`].
    pub fn build(self) -> Result<ListAgentProceduresParams, BuildError> {
        Ok(ListAgentProceduresParams {})
    }
}
