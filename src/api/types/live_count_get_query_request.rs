pub use crate::prelude::*;

/// Query parameters for Get Live Count
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LiveCountGetQueryRequest {
    /// The id of an agent to restrict the analytics to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Restrict analytics to the union of the given agents. Takes precedence over `agent_id` when both are supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

impl LiveCountGetQueryRequest {
    pub fn builder() -> LiveCountGetQueryRequestBuilder {
        <LiveCountGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiveCountGetQueryRequestBuilder {
    agent_id: Option<String>,
    agent_ids: Option<Vec<String>>,
}

impl LiveCountGetQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_ids(mut self, value: Vec<String>) -> Self {
        self.agent_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LiveCountGetQueryRequest`].
    pub fn build(self) -> Result<LiveCountGetQueryRequest, BuildError> {
        Ok(LiveCountGetQueryRequest {
            agent_id: self.agent_id,
            agent_ids: self.agent_ids,
        })
    }
}
