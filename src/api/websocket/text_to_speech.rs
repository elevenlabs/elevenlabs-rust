/// Query parameters sent when opening the `v1_text-to-speech_voice_id_stream-input` channel.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TextToSpeechConnectOptions {
    pub authorization: Option<String>,
    pub single_use_token: Option<String>,
    pub model_id: Option<String>,
    pub language_code: Option<String>,
    pub enable_logging: Option<bool>,
    pub output_format: Option<String>,
    pub inactivity_timeout: Option<i64>,
    pub sync_alignment: Option<bool>,
    pub auto_mode: Option<bool>,
    pub apply_text_normalization:
        Option<crate::api::types::BodyTextToDialogueFullWithTimestampsApplyTextNormalization>,
    pub seed: Option<i64>,
    pub enable_ssml_parsing: Option<bool>,
}

/// A JSON message the server sends on the `v1_text-to-speech_voice_id_stream-input` channel.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum TextToSpeechServerMessage {
    ReceiveMessage(crate::api::types::ReceiveMessage),
    /// Unknown or new server message type not yet supported by this SDK version.
    Unknown(serde_json::Value),
}

/// Picks the variant whose re-serialized keys overlap the frame's keys the most; a tie keeps the
/// earlier variant, and a frame no variant matches, or one that is not a non-empty object, is
/// `Unknown`.
impl<'de> serde::Deserialize<'de> for TextToSpeechServerMessage {
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

        if let Ok(v) = serde_json::from_value::<crate::api::types::ReceiveMessage>(value.clone()) {
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
                    best_variant = Some(Self::ReceiveMessage(v));
                }
            }
        }

        let _ = best_score;
        Ok(best_variant.unwrap_or(Self::Unknown(value)))
    }
}

/// An open connection to the `v1_text-to-speech_voice_id_stream-input` channel.
pub struct TextToSpeechClient {
    socket: crate::core::websocket::WebSocketClient,
    incoming: tokio::sync::mpsc::UnboundedReceiver<
        Result<crate::core::websocket::WebSocketMessage, crate::error::ApiError>,
    >,
}

