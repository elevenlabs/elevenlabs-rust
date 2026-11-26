pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateToolCallUnitTestRequest {
    /// Metadata of a conversation this test was created from (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_conversation_metadata: Option<TestFromConversationMetadataInput>,
    /// Dynamic variables to replace in the agent config during testing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelInput>>,
    /// Simulate the test as if the conversation originated from this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
    /// The environment to resolve environment-specific variable values against when running this test (URL, headers, auth connections). If not provided, defaults to 'production'. For simulation tests, simulation_environment takes precedence when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// How to evaluate the agent's tool call (if any). If empty, the tool call is not evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_parameters: Option<UnitTestToolCallEvaluationModelInput>,
    /// If set to True this test will pass if any tool call returned by the LLM matches the criteria. Otherwise it will fail if more than one tool is returned by the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_any_tool_matches: Option<bool>,
    #[serde(default)]
    pub name: String,
    /// The ID of the parent folder. If not provided, the test will be created at the root level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl CreateToolCallUnitTestRequest {
    pub fn builder() -> CreateToolCallUnitTestRequestBuilder {
        <CreateToolCallUnitTestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateToolCallUnitTestRequestBuilder {
    from_conversation_metadata: Option<TestFromConversationMetadataInput>,
    dynamic_variables: Option<serde_json::Value>,
    chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelInput>>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    environment: Option<String>,
    tool_call_parameters: Option<UnitTestToolCallEvaluationModelInput>,
    check_any_tool_matches: Option<bool>,
    name: Option<String>,
    parent_folder_id: Option<String>,
}

impl CreateToolCallUnitTestRequestBuilder {
    pub fn from_conversation_metadata(mut self, value: TestFromConversationMetadataInput) -> Self {
        self.from_conversation_metadata = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: serde_json::Value) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn chat_history(
        mut self,
        value: Vec<ConversationHistoryTranscriptCommonModelInput>,
    ) -> Self {
        self.chat_history = Some(value);
        self
    }

    pub fn conversation_initiation_source(mut self, value: ConversationInitiationSource) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn tool_call_parameters(mut self, value: UnitTestToolCallEvaluationModelInput) -> Self {
        self.tool_call_parameters = Some(value);
        self
    }

    pub fn check_any_tool_matches(mut self, value: bool) -> Self {
        self.check_any_tool_matches = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateToolCallUnitTestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateToolCallUnitTestRequestBuilder::name)
    pub fn build(self) -> Result<CreateToolCallUnitTestRequest, BuildError> {
        Ok(CreateToolCallUnitTestRequest {
            from_conversation_metadata: self.from_conversation_metadata,
            dynamic_variables: self.dynamic_variables,
            chat_history: self.chat_history,
            conversation_initiation_source: self.conversation_initiation_source,
            environment: self.environment,
            tool_call_parameters: self.tool_call_parameters,
            check_any_tool_matches: self.check_any_tool_matches,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_folder_id: self.parent_folder_id,
        })
    }
}
