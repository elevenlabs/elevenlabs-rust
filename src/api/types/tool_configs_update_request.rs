pub use crate::prelude::*;

/// Request for Update (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolConfigsUpdateRequest {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip)]
    pub environment: Option<String>,
    pub body: McpToolConfigOverrideUpdateRequestModel,
}

impl ToolConfigsUpdateRequest {
    pub fn builder() -> ToolConfigsUpdateRequestBuilder {
        <ToolConfigsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolConfigsUpdateRequestBuilder {
    environment: Option<String>,
    body: Option<McpToolConfigOverrideUpdateRequestModel>,
}

impl ToolConfigsUpdateRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn body(mut self, value: McpToolConfigOverrideUpdateRequestModel) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolConfigsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ToolConfigsUpdateRequestBuilder::body)
    pub fn build(self) -> Result<ToolConfigsUpdateRequest, BuildError> {
        Ok(ToolConfigsUpdateRequest {
            environment: self.environment,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
