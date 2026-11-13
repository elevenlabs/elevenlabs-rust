pub use crate::prelude::*;

/// Query parameters for Get Agent Summaries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SummariesGetQueryRequest {
    /// List of agent IDs to fetch summaries for
    pub agent_ids: Vec<String>,
}

impl SummariesGetQueryRequest {
    pub fn builder() -> SummariesGetQueryRequestBuilder {
        <SummariesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummariesGetQueryRequestBuilder {
    agent_ids: Option<Vec<String>>,
}

impl SummariesGetQueryRequestBuilder {
    pub fn agent_ids(mut self, value: Vec<String>) -> Self {
        self.agent_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummariesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_ids`](SummariesGetQueryRequestBuilder::agent_ids)
    pub fn build(self) -> Result<SummariesGetQueryRequest, BuildError> {
        Ok(SummariesGetQueryRequest {
            agent_ids: self
                .agent_ids
                .ok_or_else(|| BuildError::missing_field("agent_ids"))?,
        })
    }
}
