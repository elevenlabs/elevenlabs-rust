pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwaitingApproval {
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
    /// Timeout in seconds for user approval
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub approval_timeout_secs: Option<i64>,
}

impl AwaitingApproval {
    pub fn builder() -> AwaitingApprovalBuilder {
        <AwaitingApprovalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AwaitingApprovalBuilder {
    service_id: Option<String>,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    tool_description: Option<String>,
    parameters: Option<serde_json::Value>,
    timestamp: Option<String>,
    approval_timeout_secs: Option<i64>,
}

impl AwaitingApprovalBuilder {
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

    pub fn approval_timeout_secs(mut self, value: i64) -> Self {
        self.approval_timeout_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AwaitingApproval`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](AwaitingApprovalBuilder::service_id)
    /// - [`tool_call_id`](AwaitingApprovalBuilder::tool_call_id)
    /// - [`tool_name`](AwaitingApprovalBuilder::tool_name)
    /// - [`parameters`](AwaitingApprovalBuilder::parameters)
    pub fn build(self) -> Result<AwaitingApproval, BuildError> {
        Ok(AwaitingApproval {
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
            approval_timeout_secs: self.approval_timeout_secs,
        })
    }
}
