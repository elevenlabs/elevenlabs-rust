#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct ProceduresClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl ProceduresClient {
    /// Builds a standalone client from a configuration of its own.
    ///
    /// A client reached through the root client shares that client's
    /// configuration, so a later `set_access_token` reaches it; one built here
    /// owns its configuration outright and is unaffected by the root client.
    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {
        let http_client = Arc::new(
            HttpClient::new(
                config.base_url.clone(),
                config.timeout.as_millis() as u64,
                config.max_retries,
            )
            .map_err(crate::error::ApiError::Network)?,
        );
        let config_manager = Arc::new(ConfigManager::new(config));
        Ok(Self::with_shared(http_client, config_manager))
    }

    pub(crate) fn with_shared(http_client: Arc<HttpClient>, config: Arc<ConfigManager>) -> Self {
        Self {
            http_client,
            config,
        }
    }

    /// List Procedures
    ///
    /// List the procedures attached to this agent branch. By default, unpublished drafts take precedence over the latest committed version. Pass agent_version_id to list a published snapshot instead. has_draft is true when a procedure has unpublished draft changes on this branch. Procedure content is not included; use Get Procedure to read a procedure's body.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list(
        &self,
        agent_id: &str,
        branch_id: &str,
        request: &ProceduresListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProceduresResponseModel, crate::error::ApiError> {
        let agent_version_id = request.agent_version_id.clone();
        let url = format!(
            "/v1/convai/agents/{}/branches/{}/procedures",
            agent_id, branch_id
        );

        let mut request = self.http_client.get(url);
        if let Some(val) = &agent_version_id {
            request = request.query(&[("agent_version_id", &val.to_string())]);
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
        let config = self.config.get_procedures_config();
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
        let result: ListProceduresResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Create Procedure
    ///
    /// Create a new procedure for the agent on a branch.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create(
        &self,
        agent_id: &str,
        branch_id: &str,
        body: Option<&Option<CreateProcedureRequestModel>>,
        options: Option<RequestOptions>,
    ) -> Result<CreateProcedureResponseModel, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/agents/{}/branches/{}/procedures",
            agent_id, branch_id
        );

        let mut request = self.http_client.post(url);
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
        let config = self.config.get_procedures_config();
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
        let result: CreateProcedureResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Legacy: Compile Procedures
    ///
    /// Legacy. Do not use. Saving an agent draft (`POST /v1/convai/agents/{agent_id}/drafts`) and publishing an agent (`PATCH /v1/convai/agents/{agent_id}`) compile structured procedures into workflow nodes and edges, save the compiled workflow with the draft or version, and return validation errors, so a separate compile call is no longer needed. This endpoint remains available for the time being so existing callers do not break, as a dry-run that compiles the current procedure drafts into a workflow without persisting anything. It will eventually be deprecated.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn compile(
        &self,
        agent_id: &str,
        branch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CompileProceduresResponseModel, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/agents/{}/branches/{}/procedures/compile",
            agent_id, branch_id
        );

        let mut request = self.http_client.post(url);
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
        let config = self.config.get_procedures_config();
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
        let result: CompileProceduresResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Procedure
    ///
    /// Retrieve a procedure at a specific version or the current branch HEAD.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `procedure_id` - The procedure ID
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        agent_id: &str,
        branch_id: &str,
        procedure_id: &str,
        request: &ProceduresGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProcedureAtVersionResponseModel, crate::error::ApiError> {
        let version_id = request.version_id.clone();
        let agent_version_id = request.agent_version_id.clone();
        let url = format!(
            "/v1/convai/agents/{}/branches/{}/procedures/{}",
            agent_id, branch_id, procedure_id
        );

        let mut request = self.http_client.get(url);
        if let Some(val) = &version_id {
            request = request.query(&[("version_id", &val.to_string())]);
        }
        if let Some(val) = &agent_version_id {
            request = request.query(&[("agent_version_id", &val.to_string())]);
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
        let config = self.config.get_procedures_config();
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
        let result: ProcedureAtVersionResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Remove Procedure
    ///
    /// Remove a procedure from the agent's draft working set. Removing a folder cascades to its entire subtree, rejected if any procedure outside the subtree hands off into it.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `procedure_id` - The procedure ID
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn remove(
        &self,
        agent_id: &str,
        branch_id: &str,
        procedure_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/agents/{}/branches/{}/procedures/{}",
            agent_id, branch_id, procedure_id
        );

        let mut request = self.http_client.delete(url);
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
        let config = self.config.get_procedures_config();
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
        let result: serde_json::Value = if bytes.is_empty() {
            Default::default()
        } else {
            serde_json::from_slice(&bytes).map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?
        };
        Ok(result)
    }
}
