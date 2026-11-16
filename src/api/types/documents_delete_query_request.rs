pub use crate::prelude::*;

/// Query parameters for Delete Knowledge Base Document Or Folder
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentsDeleteQueryRequest {
    /// If set to true, the document or folder will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents. For non-empty folders, this will also delete all child documents and folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl DocumentsDeleteQueryRequest {
    pub fn builder() -> DocumentsDeleteQueryRequestBuilder {
        <DocumentsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl DocumentsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsDeleteQueryRequest`].
    pub fn build(self) -> Result<DocumentsDeleteQueryRequest, BuildError> {
        Ok(DocumentsDeleteQueryRequest { force: self.force })
    }
}
