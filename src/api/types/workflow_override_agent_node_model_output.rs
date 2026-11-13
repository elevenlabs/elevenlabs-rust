pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowOverrideAgentNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    #[serde(default)]
    pub conversation_config: ConversationalConfigApiModelWorkflowOverrideOutput,
    /// Specific goal for this subagent. It will be added to the system prompt and can be used to further refine the agent's behavior in this specific context.
    #[serde(default)]
    pub additional_prompt: String,
    /// Additional knowledge base documents that the subagent has access to. These will be used in addition to the main agent's documents.
    #[serde(default)]
    pub additional_knowledge_base: Vec<KnowledgeBaseLocator>,
    /// IDs of additional tools that the subagent has access to. These will be used in addition to the main agent's tools.
    #[serde(default)]
    pub additional_tool_ids: Vec<String>,
    /// Human-readable label for the node used throughout the UI.
    #[serde(default)]
    pub label: String,
    pub entry_behavior: EntryBehavior,
}

impl WorkflowOverrideAgentNodeModelOutput {
    pub fn builder() -> WorkflowOverrideAgentNodeModelOutputBuilder {
        <WorkflowOverrideAgentNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowOverrideAgentNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideOutput>,
    additional_prompt: Option<String>,
    additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    additional_tool_ids: Option<Vec<String>>,
    label: Option<String>,
    entry_behavior: Option<EntryBehavior>,
}

impl WorkflowOverrideAgentNodeModelOutputBuilder {
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

    pub fn entry_behavior(mut self, value: EntryBehavior) -> Self {
        self.entry_behavior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowOverrideAgentNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowOverrideAgentNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowOverrideAgentNodeModelOutputBuilder::edge_order)
    /// - [`conversation_config`](WorkflowOverrideAgentNodeModelOutputBuilder::conversation_config)
    /// - [`additional_prompt`](WorkflowOverrideAgentNodeModelOutputBuilder::additional_prompt)
    /// - [`additional_knowledge_base`](WorkflowOverrideAgentNodeModelOutputBuilder::additional_knowledge_base)
    /// - [`additional_tool_ids`](WorkflowOverrideAgentNodeModelOutputBuilder::additional_tool_ids)
    /// - [`label`](WorkflowOverrideAgentNodeModelOutputBuilder::label)
    /// - [`entry_behavior`](WorkflowOverrideAgentNodeModelOutputBuilder::entry_behavior)
    pub fn build(self) -> Result<WorkflowOverrideAgentNodeModelOutput, BuildError> {
        Ok(WorkflowOverrideAgentNodeModelOutput {
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
            entry_behavior: self
                .entry_behavior
                .ok_or_else(|| BuildError::missing_field("entry_behavior"))?,
        })
    }
}
