pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponse {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
}

impl AgentResponse {
    pub fn builder() -> AgentResponseBuilder {
        <AgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseBuilder {
    event_id: Option<i64>,
    content: Option<String>,
    is_final: Option<bool>,
}

impl AgentResponseBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](AgentResponseBuilder::event_id)
    pub fn build(self) -> Result<AgentResponse, BuildError> {
        Ok(AgentResponse {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            content: self.content,
            is_final: self.is_final,
        })
    }
}
