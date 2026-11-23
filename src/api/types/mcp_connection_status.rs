pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct McpConnectionStatus {
    /// MCP connection status with array of integration statuses.
    #[serde(default)]
    pub mcp_connection_status: McpConnectionStatusMcpConnectionStatus,
}

impl McpConnectionStatus {
    pub fn builder() -> McpConnectionStatusBuilder {
        <McpConnectionStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpConnectionStatusBuilder {
    mcp_connection_status: Option<McpConnectionStatusMcpConnectionStatus>,
}

impl McpConnectionStatusBuilder {
    pub fn mcp_connection_status(mut self, value: McpConnectionStatusMcpConnectionStatus) -> Self {
        self.mcp_connection_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpConnectionStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mcp_connection_status`](McpConnectionStatusBuilder::mcp_connection_status)
    pub fn build(self) -> Result<McpConnectionStatus, BuildError> {
        Ok(McpConnectionStatus {
            mcp_connection_status: self
                .mcp_connection_status
                .ok_or_else(|| BuildError::missing_field("mcp_connection_status"))?,
        })
    }
}
