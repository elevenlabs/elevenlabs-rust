pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationHistoryTwilioPhoneCallModel {
    pub direction: SmsConversationInfoDirection,
    #[serde(default)]
    pub phone_number_id: String,
    #[serde(default)]
    pub agent_number: String,
    #[serde(default)]
    pub external_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_result: Option<CallResult>,
    #[serde(default)]
    pub stream_sid: String,
    #[serde(default)]
    pub call_sid: String,
}

impl ConversationHistoryTwilioPhoneCallModel {
    pub fn builder() -> ConversationHistoryTwilioPhoneCallModelBuilder {
        <ConversationHistoryTwilioPhoneCallModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTwilioPhoneCallModelBuilder {
    direction: Option<SmsConversationInfoDirection>,
    phone_number_id: Option<String>,
    agent_number: Option<String>,
    external_number: Option<String>,
    call_result: Option<CallResult>,
    stream_sid: Option<String>,
    call_sid: Option<String>,
}

impl ConversationHistoryTwilioPhoneCallModelBuilder {
    pub fn direction(mut self, value: SmsConversationInfoDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn agent_number(mut self, value: impl Into<String>) -> Self {
        self.agent_number = Some(value.into());
        self
    }

    pub fn external_number(mut self, value: impl Into<String>) -> Self {
        self.external_number = Some(value.into());
        self
    }

    pub fn call_result(mut self, value: CallResult) -> Self {
        self.call_result = Some(value);
        self
    }

    pub fn stream_sid(mut self, value: impl Into<String>) -> Self {
        self.stream_sid = Some(value.into());
        self
    }

    pub fn call_sid(mut self, value: impl Into<String>) -> Self {
        self.call_sid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTwilioPhoneCallModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](ConversationHistoryTwilioPhoneCallModelBuilder::direction)
    /// - [`phone_number_id`](ConversationHistoryTwilioPhoneCallModelBuilder::phone_number_id)
    /// - [`agent_number`](ConversationHistoryTwilioPhoneCallModelBuilder::agent_number)
    /// - [`external_number`](ConversationHistoryTwilioPhoneCallModelBuilder::external_number)
    /// - [`stream_sid`](ConversationHistoryTwilioPhoneCallModelBuilder::stream_sid)
    /// - [`call_sid`](ConversationHistoryTwilioPhoneCallModelBuilder::call_sid)
    pub fn build(self) -> Result<ConversationHistoryTwilioPhoneCallModel, BuildError> {
        Ok(ConversationHistoryTwilioPhoneCallModel {
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            phone_number_id: self
                .phone_number_id
                .ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
            agent_number: self
                .agent_number
                .ok_or_else(|| BuildError::missing_field("agent_number"))?,
            external_number: self
                .external_number
                .ok_or_else(|| BuildError::missing_field("external_number"))?,
            call_result: self.call_result,
            stream_sid: self
                .stream_sid
                .ok_or_else(|| BuildError::missing_field("stream_sid"))?,
            call_sid: self
                .call_sid
                .ok_or_else(|| BuildError::missing_field("call_sid"))?,
        })
    }
}
