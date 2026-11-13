pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "system_tool_type")]
#[non_exhaustive]
pub enum SystemToolConfigInputParams {
    #[serde(rename = "end_call")]
    EndCall {
        #[serde(flatten)]
        data: EndCallToolConfig,
    },

    #[serde(rename = "end_procedure")]
    EndProcedure {
        #[serde(flatten)]
        data: EndProcedureToolConfig,
    },

    #[serde(rename = "knowledge_base")]
    KnowledgeBase {
        #[serde(flatten)]
        data: KnowledgeBaseToolConfig,
    },

    #[serde(rename = "knowledge_base_rag")]
    KnowledgeBaseRag {
        #[serde(flatten)]
        data: KnowledgeBaseRagToolConfig,
    },

    #[serde(rename = "language_detection")]
    LanguageDetection {
        #[serde(flatten)]
        data: LanguageDetectionToolConfig,
    },

    #[serde(rename = "play_keypad_touch_tone")]
    PlayKeypadTouchTone {
        #[serde(flatten)]
        data: PlayDtmfToolConfig,
    },

    #[serde(rename = "skip_turn")]
    SkipTurn {
        #[serde(flatten)]
        data: SkipTurnToolConfig,
    },

    #[serde(rename = "start_procedure")]
    StartProcedure {
        #[serde(flatten)]
        data: StartProcedureToolConfig,
    },

    #[serde(rename = "transfer_to_agent")]
    TransferToAgent {
        #[serde(flatten)]
        data: TransferToAgentToolConfigInput,
    },

    #[serde(rename = "transfer_to_number")]
    TransferToNumber {
        #[serde(flatten)]
        data: TransferToNumberToolConfigInput,
    },

    #[serde(rename = "voicemail_detection")]
    VoicemailDetection {
        #[serde(flatten)]
        data: VoicemailDetectionToolConfig,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SystemToolConfigInputParams {
    pub fn is_end_call(&self) -> bool {
        matches!(self, Self::EndCall { .. })
    }

    pub fn as_end_call(&self) -> Option<&EndCallToolConfig> {
        match self {
            Self::EndCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_call(self) -> Option<EndCallToolConfig> {
        match self {
            Self::EndCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_end_procedure(&self) -> bool {
        matches!(self, Self::EndProcedure { .. })
    }

    pub fn as_end_procedure(&self) -> Option<&EndProcedureToolConfig> {
        match self {
            Self::EndProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_procedure(self) -> Option<EndProcedureToolConfig> {
        match self {
            Self::EndProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_knowledge_base(&self) -> bool {
        matches!(self, Self::KnowledgeBase { .. })
    }

    pub fn as_knowledge_base(&self) -> Option<&KnowledgeBaseToolConfig> {
        match self {
            Self::KnowledgeBase { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_knowledge_base(self) -> Option<KnowledgeBaseToolConfig> {
        match self {
            Self::KnowledgeBase { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_knowledge_base_rag(&self) -> bool {
        matches!(self, Self::KnowledgeBaseRag { .. })
    }

    pub fn as_knowledge_base_rag(&self) -> Option<&KnowledgeBaseRagToolConfig> {
        match self {
            Self::KnowledgeBaseRag { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_knowledge_base_rag(self) -> Option<KnowledgeBaseRagToolConfig> {
        match self {
            Self::KnowledgeBaseRag { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_language_detection(&self) -> bool {
        matches!(self, Self::LanguageDetection { .. })
    }

    pub fn as_language_detection(&self) -> Option<&LanguageDetectionToolConfig> {
        match self {
            Self::LanguageDetection { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_language_detection(self) -> Option<LanguageDetectionToolConfig> {
        match self {
            Self::LanguageDetection { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_play_keypad_touch_tone(&self) -> bool {
        matches!(self, Self::PlayKeypadTouchTone { .. })
    }

    pub fn as_play_keypad_touch_tone(&self) -> Option<&PlayDtmfToolConfig> {
        match self {
            Self::PlayKeypadTouchTone { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_play_keypad_touch_tone(self) -> Option<PlayDtmfToolConfig> {
        match self {
            Self::PlayKeypadTouchTone { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_skip_turn(&self) -> bool {
        matches!(self, Self::SkipTurn { .. })
    }

    pub fn as_skip_turn(&self) -> Option<&SkipTurnToolConfig> {
        match self {
            Self::SkipTurn { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_skip_turn(self) -> Option<SkipTurnToolConfig> {
        match self {
            Self::SkipTurn { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start_procedure(&self) -> bool {
        matches!(self, Self::StartProcedure { .. })
    }

    pub fn as_start_procedure(&self) -> Option<&StartProcedureToolConfig> {
        match self {
            Self::StartProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start_procedure(self) -> Option<StartProcedureToolConfig> {
        match self {
            Self::StartProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_agent(&self) -> bool {
        matches!(self, Self::TransferToAgent { .. })
    }

    pub fn as_transfer_to_agent(&self) -> Option<&TransferToAgentToolConfigInput> {
        match self {
            Self::TransferToAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_agent(self) -> Option<TransferToAgentToolConfigInput> {
        match self {
            Self::TransferToAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_number(&self) -> bool {
        matches!(self, Self::TransferToNumber { .. })
    }

    pub fn as_transfer_to_number(&self) -> Option<&TransferToNumberToolConfigInput> {
        match self {
            Self::TransferToNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_number(self) -> Option<TransferToNumberToolConfigInput> {
        match self {
            Self::TransferToNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_voicemail_detection(&self) -> bool {
        matches!(self, Self::VoicemailDetection { .. })
    }

    pub fn as_voicemail_detection(&self) -> Option<&VoicemailDetectionToolConfig> {
        match self {
            Self::VoicemailDetection { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_voicemail_detection(self) -> Option<VoicemailDetectionToolConfig> {
        match self {
            Self::VoicemailDetection { data } => Some(data),
            _ => None,
        }
    }
}
