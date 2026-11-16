pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "result_type")]
#[non_exhaustive]
pub enum ConversationHistoryTranscriptSystemToolResultCommonModelInputResult {
    #[serde(rename = "dummy")]
    Dummy {
        #[serde(flatten)]
        data: DummyToolResultModel,
    },

    #[serde(rename = "end_call_success")]
    EndCallSuccess {
        #[serde(flatten)]
        data: EndCallToolResultModel,
    },

    #[serde(rename = "end_procedure_error")]
    EndProcedureError {
        #[serde(flatten)]
        data: EndProcedureToolResultErrorModel,
    },

    #[serde(rename = "end_procedure_success")]
    EndProcedureSuccess {
        #[serde(flatten)]
        data: EndProcedureToolResultSuccessModel,
    },

    #[serde(rename = "knowledge_base_rag_success")]
    KnowledgeBaseRagSuccess {
        #[serde(flatten)]
        data: KnowledgeBaseRagToolResultModel,
    },

    #[serde(rename = "knowledge_base_success")]
    KnowledgeBaseSuccess {
        #[serde(flatten)]
        data: KnowledgeBaseToolResultModel,
    },

    #[serde(rename = "language_detection_success")]
    LanguageDetectionSuccess {
        #[serde(flatten)]
        data: LanguageDetectionToolResultModel,
    },

    #[serde(rename = "play_dtmf_error")]
    PlayDtmfError {
        #[serde(flatten)]
        data: PlayDtmfResultErrorModel,
    },

    #[serde(rename = "play_dtmf_success")]
    PlayDtmfSuccess {
        #[serde(flatten)]
        data: PlayDtmfResultSuccessModel,
    },

    #[serde(rename = "skip_turn_success")]
    SkipTurnSuccess {
        #[serde(flatten)]
        data: SkipTurnToolResponseModel,
    },

    #[serde(rename = "start_procedure_error")]
    StartProcedureError {
        #[serde(flatten)]
        data: StartProcedureToolResultErrorModel,
    },

    #[serde(rename = "start_procedure_success")]
    StartProcedureSuccess {
        #[serde(flatten)]
        data: StartProcedureToolResultSuccessModel,
    },

    #[serde(rename = "testing_tool_result")]
    TestingToolResult {
        #[serde(flatten)]
        data: TestToolResultModel,
    },

    #[serde(rename = "transfer_to_agent_error")]
    TransferToAgentError {
        #[serde(flatten)]
        data: TransferToAgentToolResultErrorModel,
    },

    #[serde(rename = "transfer_to_agent_success")]
    TransferToAgentSuccess {
        #[serde(flatten)]
        data: TransferToAgentToolResultSuccessModelInput,
    },

    #[serde(rename = "transfer_to_number_amazon_connect_success")]
    TransferToNumberAmazonConnectSuccess {
        #[serde(flatten)]
        data: TransferToNumberResultAmazonConnectSuccessModel,
    },

    #[serde(rename = "transfer_to_number_error")]
    TransferToNumberError {
        #[serde(flatten)]
        data: TransferToNumberResultErrorModel,
    },

    #[serde(rename = "transfer_to_number_exotel_success")]
    TransferToNumberExotelSuccess {
        #[serde(flatten)]
        data: TransferToNumberResultExotelSuccessModel,
    },

    #[serde(rename = "transfer_to_number_sip_success")]
    TransferToNumberSipSuccess {
        #[serde(flatten)]
        data: TransferToNumberResultSipSuccessModel,
    },

    #[serde(rename = "transfer_to_number_twilio_success")]
    TransferToNumberTwilioSuccess {
        #[serde(flatten)]
        data: TransferToNumberResultTwilioSuccessModel,
    },

