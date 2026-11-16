pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientInteractionsParams {}

impl ListClientInteractionsParams {
    pub fn builder() -> ListClientInteractionsParamsBuilder {
        <ListClientInteractionsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientInteractionsParamsBuilder {}

impl ListClientInteractionsParamsBuilder {
    /// Consumes the builder and constructs a [`ListClientInteractionsParams`].
    pub fn build(self) -> Result<ListClientInteractionsParams, BuildError> {
        Ok(ListClientInteractionsParams {})
    }
}
