pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SongMetadata {}

impl SongMetadata {
    pub fn builder() -> SongMetadataBuilder {
        <SongMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SongMetadataBuilder {}

impl SongMetadataBuilder {
    /// Consumes the builder and constructs a [`SongMetadata`].
    pub fn build(self) -> Result<SongMetadata, BuildError> {
        Ok(SongMetadata {})
    }
}
