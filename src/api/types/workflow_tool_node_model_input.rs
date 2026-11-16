pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    /// List of tools to execute in parallel. The entire node is considered successful if all tools are executed successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<WorkflowToolLocator>>,
}

impl WorkflowToolNodeModelInput {
    pub fn builder() -> WorkflowToolNodeModelInputBuilder {
        <WorkflowToolNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    tools: Option<Vec<WorkflowToolLocator>>,
}

impl WorkflowToolNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<WorkflowToolLocator>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolNodeModelInput`].
    pub fn build(self) -> Result<WorkflowToolNodeModelInput, BuildError> {
        Ok(WorkflowToolNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            tools: self.tools,
        })
    }
}
