#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct TransferRulesClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl TransferRulesClient {
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

    /// Add Transfer Rule
    ///
    /// Add a phone forwarding rule to the named receptionist.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `body`
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create(
        &self,
        agent_id: &str,
        body: &ReceptionCreateTransferRuleRequest,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionTransferRule, crate::error::ApiError> {
        let url = format!(
            "/v1/reception/customer-facing-agents/{}/transfer-rules",
            agent_id
        );

        let mut request = self.http_client.post(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
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
        let config = self.config.get_transfer_rules_config();
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
        let result: ReceptionTransferRule =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Update Transfer Rule
    ///
    /// Replace an existing transfer rule on the named receptionist. Send every field: an omitted `post_dial_digits` clears it.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `rule_id` - ID of the transfer rule. Returned when you add one, and listed in the receptionist's `config.transfer_rules`.
    /// * `body`
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update(
        &self,
        agent_id: &str,
        rule_id: &str,
        body: &ReceptionUpdateTransferRuleRequest,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionTransferRule, crate::error::ApiError> {
        let url = format!(
            "/v1/reception/customer-facing-agents/{}/transfer-rules/{}",
            agent_id, rule_id
        );

        let mut request = self.http_client.put(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
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
        let config = self.config.get_transfer_rules_config();
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
        let result: ReceptionTransferRule =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Delete Transfer Rule
    ///
    /// Delete a transfer rule from the named receptionist.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `rule_id` - ID of the transfer rule. Returned when you add one, and listed in the receptionist's `config.transfer_rules`.
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        agent_id: &str,
        rule_id: &str,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<std::collections::HashMap<String, String>, crate::error::ApiError> {
        let url = format!(
            "/v1/reception/customer-facing-agents/{}/transfer-rules/{}",
            agent_id, rule_id
        );

        let mut request = self.http_client.delete(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
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
        let config = self.config.get_transfer_rules_config();
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
        let result: std::collections::HashMap<String, String> = if bytes.is_empty() {
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
