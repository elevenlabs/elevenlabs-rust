pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Close {}

impl Close {
    pub fn builder() -> CloseBuilder {
        <CloseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloseBuilder {}

impl CloseBuilder {
    /// Consumes the builder and constructs a [`Close`].
    pub fn build(self) -> Result<Close, BuildError> {
        Ok(Close {})
    }
}
