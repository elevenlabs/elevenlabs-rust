pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamEndedEvent {
    pub stream_ended_event: StreamEndedData,
}

impl StreamEndedEvent {
    pub fn builder() -> StreamEndedEventBuilder {
        <StreamEndedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEndedEventBuilder {
    stream_ended_event: Option<StreamEndedData>,
}

impl StreamEndedEventBuilder {
    pub fn stream_ended_event(mut self, value: StreamEndedData) -> Self {
        self.stream_ended_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamEndedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stream_ended_event`](StreamEndedEventBuilder::stream_ended_event)
    pub fn build(self) -> Result<StreamEndedEvent, BuildError> {
        Ok(StreamEndedEvent {
            stream_ended_event: self
                .stream_ended_event
                .ok_or_else(|| BuildError::missing_field("stream_ended_event"))?,
        })
    }
}
