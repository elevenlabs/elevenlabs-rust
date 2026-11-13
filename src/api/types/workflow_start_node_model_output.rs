pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStartNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
}

impl WorkflowStartNodeModelOutput {
    pub fn builder() -> WorkflowStartNodeModelOutputBuilder {
        <WorkflowStartNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStartNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
}

impl WorkflowStartNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowStartNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowStartNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowStartNodeModelOutputBuilder::edge_order)
    pub fn build(self) -> Result<WorkflowStartNodeModelOutput, BuildError> {
        Ok(WorkflowStartNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
        })
    }
}
