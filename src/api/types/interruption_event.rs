pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InterruptionEvent {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
}

impl InterruptionEvent {
    pub fn builder() -> InterruptionEventBuilder {
        <InterruptionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InterruptionEventBuilder {
    event_id: Option<i64>,
}

impl InterruptionEventBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InterruptionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](InterruptionEventBuilder::event_id)
    pub fn build(self) -> Result<InterruptionEvent, BuildError> {
        Ok(InterruptionEvent {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}
