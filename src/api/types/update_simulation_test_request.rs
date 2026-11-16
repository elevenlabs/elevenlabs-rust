pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateSimulationTestRequest {
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
    /// Deprecated legacy single success criterion. Use success_conditions instead. At least one of success_condition or success_conditions is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
    /// List of prompts that evaluate whether the simulation was successful. If provided, all criteria are evaluated and merged into a final result. Capped at the maximum number of evaluation criteria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_conditions: Option<Vec<String>>,
    /// Description of the simulation scenario and user persona for simulation tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_scenario: Option<String>,
    /// Maximum number of conversation turns for simulation tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub simulation_max_turns: Option<i64>,
    /// The environment to use when running this simulation test. If not provided, defaults to 'production'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_environment: Option<String>,
    /// Simulation/preview-side config: tools are identified by IDs, resolved to names at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_mock_config: Option<SimulationToolMockBehaviorConfig>,
    /// Test-specific response mocks, keyed by tool ID. Applied ahead of the tool's shared mocks and only within this test. Only take effect for tools that are mocked (see tool_mock_config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_mock_overrides:
        Option<std::collections::HashMap<String, Vec<ToolResponseMockConfigInput>>>,
    /// LLM model to use for evaluating simulation results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_model: Option<Llm>,
    /// LLM model for the simulated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulated_user_model: Option<Llm>,
    #[serde(default)]
    pub name: String,
    /// The ID of the parent folder. If not provided, the test will be moved to the root level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl UpdateSimulationTestRequest {
    pub fn builder() -> UpdateSimulationTestRequestBuilder {
        <UpdateSimulationTestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSimulationTestRequestBuilder {
    from_conversation_metadata: Option<TestFromConversationMetadataInput>,
    dynamic_variables: Option<serde_json::Value>,
    chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelInput>>,
    conversation_initiation_source: Option<ListConversationInitiationSource>,
    environment: Option<String>,
    success_condition: Option<String>,
    success_conditions: Option<Vec<String>>,
    simulation_scenario: Option<String>,
    simulation_max_turns: Option<i64>,
    simulation_environment: Option<String>,
    tool_mock_config: Option<SimulationToolMockBehaviorConfig>,
    tool_mock_overrides:
        Option<std::collections::HashMap<String, Vec<ToolResponseMockConfigInput>>>,
    evaluation_model: Option<Llm>,
    simulated_user_model: Option<Llm>,
    name: Option<String>,
    parent_folder_id: Option<String>,
}

impl UpdateSimulationTestRequestBuilder {
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

    pub fn success_conditions(mut self, value: Vec<String>) -> Self {
        self.success_conditions = Some(value);
        self
    }

    pub fn simulation_scenario(mut self, value: impl Into<String>) -> Self {
        self.simulation_scenario = Some(value.into());
        self
    }

    pub fn simulation_max_turns(mut self, value: i64) -> Self {
        self.simulation_max_turns = Some(value);
        self
    }

    pub fn simulation_environment(mut self, value: impl Into<String>) -> Self {
        self.simulation_environment = Some(value.into());
        self
    }

    pub fn tool_mock_config(mut self, value: SimulationToolMockBehaviorConfig) -> Self {
        self.tool_mock_config = Some(value);
        self
    }

    pub fn tool_mock_overrides(
        mut self,
        value: std::collections::HashMap<String, Vec<ToolResponseMockConfigInput>>,
    ) -> Self {
        self.tool_mock_overrides = Some(value);
        self
    }

    pub fn evaluation_model(mut self, value: Llm) -> Self {
        self.evaluation_model = Some(value);
        self
    }

    pub fn simulated_user_model(mut self, value: Llm) -> Self {
        self.simulated_user_model = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateSimulationTestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateSimulationTestRequestBuilder::name)
    pub fn build(self) -> Result<UpdateSimulationTestRequest, BuildError> {
        Ok(UpdateSimulationTestRequest {
            from_conversation_metadata: self.from_conversation_metadata,
            dynamic_variables: self.dynamic_variables,
            chat_history: self.chat_history,
            conversation_initiation_source: self.conversation_initiation_source,
            environment: self.environment,
            success_condition: self.success_condition,
            success_conditions: self.success_conditions,
            simulation_scenario: self.simulation_scenario,
            simulation_max_turns: self.simulation_max_turns,
            simulation_environment: self.simulation_environment,
            tool_mock_config: self.tool_mock_config,
            tool_mock_overrides: self.tool_mock_overrides,
            evaluation_model: self.evaluation_model,
            simulated_user_model: self.simulated_user_model,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_folder_id: self.parent_folder_id,
        })
    }
}
