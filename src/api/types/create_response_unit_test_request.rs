pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponseUnitTestRequest {
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
    pub conversation_initiation_source: Option<ListConversationInitiationSource>,
    /// The environment to resolve environment-specific variable values against when running this test (URL, headers, auth connections). If not provided, defaults to 'production'. For simulation tests, simulation_environment takes precedence when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// A prompt that evaluates whether the agent's response is successful. Should return True or False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
    /// Non-empty list of example responses that should be considered successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_examples: Option<Vec<AgentSuccessfulResponseExample>>,
    /// Non-empty list of example responses that should be considered failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_examples: Option<Vec<AgentFailureResponseExample>>,
    #[serde(default)]
    pub name: String,
    /// The ID of the parent folder. If not provided, the test will be created at the root level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl CreateResponseUnitTestRequest {
    pub fn builder() -> CreateResponseUnitTestRequestBuilder {
        <CreateResponseUnitTestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateResponseUnitTestRequestBuilder {
    from_conversation_metadata: Option<TestFromConversationMetadataInput>,
    dynamic_variables: Option<serde_json::Value>,
    chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelInput>>,
    conversation_initiation_source: Option<ListConversationInitiationSource>,
    environment: Option<String>,
    success_condition: Option<String>,
    success_examples: Option<Vec<AgentSuccessfulResponseExample>>,
    failure_examples: Option<Vec<AgentFailureResponseExample>>,
    name: Option<String>,
    parent_folder_id: Option<String>,
}

impl CreateResponseUnitTestRequestBuilder {
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

    pub fn success_condition(mut self, value: impl Into<String>) -> Self {
        self.success_condition = Some(value.into());
        self
    }

    pub fn success_examples(mut self, value: Vec<AgentSuccessfulResponseExample>) -> Self {
        self.success_examples = Some(value);
        self
    }

    pub fn failure_examples(mut self, value: Vec<AgentFailureResponseExample>) -> Self {
        self.failure_examples = Some(value);
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

    /// Consumes the builder and constructs a [`CreateResponseUnitTestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateResponseUnitTestRequestBuilder::name)
    pub fn build(self) -> Result<CreateResponseUnitTestRequest, BuildError> {
        Ok(CreateResponseUnitTestRequest {
            from_conversation_metadata: self.from_conversation_metadata,
            dynamic_variables: self.dynamic_variables,
            chat_history: self.chat_history,
            conversation_initiation_source: self.conversation_initiation_source,
            environment: self.environment,
            success_condition: self.success_condition,
            success_examples: self.success_examples,
            failure_examples: self.failure_examples,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_folder_id: self.parent_folder_id,
        })
    }
}
