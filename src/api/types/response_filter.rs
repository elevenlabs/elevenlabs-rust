pub use crate::prelude::*;

/// Configuration for filtering tool responses before they are visible to the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponseFilter {
    /// Controls how tool responses are filtered before being visible to the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ResponseFilterMode>,
    /// Dot notation paths to include when mode is 'allow' (e.g., ['ticket.id', 'ticket.status']).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<String>>,
    /// Content type for response filtering. Only 'application/json' responses are filtered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ResponseFilterContentType>,
}

impl ResponseFilter {
    pub fn builder() -> ResponseFilterBuilder {
        <ResponseFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseFilterBuilder {
    mode: Option<ResponseFilterMode>,
    filters: Option<Vec<String>>,
    content_type: Option<ResponseFilterContentType>,
}

impl ResponseFilterBuilder {
    pub fn mode(mut self, value: ResponseFilterMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<String>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn content_type(mut self, value: ResponseFilterContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponseFilter`].
    pub fn build(self) -> Result<ResponseFilter, BuildError> {
        Ok(ResponseFilter {
            mode: self.mode,
            filters: self.filters,
            content_type: self.content_type,
        })
    }
}
