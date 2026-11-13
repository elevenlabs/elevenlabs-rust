pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodcastConversationMode {
    pub conversation: PodcastConversationModeData,
}

impl PodcastConversationMode {
    pub fn builder() -> PodcastConversationModeBuilder {
        <PodcastConversationModeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastConversationModeBuilder {
    conversation: Option<PodcastConversationModeData>,
}

impl PodcastConversationModeBuilder {
    pub fn conversation(mut self, value: PodcastConversationModeData) -> Self {
        self.conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodcastConversationMode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation`](PodcastConversationModeBuilder::conversation)
    pub fn build(self) -> Result<PodcastConversationMode, BuildError> {
        Ok(PodcastConversationMode {
            conversation: self
                .conversation
                .ok_or_else(|| BuildError::missing_field("conversation"))?,
        })
    }
}
