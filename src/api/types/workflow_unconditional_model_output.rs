pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowUnconditionalModelOutput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(default)]
    pub label: Option<String>,
}

impl WorkflowUnconditionalModelOutput {
    pub fn builder() -> WorkflowUnconditionalModelOutputBuilder {
        <WorkflowUnconditionalModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUnconditionalModelOutputBuilder {
    label: Option<String>,
}

impl WorkflowUnconditionalModelOutputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUnconditionalModelOutput`].
    pub fn build(self) -> Result<WorkflowUnconditionalModelOutput, BuildError> {
        Ok(WorkflowUnconditionalModelOutput { label: self.label })
    }
}
