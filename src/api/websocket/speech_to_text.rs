/// Query parameters sent when opening the `v1_speech-to-text_realtime` channel.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SpeechToTextConnectOptions {
    pub model_id: Option<crate::api::types::SpeechToTextRealtimeModelId>,
    pub token: Option<String>,
    pub audio_format: Option<String>,
    pub language_code: Option<String>,
    pub secondary_languages: Vec<String>,
    pub commit_strategy: Option<crate::api::types::SpeechToTextRealtimeCommitStrategy>,
    pub vad_threshold: Option<f64>,
    pub vad_silence_threshold_secs: Option<f64>,
    pub min_speech_duration_ms: Option<i64>,
    pub min_silence_duration_ms: Option<i64>,
    pub include_timestamps: Option<bool>,
    pub include_language_detection: Option<bool>,
    pub keyterms: Vec<String>,
    pub no_verbatim: Option<bool>,
    pub multilang_mode: Option<bool>,
    pub diarize: Option<bool>,
    pub entity_detection: Option<crate::api::types::V1SpeechToTextRealtimeEntityDetection>,
    pub entity_redaction: Option<crate::api::types::V1SpeechToTextRealtimeEntityRedaction>,
    pub entity_redaction_mode: Option<crate::api::types::SpeechToTextRealtimeEntityRedactionMode>,
    pub transcript_edit: Option<String>,
    pub filter_background_audio: Option<bool>,
    pub keepalive_interval_ms: Option<i64>,
    pub enable_logging: Option<bool>,
}

/// A JSON message the server sends on the `v1_speech-to-text_realtime` channel.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum SpeechToTextServerMessage {
    ReceiveTranscription(crate::api::types::ReceiveTranscription),
    /// Unknown or new server message type not yet supported by this SDK version.
    Unknown(serde_json::Value),
}

/// Picks the variant whose re-serialized keys overlap the frame's keys the most; a tie keeps the
/// earlier variant, and a frame no variant matches, or one that is not a non-empty object, is
/// `Unknown`.
impl<'de> serde::Deserialize<'de> for SpeechToTextServerMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let original_keys: std::collections::BTreeSet<String> = value
            .as_object()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();

        if original_keys.is_empty() {
            return Ok(Self::Unknown(value));
        }

        let mut best_variant: Option<Self> = None;
        let mut best_score: usize = 0;

        if let Ok(v) =
            serde_json::from_value::<crate::api::types::ReceiveTranscription>(value.clone())
        {
            if let Ok(re) = serde_json::to_value(&v) {
                let score = re
                    .as_object()
                    .map(|o| {
                        o.keys()
                            .filter(|k| original_keys.contains(k.as_str()))
                            .count()
                    })
                    .unwrap_or(0);
                if score > best_score {
                    best_score = score;
                    best_variant = Some(Self::ReceiveTranscription(v));
                }
            }
        }

        let _ = best_score;
        Ok(best_variant.unwrap_or(Self::Unknown(value)))
    }
}

/// An open connection to the `v1_speech-to-text_realtime` channel.
pub struct SpeechToTextClient {
    socket: crate::core::websocket::WebSocketClient,
    incoming: tokio::sync::mpsc::UnboundedReceiver<
        Result<crate::core::websocket::WebSocketMessage, crate::error::ApiError>,
    >,
}

