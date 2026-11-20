pub use crate::prelude::*;

/// Starts one procedure when the workflow reaches it.
///
/// A freeform procedure is injected and traversal continues along the node's
/// outgoing edges in the same turn. A structured procedure transfers into its
/// compiled island with this node as the return node; when it ends, the
/// workflow runner hosts this node and evaluates its outgoing edges.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStartProcedureNodeModelOutput {
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    /// ID of the procedure to start when the workflow reaches this node.
    #[serde(default)]
    pub procedure_id: String,
}

impl WorkflowStartProcedureNodeModelOutput {
    pub fn builder() -> WorkflowStartProcedureNodeModelOutputBuilder {
        <WorkflowStartProcedureNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStartProcedureNodeModelOutputBuilder {
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    procedure_id: Option<String>,
}

impl WorkflowStartProcedureNodeModelOutputBuilder {
    pub fn position(mut self, value: PositionOutput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowStartProcedureNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](WorkflowStartProcedureNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowStartProcedureNodeModelOutputBuilder::edge_order)
    /// - [`procedure_id`](WorkflowStartProcedureNodeModelOutputBuilder::procedure_id)
    pub fn build(self) -> Result<WorkflowStartProcedureNodeModelOutput, BuildError> {
        Ok(WorkflowStartProcedureNodeModelOutput {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            procedure_id: self
                .procedure_id
                .ok_or_else(|| BuildError::missing_field("procedure_id"))?,
        })
    }
}
