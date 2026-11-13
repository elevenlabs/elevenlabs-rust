pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowUpdateStateNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    /// Human-readable label for the node used throughout the UI.
    #[serde(default)]
    pub label: String,
    /// State updates applied when the workflow reaches this node. A node accepts at most 50 updates.
    #[serde(default)]
    pub updates: Vec<WorkflowStateUpdateOutput>,
}

impl WorkflowUpdateStateNodeModelOutput {
    pub fn builder() -> WorkflowUpdateStateNodeModelOutputBuilder {
        <WorkflowUpdateStateNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUpdateStateNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    label: Option<String>,
    updates: Option<Vec<WorkflowStateUpdateOutput>>,
}

impl WorkflowUpdateStateNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn updates(mut self, value: Vec<WorkflowStateUpdateOutput>) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUpdateStateNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowUpdateStateNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowUpdateStateNodeModelOutputBuilder::edge_order)
    /// - [`label`](WorkflowUpdateStateNodeModelOutputBuilder::label)
    /// - [`updates`](WorkflowUpdateStateNodeModelOutputBuilder::updates)
    pub fn build(self) -> Result<WorkflowUpdateStateNodeModelOutput, BuildError> {
        Ok(WorkflowUpdateStateNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            updates: self
                .updates
                .ok_or_else(|| BuildError::missing_field("updates"))?,
        })
    }
}
