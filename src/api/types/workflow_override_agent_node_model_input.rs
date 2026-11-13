pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowOverrideAgentNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>,
    /// Specific goal for this subagent. It will be added to the system prompt and can be used to further refine the agent's behavior in this specific context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_prompt: Option<String>,
    /// Additional knowledge base documents that the subagent has access to. These will be used in addition to the main agent's documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    /// IDs of additional tools that the subagent has access to. These will be used in addition to the main agent's tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tool_ids: Option<Vec<String>>,
    /// Human-readable label for the node used throughout the UI.
    #[serde(default)]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_behavior: Option<EntryBehavior>,
}

impl WorkflowOverrideAgentNodeModelInput {
    pub fn builder() -> WorkflowOverrideAgentNodeModelInputBuilder {
        <WorkflowOverrideAgentNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowOverrideAgentNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>,
    additional_prompt: Option<String>,
    additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    additional_tool_ids: Option<Vec<String>>,
    label: Option<String>,
    entry_behavior: Option<EntryBehavior>,
}

impl WorkflowOverrideAgentNodeModelInputBuilder {
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

    pub fn entry_behavior(mut self, value: EntryBehavior) -> Self {
        self.entry_behavior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowOverrideAgentNodeModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](WorkflowOverrideAgentNodeModelInputBuilder::label)
    pub fn build(self) -> Result<WorkflowOverrideAgentNodeModelInput, BuildError> {
        Ok(WorkflowOverrideAgentNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            conversation_config: self.conversation_config,
            additional_prompt: self.additional_prompt,
            additional_knowledge_base: self.additional_knowledge_base,
            additional_tool_ids: self.additional_tool_ids,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            entry_behavior: self.entry_behavior,
        })
    }
}
