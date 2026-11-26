pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginatedResultTicketListItemResponse_ {
    #[serde(default)]
    pub results: Vec<TicketListItemResponse>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl PaginatedResultTicketListItemResponse_ {
    pub fn builder() -> PaginatedResultTicketListItemResponse_Builder {
        <PaginatedResultTicketListItemResponse_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultTicketListItemResponse_Builder {
    results: Option<Vec<TicketListItemResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl PaginatedResultTicketListItemResponse_Builder {
    pub fn results(mut self, value: Vec<TicketListItemResponse>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedResultTicketListItemResponse_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](PaginatedResultTicketListItemResponse_Builder::results)
    pub fn build(self) -> Result<PaginatedResultTicketListItemResponse_, BuildError> {
        Ok(PaginatedResultTicketListItemResponse_ {
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
