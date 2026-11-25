pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingEndOfStream {}

impl SubtitlingEndOfStream {
    pub fn builder() -> SubtitlingEndOfStreamBuilder {
        <SubtitlingEndOfStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingEndOfStreamBuilder {}

impl SubtitlingEndOfStreamBuilder {
    /// Consumes the builder and constructs a [`SubtitlingEndOfStream`].
    pub fn build(self) -> Result<SubtitlingEndOfStream, BuildError> {
        Ok(SubtitlingEndOfStream {})
    }
}
