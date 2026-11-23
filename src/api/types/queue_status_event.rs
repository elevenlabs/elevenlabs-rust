pub use crate::prelude::*;

/// Data for concurrency wait-queue status events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueueStatusEvent {
    pub status: QueueStatusEventStatus,
}

impl QueueStatusEvent {
    pub fn builder() -> QueueStatusEventBuilder {
        <QueueStatusEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueueStatusEventBuilder {
    status: Option<QueueStatusEventStatus>,
}

impl QueueStatusEventBuilder {
    pub fn status(mut self, value: QueueStatusEventStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueueStatusEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueueStatusEventBuilder::status)
    pub fn build(self) -> Result<QueueStatusEvent, BuildError> {
        Ok(QueueStatusEvent {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
