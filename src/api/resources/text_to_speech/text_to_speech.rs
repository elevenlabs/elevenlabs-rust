#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct TextToSpeechClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl TextToSpeechClient {
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

    /// Create speech
    ///
    /// Converts text into speech using a voice of your choice and returns audio.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// The generated audio file
    pub async fn convert(
        &self,
        voice_id: &str,
        request: &TextToSpeechConvertRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let enable_logging = request.enable_logging.clone();
        let optimize_streaming_latency = request.optimize_streaming_latency.clone();
        let output_format = request.output_format.clone();
        let body = request.body.clone();
        let url = format!("/v1/text-to-speech/{}", voice_id);

        let mut request = self.http_client.stream_request(reqwest::Method::POST, url);
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
        }
        if let Some(val) = &optimize_streaming_latency {
            request = request.query(&[("optimize_streaming_latency", &val.to_string())]);
        }
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
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
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_text_to_speech_config();
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

        let result = crate::core::ByteStream::new(response);
        Ok(result)
    }

    /// Create speech with timing
    ///
    /// Generate speech from text with precise character-level timing information for audio-text synchronization.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn convert_with_timestamps(
        &self,
        voice_id: &str,
        request: &ConvertWithTimestampsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioWithTimestampsResponse, crate::error::ApiError> {
        let enable_logging = request.enable_logging.clone();
        let optimize_streaming_latency = request.optimize_streaming_latency.clone();
        let output_format = request.output_format.clone();
        let body = request.body.clone();
        let url = format!("/v1/text-to-speech/{}/with-timestamps", voice_id);

        let mut request = self.http_client.post(url);
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
        }
        if let Some(val) = &optimize_streaming_latency {
            request = request.query(&[("optimize_streaming_latency", &val.to_string())]);
        }
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
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
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_text_to_speech_config();
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
        let result: AudioWithTimestampsResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Stream speech
    ///
    /// Converts text into speech using a voice of your choice and returns audio as an audio stream.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Streaming audio data
    pub async fn stream(
        &self,
        voice_id: &str,
        request: &StreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let enable_logging = request.enable_logging.clone();
        let optimize_streaming_latency = request.optimize_streaming_latency.clone();
        let output_format = request.output_format.clone();
        let body = request.body.clone();
        let url = format!("/v1/text-to-speech/{}/stream", voice_id);

        let mut request = self.http_client.stream_request(reqwest::Method::POST, url);
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
        }
        if let Some(val) = &optimize_streaming_latency {
            request = request.query(&[("optimize_streaming_latency", &val.to_string())]);
        }
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
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
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_text_to_speech_config();
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

        let result = crate::core::ByteStream::new(response);
        Ok(result)
    }

    /// Stream speech with timing
    ///
    /// Converts text into speech using a voice of your choice and returns a stream of JSONs containing audio as a base64 encoded string together with information on when which character was spoken.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// A stream of server-sent events from the API.
    pub async fn stream_with_timestamps(
        &self,
        voice_id: &str,
        request: &StreamWithTimestampsRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::SseStream<StreamingAudioChunkWithTimestampsResponse>, crate::error::ApiError>
    {
        let enable_logging = request.enable_logging.clone();
        let optimize_streaming_latency = request.optimize_streaming_latency.clone();
        let output_format = request.output_format.clone();
        let body = request.body.clone();
        let url = format!("/v1/text-to-speech/{}/stream/with-timestamps", voice_id);

        let mut request = self.http_client.post(url);
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
        }
        if let Some(val) = &optimize_streaming_latency {
            request = request.query(&[("optimize_streaming_latency", &val.to_string())]);
        }
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
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
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_text_to_speech_config();
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

        let stream = crate::core::sse::open_sse_stream::<StreamingAudioChunkWithTimestampsResponse>(
            response, false, false, None, None,
        );
        Ok(stream)
    }

    /// List Speech Generations
    ///
    /// List the speech generations created through this API, newest first.
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
        request: &TextToSpeechListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationListResponse, crate::error::ApiError> {
        let cursor = request.cursor.clone();
        let page_size = request.page_size.clone();
        let status = request.status.clone();
        let model_id = request.model_id.clone();
        let url = "/v1/flows/text-to-speech";

        let mut request = self.http_client.get(url);
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &status {
            request = request.query(&[("status", &val.to_string())]);
        }
        if let Some(val) = &model_id {
            request = request.query(&[("model_id", &val.to_string())]);
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
        let config = self.config.get_text_to_speech_config();
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
        let result: MediaGenerationListResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Create Speech Generation
    ///
    /// Start a speech generation with the selected model. Charged per character via text-to-speech billing. Use this over `/v1/text-to-speech` for the asynchronous generation lifecycle or for models not offered there; for direct, synchronous speech synthesis, prefer `/v1/text-to-speech`.
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
        body: &TextToSpeechGenerationRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationCreateResponse, crate::error::ApiError> {
        let url = "/v1/flows/text-to-speech";

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
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_text_to_speech_config();
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
        let result: MediaGenerationCreateResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Speech Generation
    ///
    /// Retrieve the status of a speech generation, and retrieve its output URL once completed.
    ///
    /// # Arguments
    ///
    /// * `generation_id`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        generation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationResponse, crate::error::ApiError> {
        let url = format!("/v1/flows/text-to-speech/{}", generation_id);

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
        let config = self.config.get_text_to_speech_config();
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
        let result: MediaGenerationResponse =
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
