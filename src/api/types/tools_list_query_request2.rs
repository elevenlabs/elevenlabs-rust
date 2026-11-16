pub use crate::prelude::*;

/// Query parameters for List Mcp Server Tools
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsListQueryRequest2 {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl ToolsListQueryRequest2 {
    pub fn builder() -> ToolsListQueryRequest2Builder {
        <ToolsListQueryRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsListQueryRequest2Builder {
    environment: Option<String>,
}

impl ToolsListQueryRequest2Builder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolsListQueryRequest2`].
    pub fn build(self) -> Result<ToolsListQueryRequest2, BuildError> {
        Ok(ToolsListQueryRequest2 {
            environment: self.environment,
        })
    }
}
