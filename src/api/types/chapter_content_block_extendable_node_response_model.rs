pub use crate::prelude::*;

/// Not used. Make sure you anticipate new types in the future.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChapterContentBlockExtendableNodeResponseModel {}

impl ChapterContentBlockExtendableNodeResponseModel {
    pub fn builder() -> ChapterContentBlockExtendableNodeResponseModelBuilder {
        <ChapterContentBlockExtendableNodeResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentBlockExtendableNodeResponseModelBuilder {}

impl ChapterContentBlockExtendableNodeResponseModelBuilder {
    /// Consumes the builder and constructs a [`ChapterContentBlockExtendableNodeResponseModel`].
    pub fn build(self) -> Result<ChapterContentBlockExtendableNodeResponseModel, BuildError> {
        Ok(ChapterContentBlockExtendableNodeResponseModel {})
    }
}
