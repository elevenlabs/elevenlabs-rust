pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowEndNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    /// Whether reaching an End node completes successfully or fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<WorkflowEndNodeOutcome>,
}

impl WorkflowEndNodeModelInput {
    pub fn builder() -> WorkflowEndNodeModelInputBuilder {
        <WorkflowEndNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowEndNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    outcome: Option<WorkflowEndNodeOutcome>,
}

impl WorkflowEndNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
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

    /// Consumes the builder and constructs a [`WorkflowEndNodeModelInput`].
    pub fn build(self) -> Result<WorkflowEndNodeModelInput, BuildError> {
        Ok(WorkflowEndNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            outcome: self.outcome,
        })
    }
}
