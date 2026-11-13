pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLocationParams {}

impl UpdateLocationParams {
    pub fn builder() -> UpdateLocationParamsBuilder {
        <UpdateLocationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocationParamsBuilder {}

impl UpdateLocationParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateLocationParams`].
    pub fn build(self) -> Result<UpdateLocationParams, BuildError> {
        Ok(UpdateLocationParams {})
    }
}
