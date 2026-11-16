pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingEndOfStream {}

impl DubbingEndOfStream {
    pub fn builder() -> DubbingEndOfStreamBuilder {
        <DubbingEndOfStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingEndOfStreamBuilder {}

impl DubbingEndOfStreamBuilder {
    /// Consumes the builder and constructs a [`DubbingEndOfStream`].
    pub fn build(self) -> Result<DubbingEndOfStream, BuildError> {
        Ok(DubbingEndOfStream {})
    }
}
