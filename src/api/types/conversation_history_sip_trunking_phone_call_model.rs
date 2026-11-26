pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistorySipTrunkingPhoneCallModel {
    pub direction: TelephonyDirection,
    #[serde(default)]
    pub phone_number_id: String,
    #[serde(default)]
    pub agent_number: String,
    #[serde(default)]
    pub external_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_result: Option<CallResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(default)]
    pub call_sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_header_dynamic_variables: Option<std::collections::HashMap<String, String>>,
}

impl ConversationHistorySipTrunkingPhoneCallModel {
    pub fn builder() -> ConversationHistorySipTrunkingPhoneCallModelBuilder {
        <ConversationHistorySipTrunkingPhoneCallModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistorySipTrunkingPhoneCallModelBuilder {
    direction: Option<TelephonyDirection>,
    phone_number_id: Option<String>,
    agent_number: Option<String>,
    external_number: Option<String>,
    call_result: Option<CallResult>,
    call_id: Option<String>,
    call_sid: Option<String>,
    sip_header_dynamic_variables: Option<std::collections::HashMap<String, String>>,
}

impl ConversationHistorySipTrunkingPhoneCallModelBuilder {
    pub fn direction(mut self, value: TelephonyDirection) -> Self {
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

    pub fn call_id(mut self, value: impl Into<String>) -> Self {
        self.call_id = Some(value.into());
        self
    }

    pub fn call_sid(mut self, value: impl Into<String>) -> Self {
        self.call_sid = Some(value.into());
        self
    }

    pub fn sip_header_dynamic_variables(
        mut self,
        value: std::collections::HashMap<String, String>,
    ) -> Self {
        self.sip_header_dynamic_variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistorySipTrunkingPhoneCallModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](ConversationHistorySipTrunkingPhoneCallModelBuilder::direction)
    /// - [`phone_number_id`](ConversationHistorySipTrunkingPhoneCallModelBuilder::phone_number_id)
    /// - [`agent_number`](ConversationHistorySipTrunkingPhoneCallModelBuilder::agent_number)
    /// - [`external_number`](ConversationHistorySipTrunkingPhoneCallModelBuilder::external_number)
    /// - [`call_sid`](ConversationHistorySipTrunkingPhoneCallModelBuilder::call_sid)
    pub fn build(self) -> Result<ConversationHistorySipTrunkingPhoneCallModel, BuildError> {
        Ok(ConversationHistorySipTrunkingPhoneCallModel {
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
            call_id: self.call_id,
            call_sid: self
                .call_sid
                .ok_or_else(|| BuildError::missing_field("call_sid"))?,
            sip_header_dynamic_variables: self.sip_header_dynamic_variables,
        })
    }
}
