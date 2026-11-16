pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceMailDetectionResultSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentSuccessfulResponseExampleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voicemail_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl VoiceMailDetectionResultSuccessModel {
    pub fn builder() -> VoiceMailDetectionResultSuccessModelBuilder {
        <VoiceMailDetectionResultSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceMailDetectionResultSuccessModelBuilder {
    status: Option<AgentSuccessfulResponseExampleType>,
    voicemail_message: Option<String>,
    reason: Option<String>,
}

impl VoiceMailDetectionResultSuccessModelBuilder {
    pub fn status(mut self, value: AgentSuccessfulResponseExampleType) -> Self {
        self.status = Some(value);
        self
    }

    pub fn voicemail_message(mut self, value: impl Into<String>) -> Self {
        self.voicemail_message = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceMailDetectionResultSuccessModel`].
    pub fn build(self) -> Result<VoiceMailDetectionResultSuccessModel, BuildError> {
        Ok(VoiceMailDetectionResultSuccessModel {
            status: self.status,
            voicemail_message: self.voicemail_message,
            reason: self.reason,
        })
    }
}
