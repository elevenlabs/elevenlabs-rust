pub use crate::prelude::*;

/// A common model for workflow tool responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolResponseModelOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<WorkflowToolResponseModelOutputStepsItem>>,
}

impl WorkflowToolResponseModelOutput {
    pub fn builder() -> WorkflowToolResponseModelOutputBuilder {
        <WorkflowToolResponseModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolResponseModelOutputBuilder {
    steps: Option<Vec<WorkflowToolResponseModelOutputStepsItem>>,
}

impl WorkflowToolResponseModelOutputBuilder {
    pub fn steps(mut self, value: Vec<WorkflowToolResponseModelOutputStepsItem>) -> Self {
        self.steps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolResponseModelOutput`].
    pub fn build(self) -> Result<WorkflowToolResponseModelOutput, BuildError> {
        Ok(WorkflowToolResponseModelOutput { steps: self.steps })
    }
}
