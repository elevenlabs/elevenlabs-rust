#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct AuditLogsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl AuditLogsClient {
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

    /// Get Workspace Audit Logs
    ///
    /// Returns the audit log for the workspace. Requires enterprise tier and the audit_log_read permission.
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
        request: &AuditLogsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAuditLogsPageResponse, crate::error::ApiError> {
        let limit = request.limit.clone();
        let cursor = request.cursor.clone();
        let time_from_unix_ms = request.time_from_unix_ms.clone();
        let time_to_unix_ms = request.time_to_unix_ms.clone();
        let actor_uid = request.actor_uid.clone();
        let class_name = request.class_name.clone();
        let activity_name = request.activity_name.clone();
        let url = "/v1/workspace/audit-logs";

        let base_url = self
            .config
            .get_audit_logs_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &limit {
            request = request.query(&[("limit", &val.to_string())]);
        }
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
        if let Some(val) = &time_from_unix_ms {
            request = request.query(&[("time_from_unix_ms", &val.to_string())]);
        }
        if let Some(val) = &time_to_unix_ms {
            request = request.query(&[("time_to_unix_ms", &val.to_string())]);
        }
        if let Some(val) = &actor_uid {
            request = request.query(&[("actor_uid", &val.to_string())]);
        }
        if let Some(val) = &class_name {
            request = request.query(&[("class_name", &val.to_string())]);
        }
        if let Some(val) = &activity_name {
            request = request.query(&[("activity_name", &val.to_string())]);
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
        let config = self.config.get_audit_logs_config();
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
        let result: WorkspaceAuditLogsPageResponse =
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
