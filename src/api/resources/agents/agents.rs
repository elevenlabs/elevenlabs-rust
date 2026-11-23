#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct AgentsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl AgentsClient {
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

    /// Create agent
    ///
    /// Create an agent from a config object
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentResponseModel, crate::error::ApiError> {
        let enable_versioning = request.enable_versioning.clone();
        let body = request.body.clone();
        let url = "/v1/convai/agents/create";

        let base_url = self
            .config
            .get_agents_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(val) = &enable_versioning {
            request = request.query(&[("enable_versioning", &val.to_string())]);
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
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_agents_config();
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
        let result: CreateAgentResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get agent
    ///
    /// Retrieve config for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        agent_id: &str,
        request: &AgentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentResponseModel, crate::error::ApiError> {
        let version_id = request.version_id.clone();
        let branch_id = request.branch_id.clone();
        let include_procedure_generated_workflow_elements = request
            .include_procedure_generated_workflow_elements
            .clone();
        let url = format!("/v1/convai/agents/{}", agent_id);

        let base_url = self
            .config
            .get_agents_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &version_id {
            request = request.query(&[("version_id", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &include_procedure_generated_workflow_elements {
            request = request.query(&[(
                "include_procedure_generated_workflow_elements",
                &val.to_string(),
            )]);
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
        let config = self.config.get_agents_config();
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
        let result: GetAgentResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Update agent
    ///
    /// Patches an Agent settings
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update(
        &self,
        agent_id: &str,
        request: &UpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentResponseModel, crate::error::ApiError> {
        let enable_versioning_if_not_enabled = request.enable_versioning_if_not_enabled.clone();
        let branch_id = request.branch_id.clone();
        let include_procedure_generated_workflow_elements = request
            .include_procedure_generated_workflow_elements
            .clone();
        let body = request.body.clone();
        let url = format!("/v1/convai/agents/{}", agent_id);

        let base_url = self
            .config
            .get_agents_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::PATCH, &base_url, url);
        if let Some(val) = &enable_versioning_if_not_enabled {
            request = request.query(&[("enable_versioning_if_not_enabled", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &include_procedure_generated_workflow_elements {
            request = request.query(&[(
                "include_procedure_generated_workflow_elements",
                &val.to_string(),
            )]);
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
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_agents_config();
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
        let result: GetAgentResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Delete agent
    ///
    /// Delete an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        agent_id: &str,
        request: &DeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), crate::error::ApiError> {
        let force = request.force.clone();
        let url = format!("/v1/convai/agents/{}", agent_id);

        let base_url = self
            .config
            .get_agents_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::DELETE, &base_url, url);
        if let Some(val) = &force {
            request = request.query(&[("force", &val.to_string())]);
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
        let config = self.config.get_agents_config();
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

    /// List Agents
    ///
    /// Returns a list of your agents and their metadata.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list(
        &self,
        request: &AgentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentsPageResponseModel, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let search = request.search.clone();
        let archived = request.archived.clone();
        let show_only_owned_agents = request.show_only_owned_agents.clone();
        let created_by_user_id = request.created_by_user_id.clone();
        let tags = request.tags.clone();
        let sort_direction = request.sort_direction.clone();
        let sort_by = request.sort_by.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/agents";

        let base_url = self
            .config
            .get_agents_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &archived {
            request = request.query(&[("archived", &val.to_string())]);
        }
        if let Some(val) = &show_only_owned_agents {
            request = request.query(&[("show_only_owned_agents", &val.to_string())]);
        }
        if let Some(val) = &created_by_user_id {
            request = request.query(&[("created_by_user_id", &val.to_string())]);
        }
        if let Some(val) = &tags {
            for item in val.iter() {
                request = request.query(&[("tags", &item.to_string())]);
            }
        }
        if let Some(val) = &sort_direction {
            request = request.query(&[("sort_direction", &val.to_string())]);
        }
        if let Some(val) = &sort_by {
            request = request.query(&[("sort_by", &val.to_string())]);
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
        let config = self.config.get_agents_config();
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
        let result: GetAgentsPageResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Duplicate Agent
    ///
    /// Create a new agent by duplicating an existing one. The new agent gets a copy of every unarchived branch of the source agent, each with the same name and description and starting from the latest version on that branch. The live traffic split is copied onto the new branches, with any share held by a branch that was not copied moved to main. Version history, drafts, archived branches, past traffic splits and experiments are not copied.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn duplicate(
        &self,
        agent_id: &str,
        body: Option<&BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost>,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/agents/{}/duplicate", agent_id);

        let base_url = self
            .config
            .get_agents_config()
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
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_agents_config();
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
        let result: CreateAgentResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Run Tests On The Agent
    ///
    /// Run selected tests on the agent with provided configuration. If the agent configuration is provided, it will be used to override default agent configuration.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn run_tests(
        &self,
        agent_id: &str,
        body: &RunAgentTestsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<GetTestSuiteInvocationResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/agents/{}/run-tests", agent_id);

        let base_url = self
            .config
            .get_agents_config()
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
        let config = self.config.get_agents_config();
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
        let result: GetTestSuiteInvocationResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Rag Index Overview.
    ///
    /// Provides total size and other information of RAG indexes used by knowledgebase documents
    ///
    /// # Arguments
    ///
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn rag_index_overview(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<RagIndexOverviewResponseModel, crate::error::ApiError> {
        let url = "/v1/convai/knowledge-base/rag-index";

        let base_url = self
            .config
            .get_agents_config()
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
        let config = self.config.get_agents_config();
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
        let result: RagIndexOverviewResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Rag Indexes Of The Specified Knowledgebase Document.
    ///
    /// Provides information about all RAG indexes of the specified knowledgebase document.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_document_rag_indexes(
        &self,
        documentation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RagDocumentIndexesResponseModel, crate::error::ApiError> {
        let url = format!("/v1/convai/knowledge-base/{}/rag-index", documentation_id);

        let base_url = self
            .config
            .get_agents_config()
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
        let config = self.config.get_agents_config();
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
        let result: RagDocumentIndexesResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Delete Rag Index.
    ///
    /// Delete RAG index for the knowledgebase document.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `rag_index_id` - The id of RAG index of document from the knowledge base.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete_document_rag_index(
        &self,
        documentation_id: &str,
        rag_index_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RagDocumentIndexResponseModel, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/knowledge-base/{}/rag-index/{}",
            documentation_id, rag_index_id
        );

        let base_url = self
            .config
            .get_agents_config()
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
        let config = self.config.get_agents_config();
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
        let result: RagDocumentIndexResponseModel =
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
