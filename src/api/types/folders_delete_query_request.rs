pub use crate::prelude::*;

/// Query parameters for Delete Agent Test Folder
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FoldersDeleteQueryRequest {
    /// Force delete. Required for deleting non-empty folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl FoldersDeleteQueryRequest {
    pub fn builder() -> FoldersDeleteQueryRequestBuilder {
        <FoldersDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FoldersDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl FoldersDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FoldersDeleteQueryRequest`].
    pub fn build(self) -> Result<FoldersDeleteQueryRequest, BuildError> {
        Ok(FoldersDeleteQueryRequest { force: self.force })
    }
}
