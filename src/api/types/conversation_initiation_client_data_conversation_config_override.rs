pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationInitiationClientDataConversationConfigOverride {
    /// Configuration for conversational transcription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<Asr>,
    /// Configuration for turn detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<Turn>,
    /// Configuration for conversational text to speech
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<Tts>,
    /// Configuration for conversational events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Conversation>,
    /// Agent specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
}

impl ConversationInitiationClientDataConversationConfigOverride {
    pub fn builder() -> ConversationInitiationClientDataConversationConfigOverrideBuilder {
        <ConversationInitiationClientDataConversationConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationClientDataConversationConfigOverrideBuilder {
    asr: Option<Asr>,
    turn: Option<Turn>,
    tts: Option<Tts>,
    conversation: Option<Conversation>,
    agent: Option<Agent>,
}

impl ConversationInitiationClientDataConversationConfigOverrideBuilder {
    pub fn asr(mut self, value: Asr) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn turn(mut self, value: Turn) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn tts(mut self, value: Tts) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn conversation(mut self, value: Conversation) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn agent(mut self, value: Agent) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationClientDataConversationConfigOverride`].
    pub fn build(
        self,
    ) -> Result<ConversationInitiationClientDataConversationConfigOverride, BuildError> {
        Ok(ConversationInitiationClientDataConversationConfigOverride {
            asr: self.asr,
            turn: self.turn,
            tts: self.tts,
            conversation: self.conversation,
            agent: self.agent,
        })
    }
}
