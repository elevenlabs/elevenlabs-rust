pub use crate::prelude::*;

/// Request for Create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolConfigsCreateRequest {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip)]
    pub environment: Option<String>,
    pub body: McpToolConfigOverrideCreateRequestModel,
}

impl ToolConfigsCreateRequest {
    pub fn builder() -> ToolConfigsCreateRequestBuilder {
        <ToolConfigsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolConfigsCreateRequestBuilder {
    environment: Option<String>,
    body: Option<McpToolConfigOverrideCreateRequestModel>,
}

impl ToolConfigsCreateRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn body(mut self, value: McpToolConfigOverrideCreateRequestModel) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolConfigsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ToolConfigsCreateRequestBuilder::body)
    pub fn build(self) -> Result<ToolConfigsCreateRequest, BuildError> {
        Ok(ToolConfigsCreateRequest {
            environment: self.environment,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
