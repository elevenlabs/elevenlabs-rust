pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferToAgentToolResultSuccessModelOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentSuccessfulResponseExampleType>,
    #[serde(default)]
    pub from_agent: String,
    #[serde(default)]
    pub to_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_node: Option<String>,
    #[serde(default)]
    pub condition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub delay_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_transferred_agent_first_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_client_tts_overrides: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_voice_settings: Option<bool>,
}

impl TransferToAgentToolResultSuccessModelOutput {
    pub fn builder() -> TransferToAgentToolResultSuccessModelOutputBuilder {
        <TransferToAgentToolResultSuccessModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToAgentToolResultSuccessModelOutputBuilder {
    status: Option<AgentSuccessfulResponseExampleType>,
    from_agent: Option<String>,
    to_agent: Option<String>,
    to_node: Option<String>,
    condition: Option<String>,
    delay_ms: Option<i64>,
    transfer_message: Option<String>,
    enable_transferred_agent_first_message: Option<bool>,
    branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>,
    preserve_client_tts_overrides: Option<bool>,
    preserve_voice_settings: Option<bool>,
}

impl TransferToAgentToolResultSuccessModelOutputBuilder {
    pub fn status(mut self, value: AgentSuccessfulResponseExampleType) -> Self {
        self.status = Some(value);
        self
    }

    pub fn from_agent(mut self, value: impl Into<String>) -> Self {
        self.from_agent = Some(value.into());
        self
    }

    pub fn to_agent(mut self, value: impl Into<String>) -> Self {
        self.to_agent = Some(value.into());
        self
    }

    pub fn to_node(mut self, value: impl Into<String>) -> Self {
        self.to_node = Some(value.into());
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    pub fn delay_ms(mut self, value: i64) -> Self {
        self.delay_ms = Some(value);
        self
    }

    pub fn transfer_message(mut self, value: impl Into<String>) -> Self {
        self.transfer_message = Some(value.into());
        self
    }

    pub fn enable_transferred_agent_first_message(mut self, value: bool) -> Self {
        self.enable_transferred_agent_first_message = Some(value);
        self
    }

    pub fn branch_info(
        mut self,
        value: TransferToAgentToolResultSuccessModelOutputBranchInfo,
    ) -> Self {
        self.branch_info = Some(value);
        self
    }

    pub fn preserve_client_tts_overrides(mut self, value: bool) -> Self {
        self.preserve_client_tts_overrides = Some(value);
        self
    }

    pub fn preserve_voice_settings(mut self, value: bool) -> Self {
        self.preserve_voice_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferToAgentToolResultSuccessModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_agent`](TransferToAgentToolResultSuccessModelOutputBuilder::from_agent)
    /// - [`to_agent`](TransferToAgentToolResultSuccessModelOutputBuilder::to_agent)
    /// - [`condition`](TransferToAgentToolResultSuccessModelOutputBuilder::condition)
    pub fn build(self) -> Result<TransferToAgentToolResultSuccessModelOutput, BuildError> {
        Ok(TransferToAgentToolResultSuccessModelOutput {
            status: self.status,
            from_agent: self
                .from_agent
                .ok_or_else(|| BuildError::missing_field("from_agent"))?,
            to_agent: self
                .to_agent
                .ok_or_else(|| BuildError::missing_field("to_agent"))?,
            to_node: self.to_node,
            condition: self
                .condition
                .ok_or_else(|| BuildError::missing_field("condition"))?,
            delay_ms: self.delay_ms,
            transfer_message: self.transfer_message,
            enable_transferred_agent_first_message: self.enable_transferred_agent_first_message,
            branch_info: self.branch_info,
            preserve_client_tts_overrides: self.preserve_client_tts_overrides,
            preserve_voice_settings: self.preserve_voice_settings,
        })
    }
}
