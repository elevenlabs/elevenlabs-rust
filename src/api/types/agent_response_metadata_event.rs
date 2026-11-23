pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentResponseMetadataEvent {
    #[serde(default)]
    pub metadata: serde_json::Value,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
}

impl AgentResponseMetadataEvent {
    pub fn builder() -> AgentResponseMetadataEventBuilder {
        <AgentResponseMetadataEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseMetadataEventBuilder {
    metadata: Option<serde_json::Value>,
    event_id: Option<i64>,
}

impl AgentResponseMetadataEventBuilder {
    pub fn metadata(mut self, value: serde_json::Value) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseMetadataEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metadata`](AgentResponseMetadataEventBuilder::metadata)
    /// - [`event_id`](AgentResponseMetadataEventBuilder::event_id)
    pub fn build(self) -> Result<AgentResponseMetadataEvent, BuildError> {
        Ok(AgentResponseMetadataEvent {
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}
