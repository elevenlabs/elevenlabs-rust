pub use crate::prelude::*;

/// Query parameters for Get Knowledge Base Summaries By Ids
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SummariesGetQueryRequest2 {
    /// The ids of knowledge base documents.
    pub document_ids: Vec<String>,
}

impl SummariesGetQueryRequest2 {
    pub fn builder() -> SummariesGetQueryRequest2Builder {
        <SummariesGetQueryRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummariesGetQueryRequest2Builder {
    document_ids: Option<Vec<String>>,
}

impl SummariesGetQueryRequest2Builder {
    pub fn document_ids(mut self, value: Vec<String>) -> Self {
        self.document_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummariesGetQueryRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](SummariesGetQueryRequest2Builder::document_ids)
    pub fn build(self) -> Result<SummariesGetQueryRequest2, BuildError> {
        Ok(SummariesGetQueryRequest2 {
            document_ids: self
                .document_ids
                .ok_or_else(|| BuildError::missing_field("document_ids"))?,
        })
    }
}
