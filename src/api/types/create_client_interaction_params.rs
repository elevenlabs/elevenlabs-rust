pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateClientInteractionParams {}

impl CreateClientInteractionParams {
    pub fn builder() -> CreateClientInteractionParamsBuilder {
        <CreateClientInteractionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientInteractionParamsBuilder {}

impl CreateClientInteractionParamsBuilder {
    /// Consumes the builder and constructs a [`CreateClientInteractionParams`].
    pub fn build(self) -> Result<CreateClientInteractionParams, BuildError> {
        Ok(CreateClientInteractionParams {})
    }
}
