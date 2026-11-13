pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowStartNodeModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
}

impl WorkflowStartNodeModelInput {
    pub fn builder() -> WorkflowStartNodeModelInputBuilder {
        <WorkflowStartNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStartNodeModelInputBuilder {
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
}

impl WorkflowStartNodeModelInputBuilder {
    pub fn position(mut self, value: PositionInput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowStartNodeModelInput`].
    pub fn build(self) -> Result<WorkflowStartNodeModelInput, BuildError> {
        Ok(WorkflowStartNodeModelInput {
            position: self.position,
            edge_order: self.edge_order,
        })
    }
}
