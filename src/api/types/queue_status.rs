pub use crate::prelude::*;

/// Status of a caller held in the concurrency wait queue. Sent once when the wait
/// starts (`waiting`) and once when it ends: `admitted` when the caller is connected
/// to the agent, or `timed_out` right before the connection is closed with code 4300.
/// Hold audio arrives as regular `audio` events while the caller waits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueueStatus {
    /// Data for concurrency wait-queue status events.
    pub queue_status_event: QueueStatusEvent,
}

impl QueueStatus {
    pub fn builder() -> QueueStatusBuilder {
        <QueueStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueueStatusBuilder {
    queue_status_event: Option<QueueStatusEvent>,
}

impl QueueStatusBuilder {
    pub fn queue_status_event(mut self, value: QueueStatusEvent) -> Self {
        self.queue_status_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueueStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`queue_status_event`](QueueStatusBuilder::queue_status_event)
    pub fn build(self) -> Result<QueueStatus, BuildError> {
        Ok(QueueStatus {
            queue_status_event: self
                .queue_status_event
                .ok_or_else(|| BuildError::missing_field("queue_status_event"))?,
        })
    }
}
