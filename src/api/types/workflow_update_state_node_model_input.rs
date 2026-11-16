pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowUpdateStateNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    /// Human-readable label for the node used throughout the UI.
    #[serde(default)]
    pub label: String,
    /// State updates applied when the workflow reaches this node. A node accepts at most 50 updates.
    #[serde(default)]
    pub updates: Vec<WorkflowStateUpdateInput>,
}

impl WorkflowUpdateStateNodeModelInput {
    pub fn builder() -> WorkflowUpdateStateNodeModelInputBuilder {
        <WorkflowUpdateStateNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUpdateStateNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    label: Option<String>,
    updates: Option<Vec<WorkflowStateUpdateInput>>,
}

impl WorkflowUpdateStateNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
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

    pub fn updates(mut self, value: Vec<WorkflowStateUpdateInput>) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUpdateStateNodeModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](WorkflowUpdateStateNodeModelInputBuilder::label)
    /// - [`updates`](WorkflowUpdateStateNodeModelInputBuilder::updates)
    pub fn build(self) -> Result<WorkflowUpdateStateNodeModelInput, BuildError> {
        Ok(WorkflowUpdateStateNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            updates: self
                .updates
                .ok_or_else(|| BuildError::missing_field("updates"))?,
        })
    }
}
