pub use crate::prelude::*;

/// Query parameters for List Workspace Conversation Tickets
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListForWorkspaceQueryRequest {
    /// How many agent issues to return. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only issues for this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter agent issues by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentConversationTicketStatus>,
    /// Filter agent issues by how they were raised (qa, agent, manual). Repeat the parameter to filter by multiple sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<AgentConversationTicketSource>>,
    /// Filter agent issues by priority. Repeat the parameter to filter by multiple priorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<AgentConversationTicketPriority>>,
    /// Order by most recently created, by priority (most urgent first, then most recently created), by most recently reported (the newest report merged into the agent issue, else its creation), or by report count (most reports merged into the agent issue first, then most recently created).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<AgentConversationTicketSortBy>,
    /// Filter agent issues by creator. Use 'agent' for agent-raised issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    /// Filter agent issues by assignee. Use 'unassigned' for issues with no assignee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    /// Filter agent issues by an exact label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Filter agent issues merged into this issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_ticket_id: Option<String>,
    /// Case-insensitive free-text search across the agent issue's title, description, comments, and turn comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListForWorkspaceQueryRequest {
    pub fn builder() -> ListForWorkspaceQueryRequestBuilder {
        <ListForWorkspaceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForWorkspaceQueryRequestBuilder {
    page_size: Option<i64>,
    agent_id: Option<String>,
    status: Option<AgentConversationTicketStatus>,
    sources: Option<Vec<AgentConversationTicketSource>>,
    priorities: Option<Vec<AgentConversationTicketPriority>>,
    sort_by: Option<AgentConversationTicketSortBy>,
    owner_user_id: Option<String>,
    assignee_user_id: Option<String>,
    label: Option<String>,
    merged_into_ticket_id: Option<String>,
    search: Option<String>,
    cursor: Option<String>,
}

impl ListForWorkspaceQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: AgentConversationTicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<AgentConversationTicketSource>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn priorities(mut self, value: Vec<AgentConversationTicketPriority>) -> Self {
        self.priorities = Some(value);
        self
    }

    pub fn sort_by(mut self, value: AgentConversationTicketSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    pub fn assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_user_id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn merged_into_ticket_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_ticket_id = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListForWorkspaceQueryRequest`].
    pub fn build(self) -> Result<ListForWorkspaceQueryRequest, BuildError> {
        Ok(ListForWorkspaceQueryRequest {
            page_size: self.page_size,
            agent_id: self.agent_id,
            status: self.status,
            sources: self.sources,
            priorities: self.priorities,
            sort_by: self.sort_by,
            owner_user_id: self.owner_user_id,
            assignee_user_id: self.assignee_user_id,
            label: self.label,
            merged_into_ticket_id: self.merged_into_ticket_id,
            search: self.search,
            cursor: self.cursor,
        })
    }
}
