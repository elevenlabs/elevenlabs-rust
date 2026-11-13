pub use crate::prelude::*;

/// Query parameters for Search Knowledge Base Content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeBaseSearchQueryRequest {
    /// The search query text
    pub query: String,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// If present, the endpoint will return only documents of the given types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<KnowledgeBaseDocumentType>>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl KnowledgeBaseSearchQueryRequest {
    pub fn builder() -> KnowledgeBaseSearchQueryRequestBuilder {
        <KnowledgeBaseSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseSearchQueryRequestBuilder {
    query: Option<String>,
    page_size: Option<i64>,
    types: Option<Vec<KnowledgeBaseDocumentType>>,
    cursor: Option<String>,
}

impl KnowledgeBaseSearchQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn types(mut self, value: Vec<KnowledgeBaseDocumentType>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](KnowledgeBaseSearchQueryRequestBuilder::query)
    pub fn build(self) -> Result<KnowledgeBaseSearchQueryRequest, BuildError> {
        Ok(KnowledgeBaseSearchQueryRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            page_size: self.page_size,
            types: self.types,
            cursor: self.cursor,
        })
    }
}
