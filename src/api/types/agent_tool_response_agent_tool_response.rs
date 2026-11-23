pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentToolResponseAgentToolResponse {
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_type: String,
    #[serde(default)]
    pub is_error: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(default)]
    pub is_called: bool,
    /// Tool call status derived from execution flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentToolResponseAgentToolResponseStatus>,
}

impl AgentToolResponseAgentToolResponse {
    pub fn builder() -> AgentToolResponseAgentToolResponseBuilder {
        <AgentToolResponseAgentToolResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolResponseAgentToolResponseBuilder {
    tool_name: Option<String>,
    tool_call_id: Option<String>,
    tool_type: Option<String>,
    is_error: Option<bool>,
    is_blocked: Option<bool>,
    event_id: Option<i64>,
    is_called: Option<bool>,
    status: Option<AgentToolResponseAgentToolResponseStatus>,
}

impl AgentToolResponseAgentToolResponseBuilder {
    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_type(mut self, value: impl Into<String>) -> Self {
        self.tool_type = Some(value.into());
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn is_blocked(mut self, value: bool) -> Self {
        self.is_blocked = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn is_called(mut self, value: bool) -> Self {
        self.is_called = Some(value);
        self
    }

    pub fn status(mut self, value: AgentToolResponseAgentToolResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolResponseAgentToolResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](AgentToolResponseAgentToolResponseBuilder::tool_name)
    /// - [`tool_call_id`](AgentToolResponseAgentToolResponseBuilder::tool_call_id)
    /// - [`tool_type`](AgentToolResponseAgentToolResponseBuilder::tool_type)
    /// - [`is_error`](AgentToolResponseAgentToolResponseBuilder::is_error)
    /// - [`event_id`](AgentToolResponseAgentToolResponseBuilder::event_id)
    /// - [`is_called`](AgentToolResponseAgentToolResponseBuilder::is_called)
    pub fn build(self) -> Result<AgentToolResponseAgentToolResponse, BuildError> {
        Ok(AgentToolResponseAgentToolResponse {
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_type: self
                .tool_type
                .ok_or_else(|| BuildError::missing_field("tool_type"))?,
            is_error: self
                .is_error
                .ok_or_else(|| BuildError::missing_field("is_error"))?,
            is_blocked: self.is_blocked,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            is_called: self
                .is_called
                .ok_or_else(|| BuildError::missing_field("is_called"))?,
            status: self.status,
        })
    }
}
