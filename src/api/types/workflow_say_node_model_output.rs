pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowSayNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    #[serde(default)]
    pub conversation_config: ConversationalConfigApiModelWorkflowOverrideOutput,
    /// Accepted for consistency with subagent nodes but not used. Describe the message in `message.prompt`, or replace the system prompt with `conversation_config.agent.prompt.prompt`.
    #[serde(default)]
    pub additional_prompt: String,
    /// Knowledge base documents available to the LLM while this node generates its message, in addition to the agent's documents.
    #[serde(default)]
    pub additional_knowledge_base: Vec<KnowledgeBaseLocator>,
    /// Accepted for consistency with subagent nodes but not used. A say node cannot call tools.
    #[serde(default)]
    pub additional_tool_ids: Vec<String>,
    /// Human-readable label for the node used throughout the UI.
    #[serde(default)]
    pub label: String,
    /// Message spoken by this node. Use `literal` for exact text or `prompt` to have the LLM generate the message from a description.
    pub message: WorkflowSayNodeModelOutputMessage,
    /// When true, the workflow only moves to the next node after the message has been fully spoken.
    #[serde(default)]
    pub wait_for_speech_completion: bool,
    /// When true, the node speaks its message and then waits for the user to reply before evaluating outgoing edges, so edges can branch on the reply. If the user stays silent, the workflow advances after the turn timeout.
    #[serde(default)]
    pub wait_for_user_response: bool,
}

impl WorkflowSayNodeModelOutput {
    pub fn builder() -> WorkflowSayNodeModelOutputBuilder {
        <WorkflowSayNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowSayNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideOutput>,
    additional_prompt: Option<String>,
    additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    additional_tool_ids: Option<Vec<String>>,
    label: Option<String>,
    message: Option<WorkflowSayNodeModelOutputMessage>,
    wait_for_speech_completion: Option<bool>,
    wait_for_user_response: Option<bool>,
}

impl WorkflowSayNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn conversation_config(
        mut self,
        value: ConversationalConfigApiModelWorkflowOverrideOutput,
    ) -> Self {
        self.conversation_config = Some(value);
        self
    }

    pub fn additional_prompt(mut self, value: impl Into<String>) -> Self {
        self.additional_prompt = Some(value.into());
        self
    }

    pub fn additional_knowledge_base(mut self, value: Vec<KnowledgeBaseLocator>) -> Self {
        self.additional_knowledge_base = Some(value);
        self
    }

    pub fn additional_tool_ids(mut self, value: Vec<String>) -> Self {
        self.additional_tool_ids = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn message(mut self, value: WorkflowSayNodeModelOutputMessage) -> Self {
        self.message = Some(value);
        self
    }

    pub fn wait_for_speech_completion(mut self, value: bool) -> Self {
        self.wait_for_speech_completion = Some(value);
        self
    }

    pub fn wait_for_user_response(mut self, value: bool) -> Self {
        self.wait_for_user_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowSayNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowSayNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowSayNodeModelOutputBuilder::edge_order)
    /// - [`conversation_config`](WorkflowSayNodeModelOutputBuilder::conversation_config)
    /// - [`additional_prompt`](WorkflowSayNodeModelOutputBuilder::additional_prompt)
    /// - [`additional_knowledge_base`](WorkflowSayNodeModelOutputBuilder::additional_knowledge_base)
    /// - [`additional_tool_ids`](WorkflowSayNodeModelOutputBuilder::additional_tool_ids)
    /// - [`label`](WorkflowSayNodeModelOutputBuilder::label)
    /// - [`message`](WorkflowSayNodeModelOutputBuilder::message)
    /// - [`wait_for_speech_completion`](WorkflowSayNodeModelOutputBuilder::wait_for_speech_completion)
    /// - [`wait_for_user_response`](WorkflowSayNodeModelOutputBuilder::wait_for_user_response)
    pub fn build(self) -> Result<WorkflowSayNodeModelOutput, BuildError> {
        Ok(WorkflowSayNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            conversation_config: self
                .conversation_config
                .ok_or_else(|| BuildError::missing_field("conversation_config"))?,
            additional_prompt: self
                .additional_prompt
                .ok_or_else(|| BuildError::missing_field("additional_prompt"))?,
            additional_knowledge_base: self
                .additional_knowledge_base
                .ok_or_else(|| BuildError::missing_field("additional_knowledge_base"))?,
            additional_tool_ids: self
                .additional_tool_ids
                .ok_or_else(|| BuildError::missing_field("additional_tool_ids"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            wait_for_speech_completion: self
                .wait_for_speech_completion
                .ok_or_else(|| BuildError::missing_field("wait_for_speech_completion"))?,
            wait_for_user_response: self
                .wait_for_user_response
                .ok_or_else(|| BuildError::missing_field("wait_for_user_response"))?,
        })
    }
}
