pub use crate::prelude::*;

/// Query parameters for Get Documentation Chunk From Knowledge Base
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChunkGetQueryRequest {
    /// The embedding model used to retrieve the chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<EmbeddingModelEnum>,
}

impl ChunkGetQueryRequest {
    pub fn builder() -> ChunkGetQueryRequestBuilder {
        <ChunkGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChunkGetQueryRequestBuilder {
    embedding_model: Option<EmbeddingModelEnum>,
}

impl ChunkGetQueryRequestBuilder {
    pub fn embedding_model(mut self, value: EmbeddingModelEnum) -> Self {
        self.embedding_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChunkGetQueryRequest`].
    pub fn build(self) -> Result<ChunkGetQueryRequest, BuildError> {
        Ok(ChunkGetQueryRequest {
            embedding_model: self.embedding_model,
        })
    }
}
