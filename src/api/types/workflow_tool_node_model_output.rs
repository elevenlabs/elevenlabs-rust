pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowToolNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    /// List of tools to execute in parallel. The entire node is considered successful if all tools are executed successfully.
    #[serde(default)]
    pub tools: Vec<WorkflowToolLocator>,
}

impl WorkflowToolNodeModelOutput {
    pub fn builder() -> WorkflowToolNodeModelOutputBuilder {
        <WorkflowToolNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    tools: Option<Vec<WorkflowToolLocator>>,
}

impl WorkflowToolNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
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

    /// Consumes the builder and constructs a [`WorkflowToolNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowToolNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowToolNodeModelOutputBuilder::edge_order)
    /// - [`tools`](WorkflowToolNodeModelOutputBuilder::tools)
    pub fn build(self) -> Result<WorkflowToolNodeModelOutput, BuildError> {
        Ok(WorkflowToolNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
