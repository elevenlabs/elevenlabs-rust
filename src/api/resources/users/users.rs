#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct UsersClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl UsersClient {
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

    /// Get Conversation Users
    ///
    /// Get distinct users from conversations with pagination.
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
        request: &UsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationUsersPageResponseModel, crate::error::ApiError> {
        let agent_id = request.agent_id.clone();
        let branch_id = request.branch_id.clone();
        let call_start_before_unix = request.call_start_before_unix.clone();
        let call_start_after_unix = request.call_start_after_unix.clone();
        let search = request.search.clone();
        let page_size = request.page_size.clone();
        let sort_by = request.sort_by.clone();
        let sort_direction = request.sort_direction.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/users";

        let mut request = self.http_client.get(url);
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &call_start_before_unix {
            request = request.query(&[("call_start_before_unix", &val.to_string())]);
        }
        if let Some(val) = &call_start_after_unix {
            request = request.query(&[("call_start_after_unix", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &sort_by {
            request = request.query(&[("sort_by", &val.to_string())]);
        }
        if let Some(val) = &sort_direction {
            request = request.query(&[("sort_direction", &val.to_string())]);
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
        let config = self.config.get_users_config();
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
        let result: GetConversationUsersPageResponseModel =
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
