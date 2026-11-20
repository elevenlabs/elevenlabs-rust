#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct CallsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl CallsClient {
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

    /// List Outbound Campaign Calls
    ///
    /// List the calls of an outbound campaign, newest first: first contacts, retries, callbacks and redirects, whether queued, under way or done.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list(
        &self,
        campaign_id: &str,
        request: &CallsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignCallsResponse, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let lead_id = request.lead_id.clone();
        let status = request.status.clone();
        let call_result = request.call_result.clone();
        let stage = request.stage.clone();
        let cursor = request.cursor.clone();
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/calls",
            campaign_id
        );

        let mut request = self.http_client.get(url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &lead_id {
            request = request.query(&[("lead_id", &val.to_string())]);
        }
        if let Some(val) = &status {
            request = request.query(&[("status", &val.to_string())]);
        }
        if let Some(val) = &call_result {
            for item in val.iter() {
                request = request.query(&[("call_result", &item.to_string())]);
            }
        }
        if let Some(val) = &stage {
            request = request.query(&[("stage", &val.to_string())]);
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
        let config = self.config.get_calls_config();
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
        let result: CampaignCallsResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Export Outbound Campaign Calls
    ///
    /// Download every call of a campaign, with its lead and conversation results, as CSV.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Campaign calls CSV.
    pub async fn export(
        &self,
        campaign_id: &str,
        request: &CallsExportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<String, crate::error::ApiError> {
        let limit = request.limit.clone();
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/export",
            campaign_id
        );

        let mut request = self.http_client.get(url);
        if let Some(val) = &limit {
            request = request.query(&[("limit", &val.to_string())]);
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
        let config = self.config.get_calls_config();
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

        let result = response.text().await?;
        Ok(result)
    }
}
