pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAgentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<ConversationalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<AgentPlatformSettingsRequestModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<AgentWorkflowRequestModel>,
    /// A name to make the agent easier to find
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags to help classify and filter the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Description for this version when publishing changes (only applicable for versioned agents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    /// Procedure versions to publish, keyed by procedure_id. When provided, this map replaces the procedures from the current draft or branch tip. When omitted or null, unpublished procedure edits are used if present; otherwise, the branch tip's procedures are retained. Pass an empty object to remove all procedures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedures: Option<std::collections::HashMap<String, ProcedureVersionRef>>,
}

impl UpdateAgentRequest {
    pub fn builder() -> UpdateAgentRequestBuilder {
        <UpdateAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentRequestBuilder {
    conversation_config: Option<ConversationalConfig>,
    platform_settings: Option<AgentPlatformSettingsRequestModel>,
    workflow: Option<AgentWorkflowRequestModel>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    version_description: Option<String>,
    procedures: Option<std::collections::HashMap<String, ProcedureVersionRef>>,
}

impl UpdateAgentRequestBuilder {
    pub fn conversation_config(mut self, value: ConversationalConfig) -> Self {
        self.conversation_config = Some(value);
        self
    }

    pub fn platform_settings(mut self, value: AgentPlatformSettingsRequestModel) -> Self {
        self.platform_settings = Some(value);
        self
    }

    pub fn workflow(mut self, value: AgentWorkflowRequestModel) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn version_description(mut self, value: impl Into<String>) -> Self {
        self.version_description = Some(value.into());
        self
    }

    pub fn procedures(
        mut self,
        value: std::collections::HashMap<String, ProcedureVersionRef>,
    ) -> Self {
        self.procedures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgentRequest`].
    pub fn build(self) -> Result<UpdateAgentRequest, BuildError> {
        Ok(UpdateAgentRequest {
            conversation_config: self.conversation_config,
            platform_settings: self.platform_settings,
            workflow: self.workflow,
            name: self.name,
            tags: self.tags,
            version_description: self.version_description,
            procedures: self.procedures,
        })
    }
}
