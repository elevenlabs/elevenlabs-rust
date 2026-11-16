pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTransferRulesParams {}

impl ListTransferRulesParams {
    pub fn builder() -> ListTransferRulesParamsBuilder {
        <ListTransferRulesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTransferRulesParamsBuilder {}

impl ListTransferRulesParamsBuilder {
    /// Consumes the builder and constructs a [`ListTransferRulesParams`].
    pub fn build(self) -> Result<ListTransferRulesParams, BuildError> {
        Ok(ListTransferRulesParams {})
    }
}
