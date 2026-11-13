#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct ConversationsClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl ConversationsClient {
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

    /// Get Signed Url
    ///
    /// Get a signed url to start a conversation with an agent with an agent that requires authorization
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_signed_url(
        &self,
        request: &GetSignedUrlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationSignedUrlResponseModel, crate::error::ApiError> {
        let agent_id = request.agent_id.clone();
        let include_conversation_id = request.include_conversation_id.clone();
        let branch_id = request.branch_id.clone();
        let version_id = request.version_id.clone();
        let environment = request.environment.clone();
        let debug_events_request = request.debug_events_request.clone();
        let url = "/v1/convai/conversation/get-signed-url";

        let mut request = self.http_client.get(url);
        request = request.query(&[("agent_id", &agent_id.to_string())]);
        if let Some(val) = &include_conversation_id {
            request = request.query(&[("include_conversation_id", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &version_id {
            request = request.query(&[("version_id", &val.to_string())]);
        }
        if let Some(val) = &environment {
            request = request.query(&[("environment", &val.to_string())]);
        }
        if let Some(val) = &debug_events_request {
            request = request.query(&[("debug_events_request", &val.to_string())]);
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
        let config = self.config.get_conversations_config();
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
        let result: ConversationSignedUrlResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get a webrtc token to start a conversation with an agent that requires authorization
    ///
    /// Get a WebRTC session token for real-time communication.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_webrtc_token(
        &self,
        request: &GetWebrtcTokenQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TokenResponseModel, crate::error::ApiError> {
        let agent_id = request.agent_id.clone();
        let participant_name = request.participant_name.clone();
        let branch_id = request.branch_id.clone();
        let version_id = request.version_id.clone();
        let environment = request.environment.clone();
        let debug_events_request = request.debug_events_request.clone();
        let url = "/v1/convai/conversation/token";

        let mut request = self.http_client.get(url);
        request = request.query(&[("agent_id", &agent_id.to_string())]);
        if let Some(val) = &participant_name {
            request = request.query(&[("participant_name", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &version_id {
            request = request.query(&[("version_id", &val.to_string())]);
        }
        if let Some(val) = &environment {
            request = request.query(&[("environment", &val.to_string())]);
        }
        if let Some(val) = &debug_events_request {
            request = request.query(&[("debug_events_request", &val.to_string())]);
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
        let config = self.config.get_conversations_config();
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
        let result: TokenResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// List conversations
    ///
    /// Get all conversations of agents that user owns. With option to restrict to a specific agent.
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
        request: &ConversationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationsPageResponseModel, crate::error::ApiError> {
        let cursor = request.cursor.clone();
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
        let data_collection_filter = request.data_collection_filter.clone();
        let dynamic_variable_filter = request.dynamic_variable_filter.clone();
        let data_collection_ids = request.data_collection_ids.clone();
        let evaluation_criteria_ids = request.evaluation_criteria_ids.clone();
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
        let page_size = request.page_size.clone();
        let summary_mode = request.summary_mode.clone();
        let search = request.search.clone();
        let conversation_initiation_source = request.conversation_initiation_source.clone();
        let text_only = request.text_only.clone();
        let conversation_product_type = request.conversation_product_type.clone();
        let branch_id = request.branch_id.clone();
        let version_id = request.version_id.clone();
        let parent_conversation_id = request.parent_conversation_id.clone();
        let topic_ids = request.topic_ids.clone();
        let exclude_statuses = request.exclude_statuses.clone();
        let tag_ids = request.tag_ids.clone();
        let workflow_node_entered_id = request.workflow_node_entered_id.clone();
        let termination_reasons = request.termination_reasons.clone();
        let guardrail_types = request.guardrail_types.clone();
        let custom_guardrail_names = request.custom_guardrail_names.clone();
        let sort_direction = request.sort_direction.clone();
        let url = "/v1/convai/conversations";

        let mut request = self.http_client.get(url);
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
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
        if let Some(val) = &data_collection_filter {
            request = request.query(&[("data_collection_filter", &val.to_string())]);
        }
        if let Some(val) = &dynamic_variable_filter {
            request = request.query(&[("dynamic_variable_filter", &val.to_string())]);
        }
        if let Some(val) = &data_collection_ids {
            for item in val.iter() {
                request = request.query(&[("data_collection_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &evaluation_criteria_ids {
            for item in val.iter() {
                request = request.query(&[("evaluation_criteria_ids", &item.to_string())]);
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
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &summary_mode {
            request = request.query(&[("summary_mode", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
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
        if let Some(val) = &parent_conversation_id {
            request = request.query(&[("parent_conversation_id", &val.to_string())]);
        }
        if let Some(val) = &topic_ids {
            for item in val.iter() {
                request = request.query(&[("topic_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &exclude_statuses {
            for item in val.iter() {
                request = request.query(&[("exclude_statuses", &item.to_string())]);
            }
        }
        if let Some(val) = &tag_ids {
            for item in val.iter() {
                request = request.query(&[("tag_ids", &item.to_string())]);
            }
        }
        if let Some(val) = &workflow_node_entered_id {
            request = request.query(&[("workflow_node_entered_id", &val.to_string())]);
        }
        if let Some(val) = &termination_reasons {
            for item in val.iter() {
                request = request.query(&[("termination_reasons", &item.to_string())]);
            }
        }
        if let Some(val) = &guardrail_types {
            for item in val.iter() {
                request = request.query(&[("guardrail_types", &item.to_string())]);
            }
        }
        if let Some(val) = &custom_guardrail_names {
            for item in val.iter() {
                request = request.query(&[("custom_guardrail_names", &item.to_string())]);
            }
        }
        if let Some(val) = &sort_direction {
            request = request.query(&[("sort_direction", &val.to_string())]);
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
        let config = self.config.get_conversations_config();
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
        let result: GetConversationsPageResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Resolve Conversation Reference
    ///
    /// Resolve a conversation URL (a Slack message URL or a Zendesk ticket URL) to the deterministic conversation ID for the given agent, then confirm the conversation exists.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn resolve(
        &self,
        request: &ResolveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponseModel, crate::error::ApiError> {
        let agent_id = request.agent_id.clone();
        let reference = request.reference.clone();
        let url = "/v1/convai/conversations/resolve";

        let mut request = self.http_client.get(url);
        request = request.query(&[("agent_id", &agent_id.to_string())]);
        request = request.query(&[("reference", &reference.to_string())]);
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
        let config = self.config.get_conversations_config();
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
        let result: GetConversationResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Conversation Details
    ///
    /// Get the details of a particular conversation
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        conversation_id: &str,
        request: &ConversationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponseModel, crate::error::ApiError> {
        let format = request.format.clone();
        let url = format!("/v1/convai/conversations/{}", conversation_id);

        let mut request = self.http_client.get(url);
        if let Some(val) = &format {
            request = request.query(&[("format", &val.to_string())]);
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
        let config = self.config.get_conversations_config();
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
        let result: GetConversationResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Delete Conversation
    ///
    /// Delete a particular conversation
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, crate::error::ApiError> {
        let url = format!("/v1/convai/conversations/{}", conversation_id);

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
        let config = self.config.get_conversations_config();
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

    /// Get Conversation Summary
    ///
    /// Get a lightweight summary of a conversation: its title, the generated transcript summary, whether the call was successful, and -- only when the conversation is short -- the plain chat messages. Tool calls, tool results, and contextual updates are omitted so the response stays small. Use this instead of the full conversation endpoint when you only need the gist (e.g. an agent reading many conversations); use GET /v1/convai/conversations/{conversation_id} when you need the full transcript with tool calls and contextual updates.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_summary(
        &self,
        conversation_id: &str,
        request: &GetSummaryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationSummaryResponseModel, crate::error::ApiError> {
        let max_messages = request.max_messages.clone();
        let url = format!("/v1/convai/conversations/{}/summary", conversation_id);

        let mut request = self.http_client.get(url);
        if let Some(val) = &max_messages {
            request = request.query(&[("max_messages", &val.to_string())]);
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
        let config = self.config.get_conversations_config();
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
        let result: GetConversationSummaryResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Sip Messages For A Conversation
    ///
    /// Get SIP messages associated with a conversation's phone call
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_sip_messages(
        &self,
        conversation_id: &str,
        request: &GetSipMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSipLogMessagesResponse, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let cursor = request.cursor.clone();
        let url = format!("/v1/convai/conversations/{}/sip-messages", conversation_id);

        let mut request = self.http_client.get(url);
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
        let config = self.config.get_conversations_config();
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
        let result: GetSipLogMessagesResponse =
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
