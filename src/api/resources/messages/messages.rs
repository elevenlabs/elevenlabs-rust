#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct MessagesClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl MessagesClient {
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

    /// Text Search Conversation Messages
    ///
    /// Search through conversation transcript messages by full-text and fuzzy search
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn text_search(
        &self,
        request: &TextSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MessagesSearchResponse, crate::error::ApiError> {
        let text_query = request.text_query.clone();
        let agent_id = request.agent_id.clone();
        let visited_agent_ids = request.visited_agent_ids.clone();
        let visited_agent_branch_ids = request.visited_agent_branch_ids.clone();
        let triggered_procedure_ids = request.triggered_procedure_ids.clone();
        let call_successful = request.call_successful.clone();
        let call_start_before_unix = request.call_start_before_unix.clone();
        let call_start_after_unix = request.call_start_after_unix.clone();
        let call_duration_min_secs = request.call_duration_min_secs.clone();
        let call_duration_max_secs = request.call_duration_max_secs.clone();
        let rating_max = request.rating_max.clone();
        let rating_min = request.rating_min.clone();
        let has_feedback_comment = request.has_feedback_comment.clone();
        let user_id = request.user_id.clone();
        let evaluation_params = request.evaluation_params.clone();
        let data_collection_params = request.data_collection_params.clone();
        let dynamic_variable_params = request.dynamic_variable_params.clone();
        let tool_names = request.tool_names.clone();
        let tool_names_successful = request.tool_names_successful.clone();
        let tool_names_errored = request.tool_names_errored.clone();
        let tool_ids = request.tool_ids.clone();
        let tool_ids_successful = request.tool_ids_successful.clone();
        let tool_ids_errored = request.tool_ids_errored.clone();
        let include_invalid_tool_calls = request.include_invalid_tool_calls.clone();
        let main_languages = request.main_languages.clone();
        let phone_number_ids = request.phone_number_ids.clone();
        let whatsapp_phone_number_ids = request.whatsapp_phone_number_ids.clone();
        let exclude_statuses = request.exclude_statuses.clone();
        let termination_reasons = request.termination_reasons.clone();
        let page_size = request.page_size.clone();
        let summary_mode = request.summary_mode.clone();
        let conversation_initiation_source = request.conversation_initiation_source.clone();
        let text_only = request.text_only.clone();
        let conversation_product_type = request.conversation_product_type.clone();
        let branch_id = request.branch_id.clone();
        let version_id = request.version_id.clone();
        let topic_ids = request.topic_ids.clone();
        let sort_by = request.sort_by.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/conversations/messages/text-search";

        let base_url = self
            .config
            .get_messages_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        request = request.query(&[("text_query", &text_query.to_string())]);
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &visited_agent_ids {
            for item in val.iter() {
                request = request.query(&[("visited_agent_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &visited_agent_branch_ids {
            for item in val.iter() {
                request = request.query(&[("visited_agent_branch_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &triggered_procedure_ids {
            for item in val.iter() {
                request = request.query(&[("triggered_procedure_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &call_successful {
            request = request.query(&[("call_successful", &val.to_string())]);
        }
        if let Some(val) = &call_start_before_unix {
            request = request.query(&[("call_start_before_unix", &val.to_string())]);
        }
        if let Some(val) = &call_start_after_unix {
            request = request.query(&[("call_start_after_unix", &val.to_string())]);
        }
        if let Some(val) = &call_duration_min_secs {
            request = request.query(&[("call_duration_min_secs", &val.to_string())]);
        }
        if let Some(val) = &call_duration_max_secs {
            request = request.query(&[("call_duration_max_secs", &val.to_string())]);
        }
        if let Some(val) = &rating_max {
            request = request.query(&[("rating_max", &val.to_string())]);
        }
        if let Some(val) = &rating_min {
            request = request.query(&[("rating_min", &val.to_string())]);
        }
        if let Some(val) = &has_feedback_comment {
            request = request.query(&[("has_feedback_comment", &val.to_string())]);
        }
        if let Some(val) = &user_id {
            request = request.query(&[("user_id", &val.to_string())]);
        }
        if let Some(val) = &evaluation_params {
            for item in val.iter() {
                request = request.query(&[("evaluation_params", &item.to_string())]);
            }
        }
        if let Some(val) = &data_collection_params {
            for item in val.iter() {
                request = request.query(&[("data_collection_params", &item.to_string())]);
            }
        }
        if let Some(val) = &dynamic_variable_params {
            for item in val.iter() {
                request = request.query(&[("dynamic_variable_params", &item.to_string())]);
            }
        }
        if let Some(val) = &tool_names {
            for item in val.iter() {
                request = request.query(&[("tool_names", &item.to_string())]);
            }
        }
        if let Some(val) = &tool_names_successful {
            for item in val.iter() {
                request = request.query(&[("tool_names_successful", &item.to_string())]);
            }
        }
        if let Some(val) = &tool_names_errored {
            for item in val.iter() {
                request = request.query(&[("tool_names_errored", &item.to_string())]);
            }
        }
        if let Some(val) = &tool_ids {
            for item in val.iter() {
                request = request.query(&[("tool_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &tool_ids_successful {
            for item in val.iter() {
                request = request.query(&[("tool_ids_successful", &item.to_string())]);
            }
        }
        if let Some(val) = &tool_ids_errored {
            for item in val.iter() {
                request = request.query(&[("tool_ids_errored", &item.to_string())]);
            }
        }
        if let Some(val) = &include_invalid_tool_calls {
            request = request.query(&[("include_invalid_tool_calls", &val.to_string())]);
        }
        if let Some(val) = &main_languages {
            for item in val.iter() {
                request = request.query(&[("main_languages", &item.to_string())]);
            }
        }
        if let Some(val) = &phone_number_ids {
            for item in val.iter() {
                request = request.query(&[("phone_number_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &whatsapp_phone_number_ids {
            for item in val.iter() {
                request = request.query(&[("whatsapp_phone_number_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &exclude_statuses {
            for item in val.iter() {
                request = request.query(&[("exclude_statuses", &item.to_string())]);
            }
        }
        if let Some(val) = &termination_reasons {
            for item in val.iter() {
                request = request.query(&[("termination_reasons", &item.to_string())]);
            }
        }
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &summary_mode {
            request = request.query(&[("summary_mode", &val.to_string())]);
        }
        if let Some(val) = &conversation_initiation_source {
            request = request.query(&[("conversation_initiation_source", &val.to_string())]);
        }
        if let Some(val) = &text_only {
            request = request.query(&[("text_only", &val.to_string())]);
        }
        if let Some(val) = &conversation_product_type {
            request = request.query(&[("conversation_product_type", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &version_id {
            request = request.query(&[("version_id", &val.to_string())]);
        }
        if let Some(val) = &topic_ids {
            for item in val.iter() {
                request = request.query(&[("topic_ids", &item.to_string())]);
            }
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
        let config = self.config.get_messages_config();
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
        let result: MessagesSearchResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Smart Search Conversation Messages
    ///
    /// Search conversation transcripts by semantic similarity to surface relevant messages based on meaning and intent, rather than exact keyword matches
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn search(
        &self,
        request: &MessagesSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MessagesSearchResponse, crate::error::ApiError> {
        let text_query = request.text_query.clone();
        let agent_id = request.agent_id.clone();
        let branch_id = request.branch_id.clone();
        let page_size = request.page_size.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/conversations/messages/smart-search";

        let base_url = self
            .config
            .get_messages_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        request = request.query(&[("text_query", &text_query.to_string())]);
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
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
        let config = self.config.get_messages_config();
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
        let result: MessagesSearchResponse =
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
