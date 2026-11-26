pub use crate::prelude::*;

/// Query parameters for List Tickets
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TicketsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<TicketStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Vec<TicketPriority>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_status: Option<Vec<TicketSlaStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_status: Option<Vec<TicketDeadlineStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_user_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<Vec<String>>,
    /// Only return tickets with any of these tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unassigned: Option<bool>,
    /// The field to sort the results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<TicketSortBy>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

impl TicketsListQueryRequest {
    pub fn builder() -> TicketsListQueryRequestBuilder {
        <TicketsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketsListQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
    search: Option<String>,
    status: Option<Vec<TicketStatus>>,
    priority: Option<Vec<TicketPriority>>,
    sla_status: Option<Vec<TicketSlaStatus>>,
    deadline_status: Option<Vec<TicketDeadlineStatus>>,
    requester_user_id: Option<Vec<String>>,
    assignee_id: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
    unassigned: Option<bool>,
    sort_by: Option<TicketSortBy>,
    sort_direction: Option<SortDirection>,
}

impl TicketsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn status(mut self, value: Vec<TicketStatus>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn priority(mut self, value: Vec<TicketPriority>) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn sla_status(mut self, value: Vec<TicketSlaStatus>) -> Self {
        self.sla_status = Some(value);
        self
    }

    pub fn deadline_status(mut self, value: Vec<TicketDeadlineStatus>) -> Self {
        self.deadline_status = Some(value);
        self
    }

    pub fn requester_user_id(mut self, value: Vec<String>) -> Self {
        self.requester_user_id = Some(value);
        self
    }

    pub fn assignee_id(mut self, value: Vec<String>) -> Self {
        self.assignee_id = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn unassigned(mut self, value: bool) -> Self {
        self.unassigned = Some(value);
        self
    }

    pub fn sort_by(mut self, value: TicketSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketsListQueryRequest`].
    pub fn build(self) -> Result<TicketsListQueryRequest, BuildError> {
        Ok(TicketsListQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
            search: self.search,
            status: self.status,
            priority: self.priority,
            sla_status: self.sla_status,
            deadline_status: self.deadline_status,
            requester_user_id: self.requester_user_id,
            assignee_id: self.assignee_id,
            tag_ids: self.tag_ids,
            unassigned: self.unassigned,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
        })
    }
}
