pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowSayNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>,
    /// Accepted for consistency with subagent nodes but not used. Describe the message in `message.prompt`, or replace the system prompt with `conversation_config.agent.prompt.prompt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_prompt: Option<String>,
    /// Knowledge base documents available to the LLM while this node generates its message, in addition to the agent's documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    /// Accepted for consistency with subagent nodes but not used. A say node cannot call tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tool_ids: Option<Vec<String>>,
    /// Human-readable label for the node used throughout the UI.
    #[serde(default)]
    pub label: String,
    /// Message spoken by this node. Use `literal` for exact text or `prompt` to have the LLM generate the message from a description.
    pub message: WorkflowSayNodeModelInputMessage,
    /// When true, the workflow only moves to the next node after the message has been fully spoken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_speech_completion: Option<bool>,
    /// When true, the node speaks its message and then waits for the user to reply before evaluating outgoing edges, so edges can branch on the reply. If the user stays silent, the workflow advances after the turn timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_user_response: Option<bool>,
}

impl WorkflowSayNodeModelInput {
    pub fn builder() -> WorkflowSayNodeModelInputBuilder {
        <WorkflowSayNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowSayNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>,
    additional_prompt: Option<String>,
    additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    additional_tool_ids: Option<Vec<String>>,
    label: Option<String>,
    message: Option<WorkflowSayNodeModelInputMessage>,
    wait_for_speech_completion: Option<bool>,
    wait_for_user_response: Option<bool>,
}

impl WorkflowSayNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn conversation_config(
        mut self,
        value: ConversationalConfigApiModelWorkflowOverrideInput,
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

    pub fn message(mut self, value: WorkflowSayNodeModelInputMessage) -> Self {
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

    /// Consumes the builder and constructs a [`WorkflowSayNodeModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](WorkflowSayNodeModelInputBuilder::label)
    /// - [`message`](WorkflowSayNodeModelInputBuilder::message)
    pub fn build(self) -> Result<WorkflowSayNodeModelInput, BuildError> {
        Ok(WorkflowSayNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            conversation_config: self.conversation_config,
            additional_prompt: self.additional_prompt,
            additional_knowledge_base: self.additional_knowledge_base,
            additional_tool_ids: self.additional_tool_ids,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            wait_for_speech_completion: self.wait_for_speech_completion,
            wait_for_user_response: self.wait_for_user_response,
        })
    }
}
