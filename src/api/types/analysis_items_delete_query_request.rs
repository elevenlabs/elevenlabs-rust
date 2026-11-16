pub use crate::prelude::*;

/// Query parameters for Delete Analysis Item
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisItemsDeleteQueryRequest {
    /// If set to true, the analysis item will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl AnalysisItemsDeleteQueryRequest {
    pub fn builder() -> AnalysisItemsDeleteQueryRequestBuilder {
        <AnalysisItemsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl AnalysisItemsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemsDeleteQueryRequest`].
    pub fn build(self) -> Result<AnalysisItemsDeleteQueryRequest, BuildError> {
        Ok(AnalysisItemsDeleteQueryRequest { force: self.force })
    }
}
