pub use crate::prelude::*;

/// Query parameters for Delete Tool
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsDeleteQueryRequest {
    /// If set to true, the tool will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl ToolsDeleteQueryRequest {
    pub fn builder() -> ToolsDeleteQueryRequestBuilder {
        <ToolsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl ToolsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsDeleteQueryRequest`].
    pub fn build(self) -> Result<ToolsDeleteQueryRequest, BuildError> {
        Ok(ToolsDeleteQueryRequest { force: self.force })
    }
}
