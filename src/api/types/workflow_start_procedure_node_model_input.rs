pub use crate::prelude::*;

/// Starts one procedure when the workflow reaches it.
///
/// A freeform procedure is injected and traversal continues along the node's
/// outgoing edges in the same turn. A structured procedure transfers into its
/// compiled island with this node as the return node; when it ends, the
/// workflow runner hosts this node and evaluates its outgoing edges.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStartProcedureNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
    /// ID of the procedure to start when the workflow reaches this node.
    #[serde(default)]
    pub procedure_id: String,
}

impl WorkflowStartProcedureNodeModelInput {
    pub fn builder() -> WorkflowStartProcedureNodeModelInputBuilder {
        <WorkflowStartProcedureNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStartProcedureNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
    procedure_id: Option<String>,
}

impl WorkflowStartProcedureNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
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

    /// Consumes the builder and constructs a [`WorkflowStartProcedureNodeModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](WorkflowStartProcedureNodeModelInputBuilder::procedure_id)
    pub fn build(self) -> Result<WorkflowStartProcedureNodeModelInput, BuildError> {
        Ok(WorkflowStartProcedureNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
            procedure_id: self
                .procedure_id
                .ok_or_else(|| BuildError::missing_field("procedure_id"))?,
        })
    }
}
