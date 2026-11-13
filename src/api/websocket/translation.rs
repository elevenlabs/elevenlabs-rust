/// Query parameters sent when opening the `v1_translation_realtime` channel.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TranslationConnectOptions {
    pub authorization: Option<String>,
    pub target_language: Option<String>,
    pub source_language: Option<String>,
    pub voice_id: Option<String>,
    pub input_format: Option<crate::api::types::TranslationRealtimeInputFormat>,
    pub output_format: Option<crate::api::types::TranslationRealtimeOutputFormat>,
    pub tts_model_id: Option<crate::api::types::TranslationRealtimeTtsModelId>,
    pub apply_text_normalization:
        Option<crate::api::types::BodyTextToDialogueFullWithTimestampsApplyTextNormalization>,
    pub eagerness: Option<crate::api::types::TranslationRealtimeEagerness>,
    pub client_session_id: Option<String>,
    pub enable_zrm: Option<bool>,
}

/// A JSON message the server sends on the `v1_translation_realtime` channel.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum TranslationServerMessage {
    ReceiveTranslateStreamMessage(crate::api::types::ReceiveTranslateStreamMessage),
    /// Unknown or new server message type not yet supported by this SDK version.
    Unknown(serde_json::Value),
}

/// Picks the variant whose re-serialized keys overlap the frame's keys the most; a tie keeps the
/// earlier variant, and a frame no variant matches, or one that is not a non-empty object, is
/// `Unknown`.
impl<'de> serde::Deserialize<'de> for TranslationServerMessage {
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

        if let Ok(v) = serde_json::from_value::<crate::api::types::ReceiveTranslateStreamMessage>(
            value.clone(),
        ) {
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
                    best_variant = Some(Self::ReceiveTranslateStreamMessage(v));
                }
            }
        }

        let _ = best_score;
        Ok(best_variant.unwrap_or(Self::Unknown(value)))
    }
}

/// An open connection to the `v1_translation_realtime` channel.
pub struct TranslationClient {
    socket: crate::core::websocket::WebSocketClient,
    incoming: tokio::sync::mpsc::UnboundedReceiver<
        Result<crate::core::websocket::WebSocketMessage, crate::error::ApiError>,
    >,
}

impl TranslationClient {
    /// Opens the channel at `url`, the API's base URL. The channel's address `/v1/translation/realtime`
    /// is appended to it.
    pub async fn realtime(
        url: &str,
        api_key: Option<&str>,
        options: &TranslationConnectOptions,
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
        if let Some(value) = &options.target_language {
            ws_options
                .query_params
                .push(("target_language".to_string(), value.clone()));
        }
        if let Some(value) = &options.source_language {
            ws_options
                .query_params
                .push(("source_language".to_string(), value.clone()));
        }
        if let Some(value) = &options.voice_id {
            ws_options
                .query_params
                .push(("voice_id".to_string(), value.clone()));
        }
        if let Some(value) = &options.input_format {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "input_format".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.output_format {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "output_format".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.tts_model_id {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "tts_model_id".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.apply_text_normalization {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "apply_text_normalization".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.eagerness {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "eagerness".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.client_session_id {
            ws_options
                .query_params
                .push(("client_session_id".to_string(), value.clone()));
        }
        if let Some(value) = options.enable_zrm {
            ws_options
                .query_params
                .push(("enable_zrm".to_string(), value.to_string()));
        }
        let full_url = crate::core::websocket::ws_url(url, "/v1/translation/realtime");
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
        message: &crate::api::types::SendTranslateStreamMessage,
    ) -> Result<(), crate::error::ApiError> {
        self.socket.send_json(message).await
    }

    /// Receives the next message, or `None` once the server closes or the stream ends.
    pub async fn recv(
        &mut self,
    ) -> Option<Result<TranslationServerMessage, crate::error::ApiError>> {
        use crate::core::websocket::WebSocketMessage;
        match self.incoming.recv().await {
            Some(Ok(WebSocketMessage::Text(raw))) => Some(
                serde_json::from_str::<TranslationServerMessage>(&raw)
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

/// Opens the `v1_translation_realtime` channel. The root client holds one; `new` builds one by hand.
pub struct TranslationConnector {
    source: TranslationConnectorSource,
}

enum TranslationConnectorSource {
    Fixed {
        base_url: String,
        auth_header: Option<String>,
    },
    #[allow(dead_code)]
    Shared {
        config: std::sync::Arc<crate::core::config_manager::ConfigManager>,
    },
}

impl TranslationConnector {
    /// A connector that opens the channel at `base_url`, sending `auth_header` as the
    /// `xi-api-key` header when one is given.
    pub fn new(base_url: String, auth_header: Option<String>) -> Self {
        Self {
            source: TranslationConnectorSource::Fixed {
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
            source: TranslationConnectorSource::Shared { config },
        }
    }

    /// Opens the `v1_translation_realtime` channel.
    pub async fn realtime(
        &self,
        options: &TranslationConnectOptions,
    ) -> Result<TranslationClient, crate::error::ApiError> {
        self.realtime_with_options(options, crate::core::websocket::WebSocketOptions::default())
            .await
    }

    /// Opens the `v1_translation_realtime` channel with the caller's transport options: reconnection,
    /// timeouts, and extra handshake headers or query parameters.
    ///
    /// The socket URL is derived from the configured base URL when this connector came from the
    /// root client, so a later `set_base_url` reaches it.
    pub async fn realtime_with_options(
        &self,
        options: &TranslationConnectOptions,
        ws_options: crate::core::websocket::WebSocketOptions,
    ) -> Result<TranslationClient, crate::error::ApiError> {
        let mut ws_options = ws_options;
        if let Some(value) = &options.authorization {
            ws_options
                .query_params
                .push(("authorization".to_string(), value.clone()));
        }
        if let Some(value) = &options.target_language {
            ws_options
                .query_params
                .push(("target_language".to_string(), value.clone()));
        }
        if let Some(value) = &options.source_language {
            ws_options
                .query_params
                .push(("source_language".to_string(), value.clone()));
        }
        if let Some(value) = &options.voice_id {
            ws_options
                .query_params
                .push(("voice_id".to_string(), value.clone()));
        }
        if let Some(value) = &options.input_format {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "input_format".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.output_format {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "output_format".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.tts_model_id {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "tts_model_id".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.apply_text_normalization {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "apply_text_normalization".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.eagerness {
            if let Ok(serialized) = serde_json::to_string(value) {
                ws_options.query_params.push((
                    "eagerness".to_string(),
                    serialized.trim_matches('"').to_string(),
                ));
            }
        }
        if let Some(value) = &options.client_session_id {
            ws_options
                .query_params
                .push(("client_session_id".to_string(), value.clone()));
        }
        if let Some(value) = options.enable_zrm {
            ws_options
                .query_params
                .push(("enable_zrm".to_string(), value.to_string()));
        }
        let base_url = match &self.source {
            TranslationConnectorSource::Fixed {
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
            TranslationConnectorSource::Shared { config } => {
                let config = config.get_v_1_translation_realtime_config();
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
        let url = crate::core::websocket::ws_url(&base_url, "/v1/translation/realtime");
        TranslationClient::open(&url, ws_options).await
    }
}
