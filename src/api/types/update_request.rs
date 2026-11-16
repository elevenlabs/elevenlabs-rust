pub use crate::prelude::*;

/// Request for Update (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateRequest {
    /// Deprecated: all agents are versioned. This parameter is ignored.
    #[serde(skip)]
    pub enable_versioning_if_not_enabled: Option<bool>,
    /// The ID of the branch to use
    #[serde(skip)]
    pub branch_id: Option<String>,
    /// Whether to include the workflow nodes and edges generated from structured procedures. When false, the workflow contains only authored elements.
    #[serde(skip)]
    pub include_procedure_generated_workflow_elements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateAgentRequest>,
}

impl UpdateRequest {
    pub fn builder() -> UpdateRequestBuilder {
        <UpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRequestBuilder {
    enable_versioning_if_not_enabled: Option<bool>,
    branch_id: Option<String>,
    include_procedure_generated_workflow_elements: Option<bool>,
    body: Option<UpdateAgentRequest>,
}

impl UpdateRequestBuilder {
    pub fn enable_versioning_if_not_enabled(mut self, value: bool) -> Self {
        self.enable_versioning_if_not_enabled = Some(value);
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn include_procedure_generated_workflow_elements(mut self, value: bool) -> Self {
        self.include_procedure_generated_workflow_elements = Some(value);
        self
    }

    pub fn body(mut self, value: UpdateAgentRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRequest`].
    pub fn build(self) -> Result<UpdateRequest, BuildError> {
        Ok(UpdateRequest {
            enable_versioning_if_not_enabled: self.enable_versioning_if_not_enabled,
            branch_id: self.branch_id,
            include_procedure_generated_workflow_elements: self
                .include_procedure_generated_workflow_elements,
            body: self.body,
        })
    }
}
