pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpToolCall {
    pub mcp_tool_call: McpToolCallMcpToolCall,
}

impl McpToolCall {
    pub fn builder() -> McpToolCallBuilder {
        <McpToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolCallBuilder {
    mcp_tool_call: Option<McpToolCallMcpToolCall>,
}

impl McpToolCallBuilder {
    pub fn mcp_tool_call(mut self, value: McpToolCallMcpToolCall) -> Self {
        self.mcp_tool_call = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mcp_tool_call`](McpToolCallBuilder::mcp_tool_call)
    pub fn build(self) -> Result<McpToolCall, BuildError> {
        Ok(McpToolCall {
            mcp_tool_call: self
                .mcp_tool_call
                .ok_or_else(|| BuildError::missing_field("mcp_tool_call"))?,
        })
    }
}
