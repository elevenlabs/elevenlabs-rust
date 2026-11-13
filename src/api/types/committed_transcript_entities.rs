pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommittedTranscriptEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<CommittedTranscriptEntitiesMessageType>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<EntitiesItem>>,
}

impl CommittedTranscriptEntities {
    pub fn builder() -> CommittedTranscriptEntitiesBuilder {
        <CommittedTranscriptEntitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommittedTranscriptEntitiesBuilder {
    message_type: Option<CommittedTranscriptEntitiesMessageType>,
    text: Option<String>,
    entities: Option<Vec<EntitiesItem>>,
}

impl CommittedTranscriptEntitiesBuilder {
    pub fn message_type(mut self, value: CommittedTranscriptEntitiesMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn entities(mut self, value: Vec<EntitiesItem>) -> Self {
        self.entities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommittedTranscriptEntities`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CommittedTranscriptEntitiesBuilder::text)
    pub fn build(self) -> Result<CommittedTranscriptEntities, BuildError> {
        Ok(CommittedTranscriptEntities {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            entities: self.entities,
        })
    }
}
