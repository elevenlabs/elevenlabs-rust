pub use crate::prelude::*;

/// Query parameters for Get Tool
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsGetQueryRequest {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl ToolsGetQueryRequest {
    pub fn builder() -> ToolsGetQueryRequestBuilder {
        <ToolsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsGetQueryRequestBuilder {
    environment: Option<String>,
}

impl ToolsGetQueryRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolsGetQueryRequest`].
    pub fn build(self) -> Result<ToolsGetQueryRequest, BuildError> {
        Ok(ToolsGetQueryRequest {
            environment: self.environment,
        })
    }
}
