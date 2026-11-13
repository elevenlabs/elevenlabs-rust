pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowStandaloneAgentNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    /// The ID of the agent to transfer the conversation to. None means transfer within the current agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Optional branch ID for the target agent. None follows the target agent's traffic deployment, or uses its main branch when node_id is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Optional target node ID in the destination agent's workflow. When set, the transfer starts at this node instead of the default entry node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Artificial delay in milliseconds applied before transferring the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub delay_ms: Option<i64>,
    /// Optional message sent to the user before the transfer is initiated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
    /// Whether to enable the transferred agent to send its configured first message after the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_transferred_agent_first_message: Option<bool>,
    /// Defines whether TTS client overrides should be carried over to the transferred agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_client_tts_overrides: Option<bool>,
    /// Defines whether the base voice, TTS model and voice settings should be carried over to the transferred agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_voice_settings: Option<bool>,
}

impl WorkflowStandaloneAgentNodeModelInput {
    pub fn builder() -> WorkflowStandaloneAgentNodeModelInputBuilder {
        <WorkflowStandaloneAgentNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStandaloneAgentNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    node_id: Option<String>,
    delay_ms: Option<i64>,
    transfer_message: Option<String>,
    enable_transferred_agent_first_message: Option<bool>,
    preserve_client_tts_overrides: Option<bool>,
    preserve_voice_settings: Option<bool>,
}

impl WorkflowStandaloneAgentNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn node_id(mut self, value: impl Into<String>) -> Self {
        self.node_id = Some(value.into());
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

    pub fn preserve_client_tts_overrides(mut self, value: bool) -> Self {
        self.preserve_client_tts_overrides = Some(value);
        self
    }

    pub fn preserve_voice_settings(mut self, value: bool) -> Self {
        self.preserve_voice_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowStandaloneAgentNodeModelInput`].
    pub fn build(self) -> Result<WorkflowStandaloneAgentNodeModelInput, BuildError> {
        Ok(WorkflowStandaloneAgentNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            node_id: self.node_id,
            delay_ms: self.delay_ms,
            transfer_message: self.transfer_message,
            enable_transferred_agent_first_message: self.enable_transferred_agent_first_message,
            preserve_client_tts_overrides: self.preserve_client_tts_overrides,
            preserve_voice_settings: self.preserve_voice_settings,
        })
    }
}
