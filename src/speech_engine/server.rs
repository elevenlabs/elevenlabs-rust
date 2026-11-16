//! WebSocket server(s) that produce a [`SpeechEngineSession`] for each inbound connection from the
//! ElevenLabs Speech Engine API.
//!
//! Two integrations are offered, both verifying the `X-ElevenLabs-Speech-Engine-Authorization` JWT
//! on the handshake before a session is wired up:
//!
//! * [`SpeechEngineServer`] — a standalone server built on `tokio-tungstenite`. It owns its own
//!   listener; call [`SpeechEngineServer::serve`] from a long-running task.
//! * [`speech_engine_router`] (behind the `axum` feature) — an [`axum::Router`] you merge into an
//!   existing server, so the Speech Engine endpoint shares a port and middleware with the rest of
//!   your app.
//!
//! Hand-written (see `.fernignore`), mirroring `SpeechEngineServer` in the Python/TS/PHP SDKs.

use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Notify;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;

use super::error::SpeechEngineError;
use super::jwt::verify_speech_engine_jwt;
use super::session::{OutboundFrame, SpeechEngineCallbacks, SpeechEngineSession};

const AUTH_HEADER: &str = "X-ElevenLabs-Speech-Engine-Authorization";

/// Configuration for a Speech Engine server.
pub struct SpeechEngineServerOptions {
    /// Port to listen on. Defaults to 3001.
    pub port: u16,
    /// Only accept connections whose request path matches this value, when set.
    pub path: Option<String>,
    /// The account API key used to verify inbound connections. Falls back to the
    /// `ELEVENLABS_API_KEY` environment variable at [`serve`](SpeechEngineServer::serve) time.
    pub api_key: Option<String>,
    /// Emit `[SpeechEngine]` diagnostics to stderr.
    pub debug: bool,
    /// Skip JWT verification. **Insecure** — only enable behind an IP allowlist scoped to
    /// ElevenLabs' egress ranges.
    pub disable_auth: bool,
}

impl Default for SpeechEngineServerOptions {
    fn default() -> Self {
        Self {
            port: 3001,
            path: None,
            api_key: None,
            debug: false,
            disable_auth: false,
        }
    }
}

struct ServerInner {
    options: SpeechEngineServerOptions,
    callbacks: Arc<SpeechEngineCallbacks>,
    shutdown: Notify,
}

/// Standalone WebSocket server for inbound Speech Engine connections.
#[derive(Clone)]
pub struct SpeechEngineServer {
    inner: Arc<ServerInner>,
}

impl SpeechEngineServer {
    /// Build a server from its options and the callbacks wired onto every session.
    pub fn new(options: SpeechEngineServerOptions, callbacks: Arc<SpeechEngineCallbacks>) -> Self {
        Self {
            inner: Arc::new(ServerInner {
                options,
                callbacks,
                shutdown: Notify::new(),
            }),
        }
    }

    fn log(&self, msg: &str) {
        if self.inner.options.debug {
            eprintln!("[SpeechEngine] {msg}");
        }
    }

    /// Start the server. Runs until [`stop`](Self::stop) is called.
    pub async fn serve(&self) -> Result<(), SpeechEngineError> {
        let api_key = self
            .inner
            .options
            .api_key
            .clone()
            .or_else(|| std::env::var("ELEVENLABS_API_KEY").ok())
            .filter(|k| !k.is_empty());

        if api_key.is_none() && !self.inner.options.disable_auth {
            return Err(SpeechEngineError::Protocol(
                "SpeechEngineServer requires an API key to verify incoming connections. Set \
                 options.api_key or the ELEVENLABS_API_KEY environment variable. To run without \
                 authentication, set disable_auth = true — but only behind an IP allowlist."
                    .to_string(),
            ));
        }
        if self.inner.options.disable_auth {
            eprintln!(
                "[SpeechEngine] authentication is disabled — incoming connections will NOT be \
                 verified. Make sure the server is protected by an IP allowlist restricting \
                 traffic to ElevenLabs."
            );
        }

        let listener = tokio::net::TcpListener::bind(("0.0.0.0", self.inner.options.port))
            .await
            .map_err(|e| SpeechEngineError::WebSocket(format!("failed to bind: {e}")))?;
        self.log(&format!(
            "speech engine server listening on port {}",
            self.inner.options.port
        ));

        loop {
            tokio::select! {
                _ = self.inner.shutdown.notified() => break,
                accepted = listener.accept() => {
                    let (stream, _addr) = accepted
                        .map_err(|e| SpeechEngineError::WebSocket(format!("accept failed: {e}")))?;
                    let inner = Arc::clone(&self.inner);
                    let api_key = api_key.clone();
                    tokio::spawn(async move {
                        handle_connection(stream, api_key, inner).await;
                    });
                }
            }
        }
        Ok(())
    }

    /// Signal the server to stop accepting connections and return from [`serve`](Self::serve).
    pub fn stop(&self) {
        self.inner.shutdown.notify_waiters();
    }
}

