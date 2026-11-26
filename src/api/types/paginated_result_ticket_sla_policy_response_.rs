pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaginatedResultTicketSlaPolicyResponse_ {
    #[serde(default)]
    pub results: Vec<TicketSlaPolicyResponse>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl PaginatedResultTicketSlaPolicyResponse_ {
    pub fn builder() -> PaginatedResultTicketSlaPolicyResponse_Builder {
        <PaginatedResultTicketSlaPolicyResponse_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultTicketSlaPolicyResponse_Builder {
    results: Option<Vec<TicketSlaPolicyResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl PaginatedResultTicketSlaPolicyResponse_Builder {
    pub fn results(mut self, value: Vec<TicketSlaPolicyResponse>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedResultTicketSlaPolicyResponse_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](PaginatedResultTicketSlaPolicyResponse_Builder::results)
    pub fn build(self) -> Result<PaginatedResultTicketSlaPolicyResponse_, BuildError> {
        Ok(PaginatedResultTicketSlaPolicyResponse_ {
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
