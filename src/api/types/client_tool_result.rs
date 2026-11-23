pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientToolResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ClientToolResultItemType>,
    /// ID of the tool call this result corresponds to.
    pub tool_call_id: String,
    pub result: String,
    pub is_error: bool,
    /// Optional category for the failure reason. When set, `is_error` is treated as true. `user_rejected` is special-cased to mark the tool as not having been called.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<ClientToolResultErrorType>,
}

impl ClientToolResult {
    pub fn builder() -> ClientToolResultBuilder {
        <ClientToolResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolResultBuilder {
    r#type: Option<ClientToolResultItemType>,
    tool_call_id: Option<String>,
    result: Option<String>,
    is_error: Option<bool>,
    error_type: Option<ClientToolResultErrorType>,
}

impl ClientToolResultBuilder {
    pub fn r#type(mut self, value: ClientToolResultItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn result(mut self, value: impl Into<String>) -> Self {
        self.result = Some(value.into());
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn error_type(mut self, value: ClientToolResultErrorType) -> Self {
        self.error_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](ClientToolResultBuilder::tool_call_id)
    /// - [`result`](ClientToolResultBuilder::result)
    /// - [`is_error`](ClientToolResultBuilder::is_error)
    pub fn build(self) -> Result<ClientToolResult, BuildError> {
        Ok(ClientToolResult {
            r#type: self.r#type,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            is_error: self
                .is_error
                .ok_or_else(|| BuildError::missing_field("is_error"))?,
            error_type: self.error_type,
        })
    }
}
