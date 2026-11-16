pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendCustomEmailParams {}

impl SendCustomEmailParams {
    pub fn builder() -> SendCustomEmailParamsBuilder {
        <SendCustomEmailParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendCustomEmailParamsBuilder {}

impl SendCustomEmailParamsBuilder {
    /// Consumes the builder and constructs a [`SendCustomEmailParams`].
    pub fn build(self) -> Result<SendCustomEmailParams, BuildError> {
        Ok(SendCustomEmailParams {})
    }
}
