pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponseAgentDeploymentHistoryItem_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListResponseMeta>,
    #[serde(default)]
    pub results: Vec<AgentDeploymentHistoryItem>,
}

impl ListResponseAgentDeploymentHistoryItem_ {
    pub fn builder() -> ListResponseAgentDeploymentHistoryItem_Builder {
        <ListResponseAgentDeploymentHistoryItem_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseAgentDeploymentHistoryItem_Builder {
    meta: Option<ListResponseMeta>,
    results: Option<Vec<AgentDeploymentHistoryItem>>,
}

impl ListResponseAgentDeploymentHistoryItem_Builder {
    pub fn meta(mut self, value: ListResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<AgentDeploymentHistoryItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseAgentDeploymentHistoryItem_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ListResponseAgentDeploymentHistoryItem_Builder::results)
    pub fn build(self) -> Result<ListResponseAgentDeploymentHistoryItem_, BuildError> {
        Ok(ListResponseAgentDeploymentHistoryItem_ {
            meta: self.meta,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
