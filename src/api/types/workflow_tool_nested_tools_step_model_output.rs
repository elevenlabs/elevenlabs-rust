pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowToolNestedToolsStepModelOutput {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub step_latency_secs: f64,
    #[serde(default)]
    pub node_id: String,
    #[serde(default)]
    pub requests: Vec<ConversationHistoryTranscriptToolCallCommonModelOutput>,
    #[serde(default)]
    pub results: Vec<WorkflowToolNestedToolsStepModelOutputResultsItem>,
    #[serde(default)]
    pub is_successful: bool,
}

impl WorkflowToolNestedToolsStepModelOutput {
    pub fn builder() -> WorkflowToolNestedToolsStepModelOutputBuilder {
        <WorkflowToolNestedToolsStepModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolNestedToolsStepModelOutputBuilder {
    step_latency_secs: Option<f64>,
    node_id: Option<String>,
    requests: Option<Vec<ConversationHistoryTranscriptToolCallCommonModelOutput>>,
    results: Option<Vec<WorkflowToolNestedToolsStepModelOutputResultsItem>>,
    is_successful: Option<bool>,
}

impl WorkflowToolNestedToolsStepModelOutputBuilder {
    pub fn step_latency_secs(mut self, value: f64) -> Self {
        self.step_latency_secs = Some(value);
        self
    }

    pub fn node_id(mut self, value: impl Into<String>) -> Self {
        self.node_id = Some(value.into());
        self
    }

    pub fn requests(
        mut self,
        value: Vec<ConversationHistoryTranscriptToolCallCommonModelOutput>,
    ) -> Self {
        self.requests = Some(value);
        self
    }

    pub fn results(
        mut self,
        value: Vec<WorkflowToolNestedToolsStepModelOutputResultsItem>,
    ) -> Self {
        self.results = Some(value);
        self
    }

    pub fn is_successful(mut self, value: bool) -> Self {
        self.is_successful = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolNestedToolsStepModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`step_latency_secs`](WorkflowToolNestedToolsStepModelOutputBuilder::step_latency_secs)
    /// - [`node_id`](WorkflowToolNestedToolsStepModelOutputBuilder::node_id)
    /// - [`requests`](WorkflowToolNestedToolsStepModelOutputBuilder::requests)
    /// - [`results`](WorkflowToolNestedToolsStepModelOutputBuilder::results)
    /// - [`is_successful`](WorkflowToolNestedToolsStepModelOutputBuilder::is_successful)
    pub fn build(self) -> Result<WorkflowToolNestedToolsStepModelOutput, BuildError> {
        Ok(WorkflowToolNestedToolsStepModelOutput {
            step_latency_secs: self
                .step_latency_secs
                .ok_or_else(|| BuildError::missing_field("step_latency_secs"))?,
            node_id: self
                .node_id
                .ok_or_else(|| BuildError::missing_field("node_id"))?,
            requests: self
                .requests
                .ok_or_else(|| BuildError::missing_field("requests"))?,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
            is_successful: self
                .is_successful
                .ok_or_else(|| BuildError::missing_field("is_successful"))?,
        })
    }
}
