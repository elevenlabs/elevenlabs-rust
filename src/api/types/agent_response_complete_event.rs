pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponseCompleteEvent {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
}

impl AgentResponseCompleteEvent {
    pub fn builder() -> AgentResponseCompleteEventBuilder {
        <AgentResponseCompleteEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponseCompleteEventBuilder {
    event_id: Option<i64>,
}

impl AgentResponseCompleteEventBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponseCompleteEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](AgentResponseCompleteEventBuilder::event_id)
    pub fn build(self) -> Result<AgentResponseCompleteEvent, BuildError> {
        Ok(AgentResponseCompleteEvent {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}
