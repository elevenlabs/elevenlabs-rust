/// Query parameters sent when opening the `v1_convai_conversation` channel.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AgentsConnectOptions {
    pub agent_id: Option<String>,
}

/// A JSON message the server sends on the `v1_convai_conversation` channel.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum AgentsServerMessage {
    SubscribeEvent(crate::api::types::SubscribeEvent),
    /// Unknown or new server message type not yet supported by this SDK version.
    Unknown(serde_json::Value),
}

/// Picks the variant whose re-serialized keys overlap the frame's keys the most; a tie keeps the
/// earlier variant, and a frame no variant matches, or one that is not a non-empty object, is
/// `Unknown`.
impl<'de> serde::Deserialize<'de> for AgentsServerMessage {
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

        if let Ok(v) = serde_json::from_value::<crate::api::types::SubscribeEvent>(value.clone()) {
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
                    best_variant = Some(Self::SubscribeEvent(v));
                }
            }
        }

        let _ = best_score;
        Ok(best_variant.unwrap_or(Self::Unknown(value)))
    }
}

/// An open connection to the `v1_convai_conversation` channel.
pub struct AgentsClient {
    socket: crate::core::websocket::WebSocketClient,
    incoming: tokio::sync::mpsc::UnboundedReceiver<
        Result<crate::core::websocket::WebSocketMessage, crate::error::ApiError>,
    >,
}

impl AgentsClient {
    /// Opens the channel at `url`, the API's base URL. The channel's address `/v1/convai/conversation`
    /// is appended to it.
    pub async fn conversation(
        url: &str,
        api_key: Option<&str>,
        options: &AgentsConnectOptions,
    ) -> Result<Self, crate::error::ApiError> {
        let mut ws_options = crate::core::websocket::WebSocketOptions::default();
        if let Some(value) = api_key {
            ws_options.set_header("xi-api-key", value);
        }
        if let Some(value) = &options.agent_id {
            ws_options
                .query_params
                .push(("agent_id".to_string(), value.clone()));
        }
        let full_url = crate::core::websocket::ws_url(url, "/v1/convai/conversation");
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
        message: &crate::api::types::PublishEvent,
    ) -> Result<(), crate::error::ApiError> {
        self.socket.send_json(message).await
    }

    /// Receives the next message, or `None` once the server closes or the stream ends.
    pub async fn recv(&mut self) -> Option<Result<AgentsServerMessage, crate::error::ApiError>> {
        use crate::core::websocket::WebSocketMessage;
        match self.incoming.recv().await {
            Some(Ok(WebSocketMessage::Text(raw))) => Some(
                serde_json::from_str::<AgentsServerMessage>(&raw)
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

/// Opens the `v1_convai_conversation` channel. The root client holds one; `new` builds one by hand.
pub struct AgentsConnector {
    source: AgentsConnectorSource,
}

enum AgentsConnectorSource {
    Fixed {
        base_url: String,
        auth_header: Option<String>,
    },
    #[allow(dead_code)]
    Shared {
        config: std::sync::Arc<crate::core::config_manager::ConfigManager>,
    },
}

impl AgentsConnector {
    /// A connector that opens the channel at `base_url`, sending `auth_header` as the
    /// `xi-api-key` header when one is given.
    pub fn new(base_url: String, auth_header: Option<String>) -> Self {
        Self {
            source: AgentsConnectorSource::Fixed {
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
            source: AgentsConnectorSource::Shared { config },
        }
    }

    /// Opens the `v1_convai_conversation` channel.
    pub async fn conversation(
        &self,
        options: &AgentsConnectOptions,
    ) -> Result<AgentsClient, crate::error::ApiError> {
        self.conversation_with_options(options, crate::core::websocket::WebSocketOptions::default())
            .await
    }

    /// Opens the `v1_convai_conversation` channel with the caller's transport options: reconnection,
    /// timeouts, and extra handshake headers or query parameters.
    ///
    /// The socket URL is derived from the configured base URL when this connector came from the
    /// root client, so a later `set_base_url` reaches it.
    pub async fn conversation_with_options(
        &self,
        options: &AgentsConnectOptions,
        ws_options: crate::core::websocket::WebSocketOptions,
    ) -> Result<AgentsClient, crate::error::ApiError> {
        let mut ws_options = ws_options;
        if let Some(value) = &options.agent_id {
            ws_options
                .query_params
                .push(("agent_id".to_string(), value.clone()));
        }
        let base_url = match &self.source {
            AgentsConnectorSource::Fixed {
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
            AgentsConnectorSource::Shared { config } => {
                let config = config.get_v_1_convai_conversation_config();
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
                config
                    .service_url(|environment| environment.production_url())
                    .to_string()
            }
        };
        let url = crate::core::websocket::ws_url(&base_url, "/v1/convai/conversation");
        AgentsClient::open(&url, ws_options).await
    }
}
