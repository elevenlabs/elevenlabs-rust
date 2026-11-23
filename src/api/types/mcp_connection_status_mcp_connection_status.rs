pub use crate::prelude::*;

/// MCP connection status with array of integration statuses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpConnectionStatusMcpConnectionStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<IntegrationsItem>>,
}

impl McpConnectionStatusMcpConnectionStatus {
    pub fn builder() -> McpConnectionStatusMcpConnectionStatusBuilder {
        <McpConnectionStatusMcpConnectionStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpConnectionStatusMcpConnectionStatusBuilder {
    integrations: Option<Vec<IntegrationsItem>>,
}

impl McpConnectionStatusMcpConnectionStatusBuilder {
    pub fn integrations(mut self, value: Vec<IntegrationsItem>) -> Self {
        self.integrations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpConnectionStatusMcpConnectionStatus`].
    pub fn build(self) -> Result<McpConnectionStatusMcpConnectionStatus, BuildError> {
        Ok(McpConnectionStatusMcpConnectionStatus {
            integrations: self.integrations,
        })
    }
}
