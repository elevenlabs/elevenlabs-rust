pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationalConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<AsrConversationalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<TurnConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConversationalConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationConfigInput>,
    /// Language presets for conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_presets: Option<std::collections::HashMap<String, LanguagePresetInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<VadConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfig>,
}

impl ConversationalConfig {
    pub fn builder() -> ConversationalConfigBuilder {
        <ConversationalConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalConfigBuilder {
    asr: Option<AsrConversationalConfig>,
    turn: Option<TurnConfig>,
    tts: Option<TtsConversationalConfigInput>,
    conversation: Option<ConversationConfigInput>,
    language_presets: Option<std::collections::HashMap<String, LanguagePresetInput>>,
    vad: Option<VadConfig>,
    agent: Option<AgentConfig>,
}

impl ConversationalConfigBuilder {
    pub fn asr(mut self, value: AsrConversationalConfig) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn turn(mut self, value: TurnConfig) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn tts(mut self, value: TtsConversationalConfigInput) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigInput) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn language_presets(
        mut self,
        value: std::collections::HashMap<String, LanguagePresetInput>,
    ) -> Self {
        self.language_presets = Some(value);
        self
    }

    pub fn vad(mut self, value: VadConfig) -> Self {
        self.vad = Some(value);
        self
    }

    pub fn agent(mut self, value: AgentConfig) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalConfig`].
    pub fn build(self) -> Result<ConversationalConfig, BuildError> {
        Ok(ConversationalConfig {
            asr: self.asr,
            turn: self.turn,
            tts: self.tts,
            conversation: self.conversation,
            language_presets: self.language_presets,
            vad: self.vad,
            agent: self.agent,
        })
    }
}
