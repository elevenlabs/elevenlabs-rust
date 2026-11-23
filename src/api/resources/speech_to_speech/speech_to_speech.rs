#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct SpeechToSpeechClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl SpeechToSpeechClient {
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

    /// Voice changer
    ///
    /// Transform audio from one voice to another. Maintain full control over emotion, timing and delivery.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `body`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// The generated audio file
    pub async fn convert(
        &self,
        voice_id: &str,
        body: Option<&SpeechToSpeechRequest>,
        request: &ConvertQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let enable_logging = request.enable_logging.clone();
        let optimize_streaming_latency = request.optimize_streaming_latency.clone();
        let output_format = request.output_format.clone();
        let url = format!("/v1/voice-changer/{}", voice_id);

        let base_url = self
            .config
            .get_speech_to_speech_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .stream_request_at(reqwest::Method::POST, &base_url, url);
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
        if let Some(b) = body {
            request = request.multipart(b.clone().to_multipart());
        }
        let config = self.config.get_speech_to_speech_config();
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

    /// Voice changer stream
    ///
    /// Stream audio from one voice to another. Maintain full control over emotion, timing and delivery.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `body`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Streaming audio data
    pub async fn stream(
        &self,
        voice_id: &str,
        body: Option<&SpeechToSpeechStreamRequest>,
        request: &StreamQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let enable_logging = request.enable_logging.clone();
        let optimize_streaming_latency = request.optimize_streaming_latency.clone();
        let output_format = request.output_format.clone();
        let url = format!("/v1/voice-changer/{}/stream", voice_id);

        let base_url = self
            .config
            .get_speech_to_speech_config()
            .service_url(|environment| environment.base_url())
            .to_string();
        let mut request = self
            .http_client
            .stream_request_at(reqwest::Method::POST, &base_url, url);
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
        if let Some(b) = body {
            request = request.multipart(b.clone().to_multipart());
        }
        let config = self.config.get_speech_to_speech_config();
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
}
