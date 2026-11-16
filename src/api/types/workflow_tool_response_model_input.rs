pub use crate::prelude::*;

/// A common model for workflow tool responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolResponseModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<WorkflowToolResponseModelInputStepsItem>>,
}

impl WorkflowToolResponseModelInput {
    pub fn builder() -> WorkflowToolResponseModelInputBuilder {
        <WorkflowToolResponseModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolResponseModelInputBuilder {
    steps: Option<Vec<WorkflowToolResponseModelInputStepsItem>>,
}

impl WorkflowToolResponseModelInputBuilder {
    pub fn steps(mut self, value: Vec<WorkflowToolResponseModelInputStepsItem>) -> Self {
        self.steps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolResponseModelInput`].
    pub fn build(self) -> Result<WorkflowToolResponseModelInput, BuildError> {
        Ok(WorkflowToolResponseModelInput { steps: self.steps })
    }
}
