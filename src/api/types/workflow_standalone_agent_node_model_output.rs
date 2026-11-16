pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStandaloneAgentNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    /// The ID of the agent to transfer the conversation to. None means transfer within the current agent.
    #[serde(default)]
    pub agent_id: Option<String>,
    /// Optional branch ID for the target agent. None follows the target agent's traffic deployment, or uses its main branch when node_id is set.
    #[serde(default)]
    pub branch_id: Option<String>,
    /// Optional target node ID in the destination agent's workflow. When set, the transfer starts at this node instead of the default entry node.
    #[serde(default)]
    pub node_id: Option<String>,
    /// Artificial delay in milliseconds applied before transferring the conversation.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub delay_ms: i64,
    /// Optional message sent to the user before the transfer is initiated.
    #[serde(default)]
    pub transfer_message: Option<String>,
    /// Whether to enable the transferred agent to send its configured first message after the transfer.
    #[serde(default)]
    pub enable_transferred_agent_first_message: bool,
    /// Defines whether TTS client overrides should be carried over to the transferred agent.
    #[serde(default)]
    pub preserve_client_tts_overrides: bool,
    /// Defines whether the base voice, TTS model and voice settings should be carried over to the transferred agent.
    #[serde(default)]
    pub preserve_voice_settings: bool,
}

impl WorkflowStandaloneAgentNodeModelOutput {
    pub fn builder() -> WorkflowStandaloneAgentNodeModelOutputBuilder {
        <WorkflowStandaloneAgentNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStandaloneAgentNodeModelOutputBuilder {
    position: Option<PositionOutput>,
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

impl WorkflowStandaloneAgentNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
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

    /// Consumes the builder and constructs a [`WorkflowStandaloneAgentNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowStandaloneAgentNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowStandaloneAgentNodeModelOutputBuilder::edge_order)
    /// - [`delay_ms`](WorkflowStandaloneAgentNodeModelOutputBuilder::delay_ms)
    /// - [`enable_transferred_agent_first_message`](WorkflowStandaloneAgentNodeModelOutputBuilder::enable_transferred_agent_first_message)
    /// - [`preserve_client_tts_overrides`](WorkflowStandaloneAgentNodeModelOutputBuilder::preserve_client_tts_overrides)
    /// - [`preserve_voice_settings`](WorkflowStandaloneAgentNodeModelOutputBuilder::preserve_voice_settings)
    pub fn build(self) -> Result<WorkflowStandaloneAgentNodeModelOutput, BuildError> {
        Ok(WorkflowStandaloneAgentNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            node_id: self.node_id,
            delay_ms: self
                .delay_ms
                .ok_or_else(|| BuildError::missing_field("delay_ms"))?,
            transfer_message: self.transfer_message,
            enable_transferred_agent_first_message: self
                .enable_transferred_agent_first_message
                .ok_or_else(|| {
                    BuildError::missing_field("enable_transferred_agent_first_message")
                })?,
            preserve_client_tts_overrides: self
                .preserve_client_tts_overrides
                .ok_or_else(|| BuildError::missing_field("preserve_client_tts_overrides"))?,
            preserve_voice_settings: self
                .preserve_voice_settings
                .ok_or_else(|| BuildError::missing_field("preserve_voice_settings"))?,
        })
    }
}