// The handshake callback's `Result<Response, ErrorResponse>` return type is fixed by
// tungstenite's `accept_hdr_async` contract, so the large `Err` variant is not ours to box away.
#[allow(clippy::result_large_err)]
async fn handle_connection(stream: TcpStream, api_key: Option<String>, inner: Arc<ServerInner>) {
    let disable_auth = inner.options.disable_auth;
    let path = inner.options.path.clone();
    let debug = inner.options.debug;

    let callback = move |req: &Request, response: Response| -> Result<Response, ErrorResponse> {
        if let Some(expected) = &path {
            if req.uri().path() != expected {
                return Err(reject(StatusCode::NOT_FOUND, "not found"));
            }
        }
        if disable_auth {
            return Ok(response);
        }
        let token = req
            .headers()
            .get(AUTH_HEADER)
            .and_then(|h| h.to_str().ok())
            .filter(|t| !t.is_empty());
        let token = match token {
            Some(t) => t,
            None => return Err(reject(StatusCode::UNAUTHORIZED, "missing authorization header")),
        };
        match verify_speech_engine_jwt(token, api_key.as_deref().unwrap_or("")) {
            Ok(_) => Ok(response),
            Err(_) => Err(reject(StatusCode::UNAUTHORIZED, "authorization failed")),
        }
    };

    let ws_stream = match tokio_tungstenite::accept_hdr_async(stream, callback).await {
        Ok(ws) => ws,
        Err(e) => {
            if debug {
                eprintln!("[SpeechEngine] rejected connection — {e}");
            }
            return;
        }
    };
    if debug {
        eprintln!("[SpeechEngine] verified connection, creating session");
    }
    drive_tungstenite_session(ws_stream, Arc::clone(&inner.callbacks), debug).await;
}

fn reject(status: StatusCode, body: &str) -> ErrorResponse {
    let mut response = ErrorResponse::new(Some(body.to_string()));
    *response.status_mut() = status;
    response
}

/// Drive a session over a `tokio-tungstenite` connection: read inbound text frames into the
/// session and write the session's outbound frames back. Returns when either side closes.
async fn drive_tungstenite_session(
    ws: tokio_tungstenite::WebSocketStream<TcpStream>,
    callbacks: Arc<SpeechEngineCallbacks>,
    debug: bool,
) {
    let (mut write, mut read) = ws.split();
    let (session, mut outbound_rx) = SpeechEngineSession::new(callbacks, debug);

    // A dedicated writer task flushes outbound frames concurrently, so streamed responses reach
    // the API as they are produced rather than only after the transcript handler returns.
    let writer = tokio::spawn(async move {
        while let Some(frame) = outbound_rx.recv().await {
            match frame {
                OutboundFrame::Text(text) => {
                    if write.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
                OutboundFrame::Close => {
                    let _ = write.send(Message::Close(None)).await;
                    break;
                }
            }
        }
    });

    while let Some(frame) = read.next().await {
        match frame {
            Ok(Message::Text(text)) => session.handle_raw_message(text.as_str()).await,
            Ok(Message::Close(_)) => break,
            // Ping/Pong/Binary are the transport's business; tungstenite answers pings itself.
            Ok(_) => {}
            Err(_) => break,
        }
    }

    session.handle_disconnect().await;
    writer.abort();
}

/// Build an [`axum::Router`] that accepts inbound Speech Engine connections at `path`.
///
/// Merge or nest it into an existing router to share a port and middleware with the rest of your
/// app. The handshake is verified exactly as [`SpeechEngineServer`] verifies it.
#[cfg(feature = "axum")]
pub fn speech_engine_router(
    path: &str,
    options: SpeechEngineServerOptions,
    callbacks: Arc<SpeechEngineCallbacks>,
) -> axum::Router {
    use axum::extract::ws::WebSocketUpgrade;
    use axum::extract::State;
    use axum::http::HeaderMap;
    use axum::response::IntoResponse;
    use axum::routing::any;

    #[derive(Clone)]
    struct AxumState {
        api_key: Option<String>,
        disable_auth: bool,
        debug: bool,
        callbacks: Arc<SpeechEngineCallbacks>,
    }

    let api_key = options
        .api_key
        .clone()
        .or_else(|| std::env::var("ELEVENLABS_API_KEY").ok())
        .filter(|k| !k.is_empty());

    if options.disable_auth {
        eprintln!(
            "[SpeechEngine] authentication is disabled on the axum router — incoming connections \
             will NOT be verified. Make sure the server is protected by an IP allowlist \
             restricting traffic to ElevenLabs."
        );
    }

    let state = AxumState {
        api_key,
        disable_auth: options.disable_auth,
        debug: options.debug,
        callbacks,
    };

    async fn handler(
        State(state): State<AxumState>,
        headers: HeaderMap,
        ws: WebSocketUpgrade,
    ) -> axum::response::Response {
        if !state.disable_auth {
            let token = headers
                .get(AUTH_HEADER)
                .and_then(|h| h.to_str().ok())
                .filter(|t| !t.is_empty());
            let ok = match token {
                Some(t) => {
                    verify_speech_engine_jwt(t, state.api_key.as_deref().unwrap_or("")).is_ok()
                }
                None => false,
            };
            if !ok {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
        let callbacks = Arc::clone(&state.callbacks);
        let debug = state.debug;
        ws.on_upgrade(move |socket| drive_axum_session(socket, callbacks, debug))
    }

    axum::Router::new()
        .route(path, any(handler))
        .with_state(state)
}

#[cfg(feature = "axum")]
async fn drive_axum_session(
    socket: axum::extract::ws::WebSocket,
    callbacks: Arc<SpeechEngineCallbacks>,
    debug: bool,
) {
    use axum::extract::ws::Message as AxumMessage;

    let (mut write, mut read) = socket.split();
    let (session, mut outbound_rx) = SpeechEngineSession::new(callbacks, debug);

    let writer = tokio::spawn(async move {
        while let Some(frame) = outbound_rx.recv().await {
            match frame {
                OutboundFrame::Text(text) => {
                    if write.send(AxumMessage::Text(text)).await.is_err() {
                        break;
                    }
                }
                OutboundFrame::Close => {
                    let _ = write.send(AxumMessage::Close(None)).await;
                    break;
                }
            }
        }
    });

    while let Some(frame) = read.next().await {
        match frame {
            Ok(AxumMessage::Text(text)) => session.handle_raw_message(&text).await,
            Ok(AxumMessage::Close(_)) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }

    session.handle_disconnect().await;
    writer.abort();
}
