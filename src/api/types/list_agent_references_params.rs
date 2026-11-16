pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentReferencesParams {}

impl ListAgentReferencesParams {
    pub fn builder() -> ListAgentReferencesParamsBuilder {
        <ListAgentReferencesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentReferencesParamsBuilder {}

impl ListAgentReferencesParamsBuilder {
    /// Consumes the builder and constructs a [`ListAgentReferencesParams`].
    pub fn build(self) -> Result<ListAgentReferencesParams, BuildError> {
        Ok(ListAgentReferencesParams {})
    }
}