    #[serde(rename = "voicemail_detection_success")]
    VoicemailDetectionSuccess {
        #[serde(flatten)]
        data: VoiceMailDetectionResultSuccessModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConversationHistoryTranscriptSystemToolResultCommonModelInputResult {
    pub fn is_dummy(&self) -> bool {
        matches!(self, Self::Dummy { .. })
    }

    pub fn as_dummy(&self) -> Option<&DummyToolResultModel> {
        match self {
            Self::Dummy { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_dummy(self) -> Option<DummyToolResultModel> {
        match self {
            Self::Dummy { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_end_call_success(&self) -> bool {
        matches!(self, Self::EndCallSuccess { .. })
    }

    pub fn as_end_call_success(&self) -> Option<&EndCallToolResultModel> {
        match self {
            Self::EndCallSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_call_success(self) -> Option<EndCallToolResultModel> {
        match self {
            Self::EndCallSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_end_procedure_error(&self) -> bool {
        matches!(self, Self::EndProcedureError { .. })
    }

    pub fn as_end_procedure_error(&self) -> Option<&EndProcedureToolResultErrorModel> {
        match self {
            Self::EndProcedureError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_procedure_error(self) -> Option<EndProcedureToolResultErrorModel> {
        match self {
            Self::EndProcedureError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_end_procedure_success(&self) -> bool {
        matches!(self, Self::EndProcedureSuccess { .. })
    }

    pub fn as_end_procedure_success(&self) -> Option<&EndProcedureToolResultSuccessModel> {
        match self {
            Self::EndProcedureSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_procedure_success(self) -> Option<EndProcedureToolResultSuccessModel> {
        match self {
            Self::EndProcedureSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_knowledge_base_rag_success(&self) -> bool {
        matches!(self, Self::KnowledgeBaseRagSuccess { .. })
    }

    pub fn as_knowledge_base_rag_success(&self) -> Option<&KnowledgeBaseRagToolResultModel> {
        match self {
            Self::KnowledgeBaseRagSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_knowledge_base_rag_success(self) -> Option<KnowledgeBaseRagToolResultModel> {
        match self {
            Self::KnowledgeBaseRagSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_knowledge_base_success(&self) -> bool {
        matches!(self, Self::KnowledgeBaseSuccess { .. })
    }

    pub fn as_knowledge_base_success(&self) -> Option<&KnowledgeBaseToolResultModel> {
        match self {
            Self::KnowledgeBaseSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_knowledge_base_success(self) -> Option<KnowledgeBaseToolResultModel> {
        match self {
            Self::KnowledgeBaseSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_language_detection_success(&self) -> bool {
        matches!(self, Self::LanguageDetectionSuccess { .. })
    }

    pub fn as_language_detection_success(&self) -> Option<&LanguageDetectionToolResultModel> {
        match self {
            Self::LanguageDetectionSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_language_detection_success(self) -> Option<LanguageDetectionToolResultModel> {
        match self {
            Self::LanguageDetectionSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_play_dtmf_error(&self) -> bool {
        matches!(self, Self::PlayDtmfError { .. })
    }

    pub fn as_play_dtmf_error(&self) -> Option<&PlayDtmfResultErrorModel> {
        match self {
            Self::PlayDtmfError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_play_dtmf_error(self) -> Option<PlayDtmfResultErrorModel> {
        match self {
            Self::PlayDtmfError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_play_dtmf_success(&self) -> bool {
        matches!(self, Self::PlayDtmfSuccess { .. })
    }

    pub fn as_play_dtmf_success(&self) -> Option<&PlayDtmfResultSuccessModel> {
        match self {
            Self::PlayDtmfSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_play_dtmf_success(self) -> Option<PlayDtmfResultSuccessModel> {
        match self {
            Self::PlayDtmfSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_skip_turn_success(&self) -> bool {
        matches!(self, Self::SkipTurnSuccess { .. })
    }

    pub fn as_skip_turn_success(&self) -> Option<&SkipTurnToolResponseModel> {
        match self {
            Self::SkipTurnSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_skip_turn_success(self) -> Option<SkipTurnToolResponseModel> {
        match self {
            Self::SkipTurnSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start_procedure_error(&self) -> bool {
        matches!(self, Self::StartProcedureError { .. })
    }

    pub fn as_start_procedure_error(&self) -> Option<&StartProcedureToolResultErrorModel> {
        match self {
            Self::StartProcedureError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start_procedure_error(self) -> Option<StartProcedureToolResultErrorModel> {
        match self {
            Self::StartProcedureError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start_procedure_success(&self) -> bool {
        matches!(self, Self::StartProcedureSuccess { .. })
    }

    pub fn as_start_procedure_success(&self) -> Option<&StartProcedureToolResultSuccessModel> {
        match self {
            Self::StartProcedureSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start_procedure_success(self) -> Option<StartProcedureToolResultSuccessModel> {
        match self {
            Self::StartProcedureSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_testing_tool_result(&self) -> bool {
        matches!(self, Self::TestingToolResult { .. })
    }

    pub fn as_testing_tool_result(&self) -> Option<&TestToolResultModel> {
        match self {
            Self::TestingToolResult { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_testing_tool_result(self) -> Option<TestToolResultModel> {
        match self {
            Self::TestingToolResult { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_agent_error(&self) -> bool {
        matches!(self, Self::TransferToAgentError { .. })
    }

    pub fn as_transfer_to_agent_error(&self) -> Option<&TransferToAgentToolResultErrorModel> {
        match self {
            Self::TransferToAgentError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_agent_error(self) -> Option<TransferToAgentToolResultErrorModel> {
        match self {
            Self::TransferToAgentError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_agent_success(&self) -> bool {
        matches!(self, Self::TransferToAgentSuccess { .. })
    }

    pub fn as_transfer_to_agent_success(
        &self,
    ) -> Option<&TransferToAgentToolResultSuccessModelInput> {
        match self {
            Self::TransferToAgentSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_agent_success(
        self,
    ) -> Option<TransferToAgentToolResultSuccessModelInput> {
        match self {
            Self::TransferToAgentSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_number_amazon_connect_success(&self) -> bool {
        matches!(self, Self::TransferToNumberAmazonConnectSuccess { .. })
    }

    pub fn as_transfer_to_number_amazon_connect_success(
        &self,
    ) -> Option<&TransferToNumberResultAmazonConnectSuccessModel> {
        match self {
            Self::TransferToNumberAmazonConnectSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_number_amazon_connect_success(
        self,
    ) -> Option<TransferToNumberResultAmazonConnectSuccessModel> {
        match self {
            Self::TransferToNumberAmazonConnectSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_number_error(&self) -> bool {
        matches!(self, Self::TransferToNumberError { .. })
    }

    pub fn as_transfer_to_number_error(&self) -> Option<&TransferToNumberResultErrorModel> {
        match self {
            Self::TransferToNumberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_number_error(self) -> Option<TransferToNumberResultErrorModel> {
        match self {
            Self::TransferToNumberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_number_exotel_success(&self) -> bool {
        matches!(self, Self::TransferToNumberExotelSuccess { .. })
    }

    pub fn as_transfer_to_number_exotel_success(
        &self,
    ) -> Option<&TransferToNumberResultExotelSuccessModel> {
        match self {
            Self::TransferToNumberExotelSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_number_exotel_success(
        self,
    ) -> Option<TransferToNumberResultExotelSuccessModel> {
        match self {
            Self::TransferToNumberExotelSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_number_sip_success(&self) -> bool {
        matches!(self, Self::TransferToNumberSipSuccess { .. })
    }

    pub fn as_transfer_to_number_sip_success(
        &self,
    ) -> Option<&TransferToNumberResultSipSuccessModel> {
        match self {
            Self::TransferToNumberSipSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_number_sip_success(
        self,
    ) -> Option<TransferToNumberResultSipSuccessModel> {
        match self {
            Self::TransferToNumberSipSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transfer_to_number_twilio_success(&self) -> bool {
        matches!(self, Self::TransferToNumberTwilioSuccess { .. })
    }

    pub fn as_transfer_to_number_twilio_success(
        &self,
    ) -> Option<&TransferToNumberResultTwilioSuccessModel> {
        match self {
            Self::TransferToNumberTwilioSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transfer_to_number_twilio_success(
        self,
    ) -> Option<TransferToNumberResultTwilioSuccessModel> {
        match self {
            Self::TransferToNumberTwilioSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_voicemail_detection_success(&self) -> bool {
        matches!(self, Self::VoicemailDetectionSuccess { .. })
    }

    pub fn as_voicemail_detection_success(&self) -> Option<&VoiceMailDetectionResultSuccessModel> {
        match self {
            Self::VoicemailDetectionSuccess { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_voicemail_detection_success(self) -> Option<VoiceMailDetectionResultSuccessModel> {
        match self {
            Self::VoicemailDetectionSuccess { data } => Some(data),
            _ => None,
        }
    }
}
