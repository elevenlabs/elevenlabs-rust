#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct OutboundCampaignsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl OutboundCampaignsClient {
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

    /// List Outbound Campaigns
    ///
    /// List the workspace's outbound campaigns, newest first.
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
        request: &OutboundCampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignPageResponse, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let status = request.status.clone();
        let agent_id = request.agent_id.clone();
        let channel = request.channel.clone();
        let mode = request.mode.clone();
        let search = request.search.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/outbound-campaigns/campaigns";

        let base_url = self
            .config
            .get_outbound_campaigns_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &status {
            for item in val.iter() {
                request = request.query(&[("status", &item.to_string())]);
            }
        }
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &channel {
            request = request.query(&[("channel", &val.to_string())]);
        }
        if let Some(val) = &mode {
            request = request.query(&[("mode", &val.to_string())]);
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignPageResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Pause All Outbound Campaigns
    ///
    /// Pause every running outbound campaign in the workspace. Paused campaigns place no new calls until resumed.
    ///
    /// # Arguments
    ///
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn pause_all(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<PauseAllCampaignsResponse, crate::error::ApiError> {
        let url = "/v1/convai/outbound-campaigns/campaigns/pause-all";

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: PauseAllCampaignsResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Stop All Outbound Campaigns
    ///
    /// Stop every outbound campaign in the workspace that hasn't finished. Returns `202` while calls already placed are still finishing.
    ///
    /// # Arguments
    ///
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn stop_all(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<StopAllCampaignsResponse, crate::error::ApiError> {
        let url = "/v1/convai/outbound-campaigns/campaigns/stop-all";

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: StopAllCampaignsResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Create An Outbound Campaign
    ///
    /// Create an outbound campaign that calls or messages each recipient with the agent, within the policy's calling windows, retrying and following up as it allows. Returns `201` for a new campaign. A repeated request, matched by `idempotency_key` or else by its body, returns the existing campaign with `created` false.
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
        body: &SubmitCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmitCampaignResponse, crate::error::ApiError> {
        let url = "/v1/convai/outbound-campaigns/campaigns/submit";

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: SubmitCampaignResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get An Outbound Campaign
    ///
    /// Get an outbound campaign and its lead counts.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, crate::error::ApiError> {
        let url = format!("/v1/convai/outbound-campaigns/campaigns/{}", campaign_id);

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Outbound Campaign Progress
    ///
    /// Get where an outbound campaign's leads and calls stand, and when its next call is due.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_progress(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignProgressResponse, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/progress",
            campaign_id
        );

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignProgressResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Outbound Campaign Follow-Up Requests
    ///
    /// Count the callbacks and redirects contacts asked for during an outbound campaign's calls.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_follow_up_requests(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignFollowUpRequestsResponse, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/follow-up-requests",
            campaign_id
        );

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignFollowUpRequestsResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Pause An Outbound Campaign
    ///
    /// Pause an outbound campaign. It places no new calls until resumed; calls already placed carry on.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn pause(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/pause",
            campaign_id
        );

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Resume An Outbound Campaign
    ///
    /// Resume a paused outbound campaign, including calls an attention item held back.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn resume(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/resume",
            campaign_id
        );

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Stop An Outbound Campaign
    ///
    /// Stop an outbound campaign for good: queued calls are cancelled and no lead is called again. Returns `202` with the campaign `stopping` while calls already placed are still finishing.
    ///
    /// # Arguments
    ///
    /// * `campaign_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn stop(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, crate::error::ApiError> {
        let url = format!(
            "/v1/convai/outbound-campaigns/campaigns/{}/stop",
            campaign_id
        );

        let base_url = self
            .config
            .get_outbound_campaigns_config()
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
        let config = self.config.get_outbound_campaigns_config();
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
        let result: CampaignResponse =
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
