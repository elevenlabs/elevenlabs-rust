#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct MusicClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl MusicClient {
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

    /// Video To Music
    ///
    /// Generate background music from one or more video files. Videos are combined in order. Optional description and style tags influence the generated music.
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Generated audio file matching the video. Content-Type and file extension depend on the output_format parameter (default mp3).
    pub async fn video_to_music(
        &self,
        body: &BodyVideoToMusicV1MusicVideoToMusicPost,
        request: &VideoToMusicQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let url = "/v1/music/video-to-music";

        let mut request = self.http_client.stream_request(reqwest::Method::POST, url);
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
        request = request.multipart(body.clone().to_multipart());
        let config = self.config.get_music_config();
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

    /// Compose Music
    ///
    /// Compose a song from a prompt or a composition plan.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// The generated audio file in the format specified
    pub async fn compose(
        &self,
        request: &ComposeRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/music";

        let mut request = self.http_client.stream_request(reqwest::Method::POST, url);
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
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_music_config();
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

    /// Compose Music With A Detailed Response
    ///
    /// Compose a song from a prompt or a composition plan.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Multipart/mixed response with JSON metadata and binary audio file
    pub async fn compose_detailed(
        &self,
        request: &ComposeDetailedRequest,
        options: Option<RequestOptions>,
    ) -> Result<ComposeDetailedOkResponse, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/music/detailed";

        let mut request = self.http_client.post(url);
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
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_music_config();
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
        let result: ComposeDetailedOkResponse = if bytes.is_empty() {
            Default::default()
        } else {
            serde_json::from_slice(&bytes).map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?
        };
        Ok(result)
    }

    /// Stream Composed Music With A Detailed Response
    ///
    /// Stream a song and its detailed metadata using Server-Sent Events (SSE).
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// A stream of server-sent events from the API.
    pub async fn compose_detailed_stream(
        &self,
        request: &ComposeDetailedStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::SseStream<String>, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/music/detailed/stream";

        let mut request = self.http_client.post(url);
        request = request.header("Accept", "text/event-stream");
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
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_music_config();
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

        let stream = crate::core::sse::open_sse_stream::<String>(response, true, true, None, None);
        Ok(stream)
    }

    /// Stream Composed Music
    ///
    /// Stream a composed song from a prompt or a composition plan.
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Streaming audio data in the format specified
    pub async fn stream(
        &self,
        request: &MusicStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let body = request.body.clone();
        let url = "/v1/music/stream";

        let mut request = self.http_client.stream_request(reqwest::Method::POST, url);
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
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_music_config();
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

    /// Upload Music
    ///
    /// Upload a music file to be later used for inpainting. Price for uploading is the same as the one for song generation. All uploaded content gets inspected for copyright infringement. If copyrighted content is detected, half of the request cost is still charged.
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successfully uploaded music file with optional composition plan
    pub async fn upload(
        &self,
        body: &BodyUploadMusicV1MusicUploadPost,
        options: Option<RequestOptions>,
    ) -> Result<MusicUploadResponse, crate::error::ApiError> {
        let url = "/v1/music/upload";

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
        request = request.multipart(body.clone().to_multipart());
        let config = self.config.get_music_config();
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
        let result: MusicUploadResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Stem Separation
    ///
    /// Separate an audio file into individual stems. This endpoint might have high latency, depending on the length of the audio file.
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// ZIP archive containing separated audio stems. Each stem is provided as a separate audio file in the requested output format.
    pub async fn separate_stems(
        &self,
        body: &BodyStemSeparationV1MusicStemSeparationPost,
        request: &SeparateStemsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::core::ByteStream, crate::error::ApiError> {
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging.clone();
        let url = "/v1/music/stem-separation";

        let mut request = self.http_client.stream_request(reqwest::Method::POST, url);
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
        request = request.multipart(body.clone().to_multipart());
        let config = self.config.get_music_config();
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
