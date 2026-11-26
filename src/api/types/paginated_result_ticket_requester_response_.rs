pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaginatedResultTicketRequesterResponse_ {
    #[serde(default)]
    pub results: Vec<TicketRequesterResponse>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl PaginatedResultTicketRequesterResponse_ {
    pub fn builder() -> PaginatedResultTicketRequesterResponse_Builder {
        <PaginatedResultTicketRequesterResponse_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultTicketRequesterResponse_Builder {
    results: Option<Vec<TicketRequesterResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl PaginatedResultTicketRequesterResponse_Builder {
    pub fn results(mut self, value: Vec<TicketRequesterResponse>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResultTicketRequesterResponse_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](PaginatedResultTicketRequesterResponse_Builder::results)
    pub fn build(self) -> Result<PaginatedResultTicketRequesterResponse_, BuildError> {
        Ok(PaginatedResultTicketRequesterResponse_ {
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
