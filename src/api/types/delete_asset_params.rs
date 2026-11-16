pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAssetParams {}

impl DeleteAssetParams {
    pub fn builder() -> DeleteAssetParamsBuilder {
        <DeleteAssetParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAssetParamsBuilder {}

impl DeleteAssetParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteAssetParams`].
    pub fn build(self) -> Result<DeleteAssetParams, BuildError> {
        Ok(DeleteAssetParams {})
    }
}
