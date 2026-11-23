#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct VoicesClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl VoicesClient {
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

    /// Get voice
    ///
    /// Returns metadata about a specific voice.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        voice_id: &str,
        request: &VoicesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Voice, crate::error::ApiError> {
        let with_settings = request.with_settings.clone();
        let url = format!("/v1/voices/{}", voice_id);

        let base_url = self
            .config
            .get_voices_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &with_settings {
            request = request.query(&[("with_settings", &val.to_string())]);
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
        let config = self.config.get_voices_config();
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
        let result: Voice =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Delete voice
    ///
    /// Deletes a voice by its ID.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteVoiceResponseModel, crate::error::ApiError> {
        let url = format!("/v1/voices/{}", voice_id);

        let base_url = self
            .config
            .get_voices_config()
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
        let config = self.config.get_voices_config();
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
        let result: DeleteVoiceResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Edit voice
    ///
    /// Edit a voice created by you.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update(
        &self,
        voice_id: &str,
        body: &EditVoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<EditVoiceResponseModel, crate::error::ApiError> {
        let url = format!("/v1/voices/{}/edit", voice_id);

        let base_url = self
            .config
            .get_voices_config()
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
        request = request.multipart(body.clone().to_multipart());
        let config = self.config.get_voices_config();
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
        let result: EditVoiceResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// List voices
    ///
    /// Gets a list of all available voices for a user with search, filtering and pagination.
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
        request: &SearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetVoicesV2Response, crate::error::ApiError> {
        let next_page_token = request.next_page_token.clone();
        let page_size = request.page_size.clone();
        let search = request.search.clone();
        let sort = request.sort.clone();
        let sort_direction = request.sort_direction.clone();
        let voice_type = request.voice_type.clone();
        let category = request.category.clone();
        let fine_tuning_state = request.fine_tuning_state.clone();
        let collection_id = request.collection_id.clone();
        let gender = request.gender.clone();
        let age = request.age.clone();
        let language = request.language.clone();
        let accent = request.accent.clone();
        let use_cases = request.use_cases.clone();
        let min_notice_period_days = request.min_notice_period_days.clone();
        let include_custom_rates = request.include_custom_rates.clone();
        let include_live_moderated = request.include_live_moderated.clone();
        let high_quality = request.high_quality.clone();
        let include_total_count = request.include_total_count.clone();
        let voice_ids = request.voice_ids.clone();
        let url = "/v2/voices";

        let base_url = self
            .config
            .get_voices_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &next_page_token {
            request = request.query(&[("next_page_token", &val.to_string())]);
        }
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &sort {
            request = request.query(&[("sort", &val.to_string())]);
        }
        if let Some(val) = &sort_direction {
            request = request.query(&[("sort_direction", &val.to_string())]);
        }
        if let Some(val) = &voice_type {
            request = request.query(&[("voice_type", &val.to_string())]);
        }
        if let Some(val) = &category {
            request = request.query(&[("category", &val.to_string())]);
        }
        if let Some(val) = &fine_tuning_state {
            request = request.query(&[("fine_tuning_state", &val.to_string())]);
        }
        if let Some(val) = &collection_id {
            request = request.query(&[("collection_id", &val.to_string())]);
        }
        if let Some(val) = &gender {
            request = request.query(&[("gender", &val.to_string())]);
        }
        if let Some(val) = &age {
            request = request.query(&[("age", &val.to_string())]);
        }
        if let Some(val) = &language {
            for item in val.iter() {
                request = request.query(&[("language", &item.to_string())]);
            }
        }
        if let Some(val) = &accent {
            request = request.query(&[("accent", &val.to_string())]);
        }
        if let Some(val) = &use_cases {
            for item in val.iter() {
                request = request.query(&[("use_cases", &item.to_string())]);
            }
        }
        if let Some(val) = &min_notice_period_days {
            request = request.query(&[("min_notice_period_days", &val.to_string())]);
        }
        if let Some(val) = &include_custom_rates {
            request = request.query(&[("include_custom_rates", &val.to_string())]);
        }
        if let Some(val) = &include_live_moderated {
            request = request.query(&[("include_live_moderated", &val.to_string())]);
        }
        if let Some(val) = &high_quality {
            request = request.query(&[("high_quality", &val.to_string())]);
        }
        if let Some(val) = &include_total_count {
            request = request.query(&[("include_total_count", &val.to_string())]);
        }
        if let Some(val) = &voice_ids {
            for item in val.iter() {
                request = request.query(&[("voice_ids", &item.to_string())]);
            }
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
        let config = self.config.get_voices_config();
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
        let result: GetVoicesV2Response =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Replicate Voice To Isolated Environment
    ///
    /// Replicates an Instant Voice Clone or Voice Design voice to a workspace in a different data residency. The target workspace must belong to the same consolidated billing group. The user must have VOICES_WRITE in the source workspace, and be an admin on the source voice. Human users (i.e. not service accounts) must also have VOICES_WRITE in the target workspace. This endpoint is available on the central environment only.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn replicate_to_isolated_environment(
        &self,
        voice_id: &str,
        body: &ReplicateVoiceToIsolatedEnvironmentRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<ReplicateVoiceToIsolatedEnvironmentResponseModel, crate::error::ApiError> {
        let url = format!("/v1/voices/{}/replicate-to-isolated-environment", voice_id);

        let base_url = self
            .config
            .get_voices_config()
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
        let config = self.config.get_voices_config();
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
        let result: ReplicateVoiceToIsolatedEnvironmentResponseModel = response
            .json()
            .await
            .map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?;
        Ok(result)
    }

    /// Add shared voice
    ///
    /// Add a shared voice to your collection of Voices
    ///
    /// # Arguments
    ///
    /// * `public_user_id` - Public user ID used to publicly identify ElevenLabs users.
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn share(
        &self,
        public_user_id: &str,
        voice_id: &str,
        body: &BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceResponseModel, crate::error::ApiError> {
        let url = format!("/v1/voices/add/{}/{}", public_user_id, voice_id);

        let base_url = self
            .config
            .get_voices_config()
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
        let config = self.config.get_voices_config();
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
        let result: AddVoiceResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// List similar voices
    ///
    /// Returns a list of shared voices similar to the provided audio sample. If neither similarity_threshold nor top_k is provided, we will apply default values.
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn find_similar_voices(
        &self,
        body: Option<&ListSimilarVoicesRequest>,
        options: Option<RequestOptions>,
    ) -> Result<GetLibraryVoicesResponse, crate::error::ApiError> {
        let url = "/v1/similar-voices";

        let base_url = self
            .config
            .get_voices_config()
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
            request = request.multipart(b.clone().to_multipart());
        }
        let config = self.config.get_voices_config();
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
        let result: GetLibraryVoicesResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get shared voices
    ///
    /// Retrieves a list of shared voices.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_shared(
        &self,
        request: &GetSharedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLibraryVoicesResponse, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let category = request.category.clone();
        let gender = request.gender.clone();
        let age = request.age.clone();
        let accent = request.accent.clone();
        let language = request.language.clone();
        let locale = request.locale.clone();
        let search = request.search.clone();
        let use_cases = request.use_cases.clone();
        let descriptives = request.descriptives.clone();
        let featured = request.featured.clone();
        let min_notice_period_days = request.min_notice_period_days.clone();
        let include_custom_rates = request.include_custom_rates.clone();
        let include_live_moderated = request.include_live_moderated.clone();
        let reader_app_enabled = request.reader_app_enabled.clone();
        let owner_id = request.owner_id.clone();
        let sort = request.sort.clone();
        let page = request.page.clone();
        let url = "/v1/shared-voices";

        let base_url = self
            .config
            .get_voices_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::GET, &base_url, url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &category {
            request = request.query(&[("category", &val.to_string())]);
        }
        if let Some(val) = &gender {
            request = request.query(&[("gender", &val.to_string())]);
        }
        if let Some(val) = &age {
            request = request.query(&[("age", &val.to_string())]);
        }
        if let Some(val) = &accent {
            request = request.query(&[("accent", &val.to_string())]);
        }
        if let Some(val) = &language {
            request = request.query(&[("language", &val.to_string())]);
        }
        if let Some(val) = &locale {
            request = request.query(&[("locale", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &use_cases {
            for item in val.iter() {
                request = request.query(&[("use_cases", &item.to_string())]);
            }
        }
        if let Some(val) = &descriptives {
            for item in val.iter() {
                request = request.query(&[("descriptives", &item.to_string())]);
            }
        }
        if let Some(val) = &featured {
            request = request.query(&[("featured", &val.to_string())]);
        }
        if let Some(val) = &min_notice_period_days {
            request = request.query(&[("min_notice_period_days", &val.to_string())]);
        }
        if let Some(val) = &include_custom_rates {
            request = request.query(&[("include_custom_rates", &val.to_string())]);
        }
        if let Some(val) = &include_live_moderated {
            request = request.query(&[("include_live_moderated", &val.to_string())]);
        }
        if let Some(val) = &reader_app_enabled {
            request = request.query(&[("reader_app_enabled", &val.to_string())]);
        }
        if let Some(val) = &owner_id {
            request = request.query(&[("owner_id", &val.to_string())]);
        }
        if let Some(val) = &sort {
            request = request.query(&[("sort", &val.to_string())]);
        }
        if let Some(val) = &page {
            request = request.query(&[("page", &val.to_string())]);
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
        let config = self.config.get_voices_config();
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
        let result: GetLibraryVoicesResponse =
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
