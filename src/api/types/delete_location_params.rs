pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteLocationParams {}

impl DeleteLocationParams {
    pub fn builder() -> DeleteLocationParamsBuilder {
        <DeleteLocationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteLocationParamsBuilder {}

impl DeleteLocationParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteLocationParams`].
    pub fn build(self) -> Result<DeleteLocationParams, BuildError> {
        Ok(DeleteLocationParams {})
    }
}