impl TextToSpeechClient {
    /// Opens the channel at `url`, the API's base URL. The channel's address `/v1/text-to-speech/{voice_id}/stream-input`
    /// is appended to it.
    pub async fn realtime(
        url: &str,
        voice_id: &str,
        api_key: Option<&str>,
        options: &TextToSpeechConnectOptions,
    ) -> Result<Self, crate::error::ApiError> {
        let mut ws_options = crate::core::websocket::WebSocketOptions::default();
        if let Some(value) = api_key {
            ws_options.set_header("xi-api-key", value);
        }
        if let Some(value) = &options.authorization {
            ws_options
                .query_params
                .push(("authorization".to_string(), value.clone()));
        }
        if let Some(value) = &options.single_use_token {
            ws_options
                .query_params
                .push(("single_use_token".to_string(), value.clone()));
        }
        if let Some(value) = &options.model_id {
            ws_options
                .query_params
                .push(("model_id".to_string(), value.clone()));
        }
        if let Some(value) = &options.language_code {
            ws_options
                .query_params
                .push(("language_code".to_string(), value.clone()));
        }
        if let Some(value) = options.enable_logging {
            ws_options
                .query_params
                .push(("enable_logging".to_string(), value.to_string()));
        }
        if let Some(value) = &options.output_format {
            ws_options
                .query_params
                .push(("output_format".to_string(), value.clone()));
        }
        if let Some(value) = options.inactivity_timeout {
            ws_options
                .query_params
                .push(("inactivity_timeout".to_string(), value.to_string()));
        }
        if let Some(value) = options.sync_alignment {
            ws_options
                .query_params
                .push(("sync_alignment".to_string(), value.to_string()));
        }
        if let Some(value) = options.auto_mode {
            ws_options
                .query_params
                .push(("auto_mode".to_string(), value.to_string()));
        }
        if let Some(value) = &options.apply_text_normalization {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "apply_text_normalization".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = options.seed {
            ws_options
                .query_params
                .push(("seed".to_string(), value.to_string()));
        }
        if let Some(value) = options.enable_ssml_parsing {
            ws_options
                .query_params
                .push(("enable_ssml_parsing".to_string(), value.to_string()));
        }
        let full_url = crate::core::websocket::ws_url(
            url,
            &format!("/v1/text-to-speech/{}/stream-input", voice_id),
        );
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
        message: &crate::api::types::SendMessage,
    ) -> Result<(), crate::error::ApiError> {
        self.socket.send_json(message).await
    }

    /// Receives the next message, or `None` once the server closes or the stream ends.
    pub async fn recv(
        &mut self,
    ) -> Option<Result<TextToSpeechServerMessage, crate::error::ApiError>> {
        use crate::core::websocket::WebSocketMessage;
        match self.incoming.recv().await {
            Some(Ok(WebSocketMessage::Text(raw))) => Some(
                serde_json::from_str::<TextToSpeechServerMessage>(&raw)
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

/// Opens the `v1_text-to-speech_voice_id_stream-input` channel. The root client holds one; `new` builds one by hand.
pub struct TextToSpeechConnector {
    source: TextToSpeechConnectorSource,
}

enum TextToSpeechConnectorSource {
    Fixed {
        base_url: String,
        auth_header: Option<String>,
    },
    #[allow(dead_code)]
    Shared {
        config: std::sync::Arc<crate::core::config_manager::ConfigManager>,
    },
}

impl TextToSpeechConnector {
    /// A connector that opens the channel at `base_url`, sending `auth_header` as the
    /// `xi-api-key` header when one is given.
    pub fn new(base_url: String, auth_header: Option<String>) -> Self {
        Self {
            source: TextToSpeechConnectorSource::Fixed {
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
            source: TextToSpeechConnectorSource::Shared { config },
        }
    }

    /// Opens the `v1_text-to-speech_voice_id_stream-input` channel.
    pub async fn realtime(
        &self,
        voice_id: &str,
        options: &TextToSpeechConnectOptions,
    ) -> Result<TextToSpeechClient, crate::error::ApiError> {
        self.realtime_with_options(
            voice_id,
            options,
            crate::core::websocket::WebSocketOptions::default(),
        )
        .await
    }

    /// Opens the `v1_text-to-speech_voice_id_stream-input` channel with the caller's transport options: reconnection,
    /// timeouts, and extra handshake headers or query parameters.
    ///
    /// The socket URL is derived from the configured base URL when this connector came from the
    /// root client, so a later `set_base_url` reaches it.
    pub async fn realtime_with_options(
        &self,
        voice_id: &str,
        options: &TextToSpeechConnectOptions,
        ws_options: crate::core::websocket::WebSocketOptions,
    ) -> Result<TextToSpeechClient, crate::error::ApiError> {
        let mut ws_options = ws_options;
        if let Some(value) = &options.authorization {
            ws_options
                .query_params
                .push(("authorization".to_string(), value.clone()));
        }
        if let Some(value) = &options.single_use_token {
            ws_options
                .query_params
                .push(("single_use_token".to_string(), value.clone()));
        }
        if let Some(value) = &options.model_id {
            ws_options
                .query_params
                .push(("model_id".to_string(), value.clone()));
        }
        if let Some(value) = &options.language_code {
            ws_options
                .query_params
                .push(("language_code".to_string(), value.clone()));
        }
        if let Some(value) = options.enable_logging {
            ws_options
                .query_params
                .push(("enable_logging".to_string(), value.to_string()));
        }
        if let Some(value) = &options.output_format {
            ws_options
                .query_params
                .push(("output_format".to_string(), value.clone()));
        }
        if let Some(value) = options.inactivity_timeout {
            ws_options
                .query_params
                .push(("inactivity_timeout".to_string(), value.to_string()));
        }
        if let Some(value) = options.sync_alignment {
            ws_options
                .query_params
                .push(("sync_alignment".to_string(), value.to_string()));
        }
        if let Some(value) = options.auto_mode {
            ws_options
                .query_params
                .push(("auto_mode".to_string(), value.to_string()));
        }
        if let Some(value) = &options.apply_text_normalization {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "apply_text_normalization".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = options.seed {
            ws_options
                .query_params
                .push(("seed".to_string(), value.to_string()));
        }
        if let Some(value) = options.enable_ssml_parsing {
            ws_options
                .query_params
                .push(("enable_ssml_parsing".to_string(), value.to_string()));
        }
        let base_url = match &self.source {
            TextToSpeechConnectorSource::Fixed {
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
            TextToSpeechConnectorSource::Shared { config } => {
                let config = config.get_v_1_text_to_speech_voice_id_stream_input_config();
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
        let url = crate::core::websocket::ws_url(
            &base_url,
            &format!("/v1/text-to-speech/{}/stream-input", voice_id),
        );
        TextToSpeechClient::open(&url, ws_options).await
    }
}
