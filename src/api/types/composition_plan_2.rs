pub use crate::prelude::*;

/// Composition plan for the `music_v2` and `music_v2_5` models. Using this field with any other model will result in an error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompositionPlan2 {
    /// The chunks that make up the generation.
    pub chunks: Vec<ChunksItem>,
}

impl CompositionPlan2 {
    pub fn builder() -> CompositionPlan2Builder {
        <CompositionPlan2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompositionPlan2Builder {
    chunks: Option<Vec<ChunksItem>>,
}

impl CompositionPlan2Builder {
    pub fn chunks(mut self, value: Vec<ChunksItem>) -> Self {
        self.chunks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompositionPlan2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](CompositionPlan2Builder::chunks)
    pub fn build(self) -> Result<CompositionPlan2, BuildError> {
        Ok(CompositionPlan2 {
            chunks: self
                .chunks
                .ok_or_else(|| BuildError::missing_field("chunks"))?,
        })
    }
}
