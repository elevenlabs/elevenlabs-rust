pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Failure {
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
    #[serde(default)]
    pub error_message: String,
}

impl Failure {
    pub fn builder() -> FailureBuilder {
        <FailureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FailureBuilder {
    service_id: Option<String>,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    tool_description: Option<String>,
    parameters: Option<serde_json::Value>,
    timestamp: Option<String>,
    error_message: Option<String>,
}

impl FailureBuilder {
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

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Failure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](FailureBuilder::service_id)
    /// - [`tool_call_id`](FailureBuilder::tool_call_id)
    /// - [`tool_name`](FailureBuilder::tool_name)
    /// - [`parameters`](FailureBuilder::parameters)
    /// - [`error_message`](FailureBuilder::error_message)
    pub fn build(self) -> Result<Failure, BuildError> {
        Ok(Failure {
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
            error_message: self
                .error_message
                .ok_or_else(|| BuildError::missing_field("error_message"))?,
        })
    }
}
