pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationPong {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConversationPongType>,
    /// ID of the ping event this pong responds to.
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
}

impl ConversationPong {
    pub fn builder() -> ConversationPongBuilder {
        <ConversationPongBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationPongBuilder {
    r#type: Option<ConversationPongType>,
    event_id: Option<i64>,
}

impl ConversationPongBuilder {
    pub fn r#type(mut self, value: ConversationPongType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationPong`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ConversationPongBuilder::event_id)
    pub fn build(self) -> Result<ConversationPong, BuildError> {
        Ok(ConversationPong {
            r#type: self.r#type,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}
