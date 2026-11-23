pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Loading {
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_description: Option<String>,
    #[serde(default)]
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl Loading {
    pub fn builder() -> LoadingBuilder {
        <LoadingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoadingBuilder {
    service_id: Option<String>,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    tool_description: Option<String>,
    parameters: Option<serde_json::Value>,
    timestamp: Option<String>,
}

impl LoadingBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_description(mut self, value: impl Into<String>) -> Self {
        self.tool_description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Loading`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](LoadingBuilder::service_id)
    /// - [`tool_call_id`](LoadingBuilder::tool_call_id)
    /// - [`tool_name`](LoadingBuilder::tool_name)
    /// - [`parameters`](LoadingBuilder::parameters)
    pub fn build(self) -> Result<Loading, BuildError> {
        Ok(Loading {
            service_id: self
                .service_id
                .ok_or_else(|| BuildError::missing_field("service_id"))?,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_description: self.tool_description,
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
            timestamp: self.timestamp,
        })
    }
}
