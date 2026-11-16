pub use crate::prelude::*;

/// A webhook tool is a tool that calls an external webhook from our server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookToolConfigOutput {
    #[serde(default)]
    pub name: String,
    /// Description of when the tool should be used and what it does.
    #[serde(default)]
    pub description: String,
    /// The maximum time in seconds to wait for the tool call to complete. Must be between 5 and 300 seconds (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub response_timeout_secs: Option<i64>,
    /// DEPRECATED: use `interruption_mode` instead. If true, the user will not be able to interrupt the agent while this tool is running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_interruptions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_mode: Option<ToolInterruptionMode>,
    /// DEPRECATED: use `pre_tool_speech` instead. If true, the agent will speak before the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_pre_tool_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_tool_speech: Option<PreToolSpeechMode>,
    /// Configuration for extracting values from tool responses and assigning them to dynamic variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<DynamicVariableAssignment>>,
    /// Predefined tool call sound type to play during tool execution. If not specified, no tool call sound will be played.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
    /// Determines how the tool call sound should be played.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    /// Controls how tool errors are processed before being shared with the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_error_handling_mode: Option<ToolErrorHandlingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<DynamicVariablesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
    pub api_schema: WebhookToolApiSchemaConfigOutput,
    /// Whether to resolve a redirect from the endpoint and return the final response. One redirect is followed, as a GET without the request body; nothing configured on this tool (headers, authentication, client certificate) is sent to the redirect target. Both the endpoint and the redirect target must use HTTPS. Not supported for API integration tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    /// Domains a redirect may point at, e.g. 'test.example.com'. Required when following redirects, and a target outside the list is refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects_allowed_domains: Option<Vec<String>>,
}

impl WebhookToolConfigOutput {
    pub fn builder() -> WebhookToolConfigOutputBuilder {
        <WebhookToolConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookToolConfigOutputBuilder {
    name: Option<String>,
    description: Option<String>,
    response_timeout_secs: Option<i64>,
    disable_interruptions: Option<bool>,
    interruption_mode: Option<ToolInterruptionMode>,
    force_pre_tool_speech: Option<bool>,
    pre_tool_speech: Option<PreToolSpeechMode>,
    assignments: Option<Vec<DynamicVariableAssignment>>,
    tool_call_sound: Option<ToolCallSoundType>,
    tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    tool_error_handling_mode: Option<ToolErrorHandlingMode>,
    dynamic_variables: Option<DynamicVariablesConfig>,
    execution_mode: Option<ToolExecutionMode>,
    api_schema: Option<WebhookToolApiSchemaConfigOutput>,
    follow_redirects: Option<bool>,
    follow_redirects_allowed_domains: Option<Vec<String>>,
}

impl WebhookToolConfigOutputBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn response_timeout_secs(mut self, value: i64) -> Self {
        self.response_timeout_secs = Some(value);
        self
    }

    pub fn disable_interruptions(mut self, value: bool) -> Self {
        self.disable_interruptions = Some(value);
        self
    }

    pub fn interruption_mode(mut self, value: ToolInterruptionMode) -> Self {
        self.interruption_mode = Some(value);
        self
    }

    pub fn force_pre_tool_speech(mut self, value: bool) -> Self {
        self.force_pre_tool_speech = Some(value);
        self
    }

    pub fn pre_tool_speech(mut self, value: PreToolSpeechMode) -> Self {
        self.pre_tool_speech = Some(value);
        self
    }

    pub fn assignments(mut self, value: Vec<DynamicVariableAssignment>) -> Self {
        self.assignments = Some(value);
        self
    }

    pub fn tool_call_sound(mut self, value: ToolCallSoundType) -> Self {
        self.tool_call_sound = Some(value);
        self
    }

    pub fn tool_call_sound_behavior(mut self, value: ToolCallSoundBehavior) -> Self {
        self.tool_call_sound_behavior = Some(value);
        self
    }

    pub fn tool_error_handling_mode(mut self, value: ToolErrorHandlingMode) -> Self {
        self.tool_error_handling_mode = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: DynamicVariablesConfig) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn api_schema(mut self, value: WebhookToolApiSchemaConfigOutput) -> Self {
        self.api_schema = Some(value);
        self
    }

    pub fn follow_redirects(mut self, value: bool) -> Self {
        self.follow_redirects = Some(value);
        self
    }

    pub fn follow_redirects_allowed_domains(mut self, value: Vec<String>) -> Self {
        self.follow_redirects_allowed_domains = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookToolConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WebhookToolConfigOutputBuilder::name)
    /// - [`description`](WebhookToolConfigOutputBuilder::description)
    /// - [`api_schema`](WebhookToolConfigOutputBuilder::api_schema)
    pub fn build(self) -> Result<WebhookToolConfigOutput, BuildError> {
        Ok(WebhookToolConfigOutput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            response_timeout_secs: self.response_timeout_secs,
            disable_interruptions: self.disable_interruptions,
            interruption_mode: self.interruption_mode,
            force_pre_tool_speech: self.force_pre_tool_speech,
            pre_tool_speech: self.pre_tool_speech,
            assignments: self.assignments,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior,
            tool_error_handling_mode: self.tool_error_handling_mode,
            dynamic_variables: self.dynamic_variables,
            execution_mode: self.execution_mode,
            api_schema: self
                .api_schema
                .ok_or_else(|| BuildError::missing_field("api_schema"))?,
            follow_redirects: self.follow_redirects,
            follow_redirects_allowed_domains: self.follow_redirects_allowed_domains,
        })
    }
}
