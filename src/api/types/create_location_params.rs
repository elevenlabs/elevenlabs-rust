pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLocationParams {}

impl CreateLocationParams {
    pub fn builder() -> CreateLocationParamsBuilder {
        <CreateLocationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLocationParamsBuilder {}

impl CreateLocationParamsBuilder {
    /// Consumes the builder and constructs a [`CreateLocationParams`].
    pub fn build(self) -> Result<CreateLocationParams, BuildError> {
        Ok(CreateLocationParams {})
    }
}