impl SpeechToTextClient {
    /// Opens the channel at `url`, the API's base URL. The channel's address `/v1/speech-to-text/realtime`
    /// is appended to it.
    pub async fn realtime(
        url: &str,
        api_key: Option<&str>,
        options: &SpeechToTextConnectOptions,
    ) -> Result<Self, crate::error::ApiError> {
        let mut ws_options = crate::core::websocket::WebSocketOptions::default();
        if let Some(value) = api_key {
            ws_options.set_header("xi-api-key", value);
        }
        if let Some(value) = &options.model_id {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "model_id".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.token {
            ws_options
                .query_params
                .push(("token".to_string(), value.clone()));
        }
        if let Some(value) = &options.audio_format {
            ws_options
                .query_params
                .push(("audio_format".to_string(), value.clone()));
        }
        if let Some(value) = &options.language_code {
            ws_options
                .query_params
                .push(("language_code".to_string(), value.clone()));
        }
        for value in &options.secondary_languages {
            ws_options
                .query_params
                .push(("secondary_languages".to_string(), value.clone()));
        }
        if let Some(value) = &options.commit_strategy {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "commit_strategy".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = options.vad_threshold {
            ws_options
                .query_params
                .push(("vad_threshold".to_string(), value.to_string()));
        }
        if let Some(value) = options.vad_silence_threshold_secs {
            ws_options
                .query_params
                .push(("vad_silence_threshold_secs".to_string(), value.to_string()));
        }
        if let Some(value) = options.min_speech_duration_ms {
            ws_options
                .query_params
                .push(("min_speech_duration_ms".to_string(), value.to_string()));
        }
        if let Some(value) = options.min_silence_duration_ms {
            ws_options
                .query_params
                .push(("min_silence_duration_ms".to_string(), value.to_string()));
        }
        if let Some(value) = options.include_timestamps {
            ws_options
                .query_params
                .push(("include_timestamps".to_string(), value.to_string()));
        }
        if let Some(value) = options.include_language_detection {
            ws_options
                .query_params
                .push(("include_language_detection".to_string(), value.to_string()));
        }
        for value in &options.keyterms {
            ws_options
                .query_params
                .push(("keyterms".to_string(), value.clone()));
        }
        if let Some(value) = options.no_verbatim {
            ws_options
                .query_params
                .push(("no_verbatim".to_string(), value.to_string()));
        }
        if let Some(value) = options.multilang_mode {
            ws_options
                .query_params
                .push(("multilang_mode".to_string(), value.to_string()));
        }
        if let Some(value) = options.diarize {
            ws_options
                .query_params
                .push(("diarize".to_string(), value.to_string()));
        }
        if let Some(value) = &options.entity_detection {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "entity_detection".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.entity_redaction {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "entity_redaction".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.entity_redaction_mode {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "entity_redaction_mode".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.transcript_edit {
            ws_options
                .query_params
                .push(("transcript_edit".to_string(), value.clone()));
        }
        if let Some(value) = options.filter_background_audio {
            ws_options
                .query_params
                .push(("filter_background_audio".to_string(), value.to_string()));
        }
        if let Some(value) = options.keepalive_interval_ms {
            ws_options
                .query_params
                .push(("keepalive_interval_ms".to_string(), value.to_string()));
        }
        if let Some(value) = options.enable_logging {
            ws_options
                .query_params
                .push(("enable_logging".to_string(), value.to_string()));
        }
        let full_url = crate::core::websocket::ws_url(url, "/v1/speech-to-text/realtime");
        Self::open(&full_url, ws_options).await
    }

    async fn open(
        url: &str,
        ws_options: crate::core::websocket::WebSocketOptions,
    ) -> Result<Self, crate::error::ApiError> {
        let (socket, incoming) =
            crate::core::websocket::WebSocketClient::connect(url, ws_options).await?;
        Ok(Self { socket, incoming })
    }

    /// Defines the message types that can be sent from client to server
    pub async fn send_publish(
        &self,
        message: &crate::api::types::InputAudioChunk,
    ) -> Result<(), crate::error::ApiError> {
        self.socket.send_json(message).await
    }

    /// Receives the next message, or `None` once the server closes or the stream ends.
    pub async fn recv(
        &mut self,
    ) -> Option<Result<SpeechToTextServerMessage, crate::error::ApiError>> {
        use crate::core::websocket::WebSocketMessage;
        match self.incoming.recv().await {
            Some(Ok(WebSocketMessage::Text(raw))) => Some(
                serde_json::from_str::<SpeechToTextServerMessage>(&raw)
                    .map_err(crate::error::ApiError::Serialization),
            ),
            Some(Ok(WebSocketMessage::Binary(data))) => {
                Some(Err(crate::error::ApiError::WebSocketError(format!(
                    "Received unexpected binary frame ({} bytes) on a JSON-only channel",
                    data.len()
                ))))
            }
            Some(Ok(WebSocketMessage::Close(_))) => None,
            Some(Err(error)) => Some(Err(error)),
            None => None,
        }
    }

    /// Closes the socket. Idempotent.
    pub async fn close(&self) -> Result<(), crate::error::ApiError> {
        self.socket.close().await
    }

    pub async fn state(&self) -> crate::core::websocket::WebSocketState {
        self.socket.state().await
    }
}

/// Opens the `v1_speech-to-text_realtime` channel. The root client holds one; `new` builds one by hand.
pub struct SpeechToTextConnector {
    source: SpeechToTextConnectorSource,
}

enum SpeechToTextConnectorSource {
    Fixed {
        base_url: String,
        auth_header: Option<String>,
    },
    #[allow(dead_code)]
    Shared {
        config: std::sync::Arc<crate::core::config_manager::ConfigManager>,
    },
}

impl SpeechToTextConnector {
    /// A connector that opens the channel at `base_url`, sending `auth_header` as the
    /// `xi-api-key` header when one is given.
    pub fn new(base_url: String, auth_header: Option<String>) -> Self {
        Self {
            source: SpeechToTextConnectorSource::Fixed {
                base_url,
                auth_header,
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_shared(
        config: std::sync::Arc<crate::core::config_manager::ConfigManager>,
    ) -> Self {
        Self {
            source: SpeechToTextConnectorSource::Shared { config },
        }
    }

    /// Opens the `v1_speech-to-text_realtime` channel.
    pub async fn realtime(
        &self,
        options: &SpeechToTextConnectOptions,
    ) -> Result<SpeechToTextClient, crate::error::ApiError> {
        self.realtime_with_options(options, crate::core::websocket::WebSocketOptions::default())
            .await
    }

    /// Opens the `v1_speech-to-text_realtime` channel with the caller's transport options: reconnection,
    /// timeouts, and extra handshake headers or query parameters.
    ///
    /// The socket URL is derived from the configured base URL when this connector came from the
    /// root client, so a later `set_base_url` reaches it.
    pub async fn realtime_with_options(
        &self,
        options: &SpeechToTextConnectOptions,
        ws_options: crate::core::websocket::WebSocketOptions,
    ) -> Result<SpeechToTextClient, crate::error::ApiError> {
        let mut ws_options = ws_options;
        if let Some(value) = &options.model_id {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "model_id".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.token {
            ws_options
                .query_params
                .push(("token".to_string(), value.clone()));
        }
        if let Some(value) = &options.audio_format {
            ws_options
                .query_params
                .push(("audio_format".to_string(), value.clone()));
        }
        if let Some(value) = &options.language_code {
            ws_options
                .query_params
                .push(("language_code".to_string(), value.clone()));
        }
        for value in &options.secondary_languages {
            ws_options
                .query_params
                .push(("secondary_languages".to_string(), value.clone()));
        }
        if let Some(value) = &options.commit_strategy {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "commit_strategy".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = options.vad_threshold {
            ws_options
                .query_params
                .push(("vad_threshold".to_string(), value.to_string()));
        }
        if let Some(value) = options.vad_silence_threshold_secs {
            ws_options
                .query_params
                .push(("vad_silence_threshold_secs".to_string(), value.to_string()));
        }
        if let Some(value) = options.min_speech_duration_ms {
            ws_options
                .query_params
                .push(("min_speech_duration_ms".to_string(), value.to_string()));
        }
        if let Some(value) = options.min_silence_duration_ms {
            ws_options
                .query_params
                .push(("min_silence_duration_ms".to_string(), value.to_string()));
        }
        if let Some(value) = options.include_timestamps {
            ws_options
                .query_params
                .push(("include_timestamps".to_string(), value.to_string()));
        }
        if let Some(value) = options.include_language_detection {
            ws_options
                .query_params
                .push(("include_language_detection".to_string(), value.to_string()));
        }
        for value in &options.keyterms {
            ws_options
                .query_params
                .push(("keyterms".to_string(), value.clone()));
        }
        if let Some(value) = options.no_verbatim {
            ws_options
                .query_params
                .push(("no_verbatim".to_string(), value.to_string()));
        }
        if let Some(value) = options.multilang_mode {
            ws_options
                .query_params
                .push(("multilang_mode".to_string(), value.to_string()));
        }
        if let Some(value) = options.diarize {
            ws_options
                .query_params
                .push(("diarize".to_string(), value.to_string()));
        }
        if let Some(value) = &options.entity_detection {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "entity_detection".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.entity_redaction {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "entity_redaction".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.entity_redaction_mode {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "entity_redaction_mode".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.transcript_edit {
            ws_options
                .query_params
                .push(("transcript_edit".to_string(), value.clone()));
        }
        if let Some(value) = options.filter_background_audio {
            ws_options
                .query_params
                .push(("filter_background_audio".to_string(), value.to_string()));
        }
        if let Some(value) = options.keepalive_interval_ms {
            ws_options
                .query_params
                .push(("keepalive_interval_ms".to_string(), value.to_string()));
        }
        if let Some(value) = options.enable_logging {
            ws_options
                .query_params
                .push(("enable_logging".to_string(), value.to_string()));
        }
        let base_url = match &self.source {
            SpeechToTextConnectorSource::Fixed {
                base_url,
                auth_header,
            } => {
                if let Some(auth) = auth_header {
                    if !ws_options.supplies_header("xi-api-key") {
                        ws_options
                            .headers
                            .insert("xi-api-key".to_string(), auth.clone());
                    }
                }
                base_url.clone()
            }
            SpeechToTextConnectorSource::Shared { config } => {
                let config = config.get_v_1_speech_to_text_realtime_config();
                if let Some(key) = config.api_key.as_deref() {
                    if !ws_options.supplies_header("xi-api-key") {
                        ws_options
                            .headers
                            .insert("xi-api-key".to_string(), key.to_string());
                    }
                }
                for (name, value) in &config.custom_headers {
                    if !ws_options.supplies_header(name) {
                        ws_options.headers.insert(name.clone(), value.clone());
                    }
                }
                config.base_url
            }
        };
        let url = crate::core::websocket::ws_url(&base_url, "/v1/speech-to-text/realtime");
        SpeechToTextClient::open(&url, ws_options).await
    }
}
