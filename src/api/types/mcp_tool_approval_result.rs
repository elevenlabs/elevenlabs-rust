pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct McpToolApprovalResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<McpToolApprovalResultType>,
    /// The ID of the tool call this approval responds to.
    pub tool_call_id: String,
    /// Whether the user approved the tool call.
    pub is_approved: bool,
}

impl McpToolApprovalResult {
    pub fn builder() -> McpToolApprovalResultBuilder {
        <McpToolApprovalResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolApprovalResultBuilder {
    r#type: Option<McpToolApprovalResultType>,
    tool_call_id: Option<String>,
    is_approved: Option<bool>,
}

impl McpToolApprovalResultBuilder {
    pub fn r#type(mut self, value: McpToolApprovalResultType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn is_approved(mut self, value: bool) -> Self {
        self.is_approved = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolApprovalResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](McpToolApprovalResultBuilder::tool_call_id)
    /// - [`is_approved`](McpToolApprovalResultBuilder::is_approved)
    pub fn build(self) -> Result<McpToolApprovalResult, BuildError> {
        Ok(McpToolApprovalResult {
            r#type: self.r#type,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            is_approved: self
                .is_approved
                .ok_or_else(|| BuildError::missing_field("is_approved"))?,
        })
    }
}
