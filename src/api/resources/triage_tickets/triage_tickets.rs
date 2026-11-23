#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct TriageTicketsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl TriageTicketsClient {
    /// Builds a standalone client from a configuration of its own.
    ///
    /// A client reached through the root client shares that client's
    /// configuration, so a later `set_access_token` reaches it; one built here
    /// owns its configuration outright and is unaffected by the root client.
    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {
        let http_client = Arc::new(HttpClient::new(config.clone())?);
        let config_manager = Arc::new(ConfigManager::new(config));
        Ok(Self::with_shared(http_client, config_manager))
    }

    pub(crate) fn with_shared(http_client: Arc<HttpClient>, config: Arc<ConfigManager>) -> Self {
        Self {
            http_client,
            config,
        }
    }

    /// List Agent Conversation Tickets
    ///
    /// List an agent's issues, ordered by most recently created first unless sorted by priority. These are issues about the agent's own performance on a conversation (to work on with Architect), not tickets an agent opens for end users.
    ///
    /// # Arguments
    ///
    /// * `agent_id`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list(
        &self,
        agent_id: &str,
        request: &TriageTicketsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentConversationTicketsPageResponseModel, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let conversation_id = request.conversation_id.clone();
        let status = request.status.clone();
        let sources = request.sources.clone();
        let priorities = request.priorities.clone();
        let sort_by = request.sort_by.clone();
        let owner_user_id = request.owner_user_id.clone();
        let assignee_user_id = request.assignee_user_id.clone();
        let label = request.label.clone();
        let merged_into_ticket_id = request.merged_into_ticket_id.clone();
        let search = request.search.clone();
        let cursor = request.cursor.clone();
        let url = format!("/v1/convai/agents/{}/triage-tickets", agent_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &conversation_id {
            request = request.query(&[("conversation_id", &val.to_string())]);
        }
        if let Some(val) = &status {
            request = request.query(&[("status", &val.to_string())]);
        }
        if let Some(val) = &sources {
            for item in val.iter() {
                request = request.query(&[("sources", &item.to_string())]);
            }
        }
        if let Some(val) = &priorities {
            for item in val.iter() {
                request = request.query(&[("priorities", &item.to_string())]);
            }
        }
        if let Some(val) = &sort_by {
            request = request.query(&[("sort_by", &val.to_string())]);
        }
        if let Some(val) = &owner_user_id {
            request = request.query(&[("owner_user_id", &val.to_string())]);
        }
        if let Some(val) = &assignee_user_id {
            request = request.query(&[("assignee_user_id", &val.to_string())]);
        }
        if let Some(val) = &label {
            request = request.query(&[("label", &val.to_string())]);
        }
        if let Some(val) = &merged_into_ticket_id {
            request = request.query(&[("merged_into_ticket_id", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: GetAgentConversationTicketsPageResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Create Manual Agent Ticket
    ///
    /// Manually raise a follow-up issue against an agent, not tied to any conversation (for example a task like 'add the KB about X'). Without a title, one is derived from the comment. Requires viewer access to the agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id`
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create_manual(
        &self,
        agent_id: &str,
        body: &CreateManualTicketRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/agents/{}/triage-tickets", agent_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// List Workspace Conversation Tickets
    ///
    /// List agent issues across every agent in the workspace, ordered by most recently created first. Use this to build a workspace-wide view (for example, issues assigned to the caller). Supports the same filters, sorting and search as the per-agent endpoint. Issues for agents the caller cannot access are omitted.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list_for_workspace(
        &self,
        request: &ListForWorkspaceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentConversationTicketsPageResponseModel, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let agent_id = request.agent_id.clone();
        let status = request.status.clone();
        let sources = request.sources.clone();
        let priorities = request.priorities.clone();
        let sort_by = request.sort_by.clone();
        let owner_user_id = request.owner_user_id.clone();
        let assignee_user_id = request.assignee_user_id.clone();
        let label = request.label.clone();
        let merged_into_ticket_id = request.merged_into_ticket_id.clone();
        let search = request.search.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/triage-tickets";

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &status {
            request = request.query(&[("status", &val.to_string())]);
        }
        if let Some(val) = &sources {
            for item in val.iter() {
                request = request.query(&[("sources", &item.to_string())]);
            }
        }
        if let Some(val) = &priorities {
            for item in val.iter() {
                request = request.query(&[("priorities", &item.to_string())]);
            }
        }
        if let Some(val) = &sort_by {
            request = request.query(&[("sort_by", &val.to_string())]);
        }
        if let Some(val) = &owner_user_id {
            request = request.query(&[("owner_user_id", &val.to_string())]);
        }
        if let Some(val) = &assignee_user_id {
            request = request.query(&[("assignee_user_id", &val.to_string())]);
        }
        if let Some(val) = &label {
            request = request.query(&[("label", &val.to_string())]);
        }
        if let Some(val) = &merged_into_ticket_id {
            request = request.query(&[("merged_into_ticket_id", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: GetAgentConversationTicketsPageResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Create Agent Conversation Ticket
    ///
    /// Raise an issue about an agent's performance on a conversation, to work on with Architect. Provide an overall comment and/or turn-level comments describing what went wrong.
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create(
        &self,
        body: &CreateAgentConversationTicketRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = "/v1/convai/triage-tickets";

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Agent Conversation Ticket Assignable Users
    ///
    /// All non-service-account workspace members, each flagged with whether they currently have at least viewer access to the agent. Members without access are included (not filtered out) so the UI can offer them as an assignee and prompt to grant access first.
    ///
    /// # Arguments
    ///
    /// * `agent_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list_assignable_users(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AssignableUserResponseModel>, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/agents/{}/triage-tickets/assignable-users",
            agent_id
        );

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        let result: Vec<AssignableUserResponseModel> = if bytes.is_empty() {
            Default::default()
        } else {
            serde_json::from_slice(&bytes).map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?
        };
        Ok(result)
    }

    /// Get Agent Conversation Ticket
    ///
    /// Get an agent issue by ID.
    ///
    /// # Arguments
    ///
    /// * `agentqa_ticket_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        agentqa_ticket_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/triage-tickets/{}", agentqa_ticket_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Update Agent Conversation Ticket
    ///
    /// Update an agent issue's title, comment, status, priority, and/or assignee. Requires viewer access to the issue's agent.
    ///
    /// # Arguments
    ///
    /// * `agentqa_ticket_id`
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update(
        &self,
        agentqa_ticket_id: &str,
        body: &PatchAgentConversationTicketRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/triage-tickets/{}", agentqa_ticket_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::PATCH, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Delete Agent Conversation Ticket
    ///
    /// Delete an agent issue. Restricted to the issue creator or a workspace admin.
    ///
    /// # Arguments
    ///
    /// * `agentqa_ticket_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        agentqa_ticket_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), crate::error::ApiError> {
        let url = format!("/v1/convai/triage-tickets/{}", agentqa_ticket_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::DELETE, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let _ = response;
        Ok(())
    }

    /// Add Comment To Agent Conversation Ticket
    ///
    /// Append a comment discussing how to resolve the agent issue. Requires viewer access to the issue's agent.
    ///
    /// # Arguments
    ///
    /// * `agentqa_ticket_id`
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn add_comment(
        &self,
        agentqa_ticket_id: &str,
        body: &AddTicketCommentRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/triage-tickets/{}/comments", agentqa_ticket_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Merge Agent Conversation Tickets
    ///
    /// Merge other open issues of the same agent into this pending, in-progress, or resolved one. Their conversations move onto this issue, its priority is raised to the highest among them, and they are kept with status 'merged'. Requires viewer access to the issue's agent.
    ///
    /// # Arguments
    ///
    /// * `agentqa_ticket_id`
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn merge(
        &self,
        agentqa_ticket_id: &str,
        body: &MergeAgentConversationTicketsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/triage-tickets/{}/merge", agentqa_ticket_id);

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Add Turn Comment To Agent Conversation Ticket
    ///
    /// Append a turn-level comment to an agent issue. Requires viewer access to the issue's agent.
    ///
    /// # Arguments
    ///
    /// * `agentqa_ticket_id`
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn add_turn_comment(
        &self,
        agentqa_ticket_id: &str,
        body: &AddTurnCommentRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/triage-tickets/{}/turn-comments",
            agentqa_ticket_id
        );

        let base_url = self
            .config
            .get_triage_tickets_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_triage_tickets_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: AgentConversationTicketResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }
}
