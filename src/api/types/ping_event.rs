pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PingEvent {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub ping_ms: Option<i64>,
}

impl PingEvent {
    pub fn builder() -> PingEventBuilder {
        <PingEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PingEventBuilder {
    event_id: Option<i64>,
    ping_ms: Option<i64>,
}

impl PingEventBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn ping_ms(mut self, value: i64) -> Self {
        self.ping_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PingEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](PingEventBuilder::event_id)
    pub fn build(self) -> Result<PingEvent, BuildError> {
        Ok(PingEvent {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            ping_ms: self.ping_ms,
        })
    }
}
