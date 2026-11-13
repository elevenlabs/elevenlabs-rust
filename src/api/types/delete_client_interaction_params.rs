pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteClientInteractionParams {}

impl DeleteClientInteractionParams {
    pub fn builder() -> DeleteClientInteractionParamsBuilder {
        <DeleteClientInteractionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteClientInteractionParamsBuilder {}

impl DeleteClientInteractionParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteClientInteractionParams`].
    pub fn build(self) -> Result<DeleteClientInteractionParams, BuildError> {
        Ok(DeleteClientInteractionParams {})
    }
}
