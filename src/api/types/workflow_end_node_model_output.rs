pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowEndNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    /// Whether reaching an End node completes successfully or fails.
    pub outcome: WorkflowEndNodeOutcome,
}

impl WorkflowEndNodeModelOutput {
    pub fn builder() -> WorkflowEndNodeModelOutputBuilder {
        <WorkflowEndNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowEndNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    outcome: Option<WorkflowEndNodeOutcome>,
}

impl WorkflowEndNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn outcome(mut self, value: WorkflowEndNodeOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowEndNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowEndNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowEndNodeModelOutputBuilder::edge_order)
    /// - [`outcome`](WorkflowEndNodeModelOutputBuilder::outcome)
    pub fn build(self) -> Result<WorkflowEndNodeModelOutput, BuildError> {
        Ok(WorkflowEndNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            outcome: self
                .outcome
                .ok_or_else(|| BuildError::missing_field("outcome"))?,
        })
    }
}
