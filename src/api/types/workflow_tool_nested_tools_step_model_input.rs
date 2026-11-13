pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowToolNestedToolsStepModelInput {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub step_latency_secs: f64,
    #[serde(default)]
    pub node_id: String,
    #[serde(default)]
    pub requests: Vec<ConversationHistoryTranscriptToolCallCommonModelInput>,
    #[serde(default)]
    pub results: Vec<WorkflowToolNestedToolsStepModelInputResultsItem>,
    #[serde(default)]
    pub is_successful: bool,
}

impl WorkflowToolNestedToolsStepModelInput {
    pub fn builder() -> WorkflowToolNestedToolsStepModelInputBuilder {
        <WorkflowToolNestedToolsStepModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolNestedToolsStepModelInputBuilder {
    step_latency_secs: Option<f64>,
    node_id: Option<String>,
    requests: Option<Vec<ConversationHistoryTranscriptToolCallCommonModelInput>>,
    results: Option<Vec<WorkflowToolNestedToolsStepModelInputResultsItem>>,
    is_successful: Option<bool>,
}

impl WorkflowToolNestedToolsStepModelInputBuilder {
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
        value: Vec<ConversationHistoryTranscriptToolCallCommonModelInput>,
    ) -> Self {
        self.requests = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<WorkflowToolNestedToolsStepModelInputResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn is_successful(mut self, value: bool) -> Self {
        self.is_successful = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolNestedToolsStepModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`step_latency_secs`](WorkflowToolNestedToolsStepModelInputBuilder::step_latency_secs)
    /// - [`node_id`](WorkflowToolNestedToolsStepModelInputBuilder::node_id)
    /// - [`requests`](WorkflowToolNestedToolsStepModelInputBuilder::requests)
    /// - [`results`](WorkflowToolNestedToolsStepModelInputBuilder::results)
    /// - [`is_successful`](WorkflowToolNestedToolsStepModelInputBuilder::is_successful)
    pub fn build(self) -> Result<WorkflowToolNestedToolsStepModelInput, BuildError> {
        Ok(WorkflowToolNestedToolsStepModelInput {
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
