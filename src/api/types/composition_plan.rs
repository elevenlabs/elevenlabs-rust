pub use crate::prelude::*;

/// Composition plan for the `music_v2` and `music_v2_5` models. Using this field with any other model will result in an error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompositionPlan {
    /// The chunks that make up the generation.
    pub chunks: Vec<ChunksItem>,
}

impl CompositionPlan {
    pub fn builder() -> CompositionPlanBuilder {
        <CompositionPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompositionPlanBuilder {
    chunks: Option<Vec<ChunksItem>>,
}

impl CompositionPlanBuilder {
    pub fn chunks(mut self, value: Vec<ChunksItem>) -> Self {
        self.chunks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompositionPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](CompositionPlanBuilder::chunks)
    pub fn build(self) -> Result<CompositionPlan, BuildError> {
        Ok(CompositionPlan {
            chunks: self
                .chunks
                .ok_or_else(|| BuildError::missing_field("chunks"))?,
        })
    }
}
