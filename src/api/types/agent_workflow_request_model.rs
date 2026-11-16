pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentWorkflowRequestModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<std::collections::HashMap<String, WorkflowEdgeModelInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<std::collections::HashMap<String, AgentWorkflowRequestModelNodes>>,
    /// Whether to prevent loops in the workflow execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_subagent_loops: Option<bool>,
}

impl AgentWorkflowRequestModel {
    pub fn builder() -> AgentWorkflowRequestModelBuilder {
        <AgentWorkflowRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentWorkflowRequestModelBuilder {
    edges: Option<std::collections::HashMap<String, WorkflowEdgeModelInput>>,
    nodes: Option<std::collections::HashMap<String, AgentWorkflowRequestModelNodes>>,
    prevent_subagent_loops: Option<bool>,
}

impl AgentWorkflowRequestModelBuilder {
    pub fn edges(
        mut self,
        value: std::collections::HashMap<String, WorkflowEdgeModelInput>,
    ) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn nodes(
        mut self,
        value: std::collections::HashMap<String, AgentWorkflowRequestModelNodes>,
    ) -> Self {
        self.nodes = Some(value);
        self
    }

    pub fn prevent_subagent_loops(mut self, value: bool) -> Self {
        self.prevent_subagent_loops = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentWorkflowRequestModel`].
    pub fn build(self) -> Result<AgentWorkflowRequestModel, BuildError> {
        Ok(AgentWorkflowRequestModel {
            edges: self.edges,
            nodes: self.nodes,
            prevent_subagent_loops: self.prevent_subagent_loops,
        })
    }
}
