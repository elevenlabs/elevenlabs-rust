pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponseAgentBranchSummary_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListResponseMeta>,
    #[serde(default)]
    pub results: Vec<AgentBranchSummary>,
}

impl ListResponseAgentBranchSummary_ {
    pub fn builder() -> ListResponseAgentBranchSummary_Builder {
        <ListResponseAgentBranchSummary_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseAgentBranchSummary_Builder {
    meta: Option<ListResponseMeta>,
    results: Option<Vec<AgentBranchSummary>>,
}

impl ListResponseAgentBranchSummary_Builder {
    pub fn meta(mut self, value: ListResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<AgentBranchSummary>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseAgentBranchSummary_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ListResponseAgentBranchSummary_Builder::results)
    pub fn build(self) -> Result<ListResponseAgentBranchSummary_, BuildError> {
        Ok(ListResponseAgentBranchSummary_ {
            meta: self.meta,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
