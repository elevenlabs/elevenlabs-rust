pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationPing {
    pub ping_event: PingEvent,
}

impl ConversationPing {
    pub fn builder() -> ConversationPingBuilder {
        <ConversationPingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationPingBuilder {
    ping_event: Option<PingEvent>,
}

impl ConversationPingBuilder {
    pub fn ping_event(mut self, value: PingEvent) -> Self {
        self.ping_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationPing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ping_event`](ConversationPingBuilder::ping_event)
    pub fn build(self) -> Result<ConversationPing, BuildError> {
        Ok(ConversationPing {
            ping_event: self
                .ping_event
                .ok_or_else(|| BuildError::missing_field("ping_event"))?,
        })
    }
}
