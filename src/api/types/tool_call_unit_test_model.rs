pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolCallUnitTestModel {
    /// Metadata of a conversation this test was created from (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
    /// Dynamic variables to replace in the agent config during testing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    /// Simulate the test as if the conversation originated from this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ListConversationInitiationSource>,
    /// The environment to resolve environment-specific variable values against when running this test (URL, headers, auth connections). If not provided, defaults to 'production'. For simulation tests, simulation_environment takes precedence when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// How to evaluate the agent's tool call (if any). If empty, the tool call is not evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>,
    /// If set to True this test will pass if any tool call returned by the LLM matches the criteria. Otherwise it will fail if more than one tool is returned by the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_any_tool_matches: Option<bool>,
}

impl ToolCallUnitTestModel {
    pub fn builder() -> ToolCallUnitTestModelBuilder {
        <ToolCallUnitTestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallUnitTestModelBuilder {
    from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
    dynamic_variables: Option<serde_json::Value>,
    chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    conversation_initiation_source: Option<ListConversationInitiationSource>,
    environment: Option<String>,
    tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>,
    check_any_tool_matches: Option<bool>,
}

impl ToolCallUnitTestModelBuilder {
    pub fn from_conversation_metadata(mut self, value: TestFromConversationMetadataOutput) -> Self {
        self.from_conversation_metadata = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: serde_json::Value) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn chat_history(
        mut self,
        value: Vec<ConversationHistoryTranscriptCommonModelOutput>,
    ) -> Self {
        self.chat_history = Some(value);
        self
    }

    pub fn conversation_initiation_source(
        mut self,
        value: ListConversationInitiationSource,
    ) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn tool_call_parameters(mut self, value: UnitTestToolCallEvaluationModelOutput) -> Self {
        self.tool_call_parameters = Some(value);
        self
    }

    pub fn check_any_tool_matches(mut self, value: bool) -> Self {
        self.check_any_tool_matches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolCallUnitTestModel`].
    pub fn build(self) -> Result<ToolCallUnitTestModel, BuildError> {
        Ok(ToolCallUnitTestModel {
            from_conversation_metadata: self.from_conversation_metadata,
            dynamic_variables: self.dynamic_variables,
            chat_history: self.chat_history,
            conversation_initiation_source: self.conversation_initiation_source,
            environment: self.environment,
            tool_call_parameters: self.tool_call_parameters,
            check_any_tool_matches: self.check_any_tool_matches,
        })
    }
}
