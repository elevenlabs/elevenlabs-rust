#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct WhatsappAccountsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl WhatsappAccountsClient {
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

    /// Get Whatsapp Account
    ///
    /// Get a WhatsApp account
    ///
    /// # Arguments
    ///
    /// * `phone_number_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsAppAccountResponse, crate::error::ApiError> {
        let url = format!("/v1/convai/whatsapp-accounts/{}", phone_number_id);

        let mut request = self.http_client.get(url);
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
        let config = self.config.get_whatsapp_accounts_config();
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
        let result: GetWhatsAppAccountResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Update Whatsapp Account
    ///
    /// Update a WhatsApp account
    ///
    /// # Arguments
    ///
    /// * `phone_number_id`
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update(
        &self,
        phone_number_id: &str,
        body: &UpdateWhatsAppAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, crate::error::ApiError> {
        let url = format!("/v1/convai/whatsapp-accounts/{}", phone_number_id);

        let mut request = self.http_client.patch(url);
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
        let config = self.config.get_whatsapp_accounts_config();
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

    /// Delete Whatsapp Account
    ///
    /// Delete a WhatsApp account
    ///
    /// # Arguments
    ///
    /// * `phone_number_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, crate::error::ApiError> {
        let url = format!("/v1/convai/whatsapp-accounts/{}", phone_number_id);

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
        let config = self.config.get_whatsapp_accounts_config();
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

    /// List Whatsapp Accounts
    ///
    /// List all WhatsApp accounts
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
        request: &WhatsappAccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWhatsAppAccountsResponse, crate::error::ApiError> {
        let agent_id = request.agent_id.clone();
        let url = "/v1/convai/whatsapp-accounts";

        let mut request = self.http_client.get(url);
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
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
        let config = self.config.get_whatsapp_accounts_config();
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
        let result: ListWhatsAppAccountsResponse =
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
