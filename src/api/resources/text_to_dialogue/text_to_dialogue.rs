#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct TextToDialogueClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl TextToDialogueClient {
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

    /// Create dialogue
    ///
    /// Converts a list of text and voice ID pairs into speech (dialogue) and returns audio.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// The generated audio file
    pub async fn convert(
        &self,
        request: &TextToDialogueConvertRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/text-to-dialogue";

        let base_url = self
            .config
            .get_text_to_dialogue_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .stream_request_at(reqwest::Method::POST, &base_url, url);
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
        }
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
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
        let config = self.config.get_text_to_dialogue_config();
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

    /// Stream dialogue
    ///
    /// Converts a list of text and voice ID pairs into speech (dialogue) and returns an audio stream.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Streaming audio data
    pub async fn stream(
        &self,
        request: &TextToDialogueStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/text-to-dialogue/stream";

        let base_url = self
            .config
            .get_text_to_dialogue_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .stream_request_at(reqwest::Method::POST, &base_url, url);
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
        }
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
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
        let config = self.config.get_text_to_dialogue_config();
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

    /// Text To Dialogue Streaming With Timestamps
    ///
    /// Converts a list of text and voice ID pairs into speech (dialogue) and returns a stream of JSON blobs containing audio as a base64 encoded string and timestamps
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// A stream of server-sent events from the API.
    pub async fn stream_with_timestamps(
        &self,
        request: &TextToDialogueStreamWithTimestampsRequest,
        options: Option<RequestOptions>,
    ) -> Result<
        crate::SseStream<StreamingAudioChunkWithTimestampsAndVoiceSegmentsResponseModel>,
        crate::error::ApiError,
    > {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/text-to-dialogue/stream/with-timestamps";

        let base_url = self
            .config
            .get_text_to_dialogue_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
        }
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
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
        let config = self.config.get_text_to_dialogue_config();
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

        let stream = crate::core::sse::open_sse_stream::<
            StreamingAudioChunkWithTimestampsAndVoiceSegmentsResponseModel,
        >(response, false, false, None, None);
        Ok(stream)
    }

    /// Text To Dialogue With Timestamps
    ///
    /// Generate dialogue from text with precise character-level timing information for audio-text synchronization.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn convert_with_timestamps(
        &self,
        request: &TextToDialogueConvertWithTimestampsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioWithTimestampsAndVoiceSegmentsResponseModel, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/text-to-dialogue/with-timestamps";

        let base_url = self
            .config
            .get_text_to_dialogue_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .request_at(reqwest::Method::POST, &base_url, url);
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
        }
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
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
        let config = self.config.get_text_to_dialogue_config();
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
        let result: AudioWithTimestampsAndVoiceSegmentsResponseModel = response
            .json()
            .await
            .map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?;
        Ok(result)
    }
}
