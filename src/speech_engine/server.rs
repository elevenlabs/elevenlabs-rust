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

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    /// How long a connection has to complete the WebSocket handshake before it is dropped. Stops an
    /// unauthenticated peer from holding a socket and task open indefinitely. Defaults to 10s.
    pub handshake_timeout: Duration,
}

impl Default for SpeechEngineServerOptions {
    fn default() -> Self {
        Self {
            port: 3001,
            path: None,
            api_key: None,
            debug: false,
            disable_auth: false,
            handshake_timeout: Duration::from_secs(10),
        }
    }
}

struct ServerInner {
    options: SpeechEngineServerOptions,
    callbacks: Arc<SpeechEngineCallbacks>,
    shutdown: Notify,
    local_addr: Mutex<Option<SocketAddr>>,
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
                local_addr: Mutex::new(None),
            }),
        }
    }

    /// The address the server is bound to, once [`serve`](Self::serve) has started listening.
    /// Useful when `options.port` is 0 and the OS assigns a port.
    pub fn local_addr(&self) -> Option<SocketAddr> {
        *self.inner.local_addr.lock().unwrap()
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
        if let Ok(addr) = listener.local_addr() {
            *self.inner.local_addr.lock().unwrap() = Some(addr);
        }
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
        *self.inner.local_addr.lock().unwrap() = None;
        Ok(())
    }

    /// Signal the server to stop accepting connections and return from [`serve`](Self::serve).
    pub fn stop(&self) {
        // notify_one, not notify_waiters: it stores a permit, so a stop() that races ahead of the
        // serve loop reaching `.notified()` is not lost.
        self.inner.shutdown.notify_one();
    }
}

// The handshake callback's `Result<Response, ErrorResponse>` return type is fixed by
// tungstenite's `accept_hdr_async` contract, so the large `Err` variant is not ours to box away.
#[allow(clippy::result_large_err)]
async fn handle_connection(stream: TcpStream, api_key: Option<String>, inner: Arc<ServerInner>) {
    let disable_auth = inner.options.disable_auth;
    let path = inner.options.path.clone();
    let debug = inner.options.debug;
    let handshake_timeout = inner.options.handshake_timeout;

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
            None => {
                return Err(reject(
                    StatusCode::UNAUTHORIZED,
                    "missing authorization header",
                ))
            }
        };
        match verify_speech_engine_jwt(token, api_key.as_deref().unwrap_or("")) {
            Ok(_) => Ok(response),
            Err(_) => Err(reject(StatusCode::UNAUTHORIZED, "authorization failed")),
        }
    };

    // Bound the handshake: an unauthenticated peer that opens a socket and sends an incomplete
    // upgrade must not hold a task and file descriptor open past this deadline. On timeout the
    // future is dropped, which closes the stream.
    let accept = tokio_tungstenite::accept_hdr_async(stream, callback);
    let ws_stream = match tokio::time::timeout(handshake_timeout, accept).await {
        Ok(Ok(ws)) => ws,
        Ok(Err(e)) => {
            if debug {
                eprintln!("[SpeechEngine] rejected connection — {e}");
            }
            return;
        }
        Err(_) => {
            if debug {
                eprintln!("[SpeechEngine] dropped connection — handshake not completed in time");
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
            // handle_raw_message hands a user_transcript to the session's worker task and returns
            // at once (it does not await the transcript callback), so this read loop stays free to
            // process ping/close even while a slow LLM response streams.
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
            // See drive_tungstenite_session: handle_raw_message enqueues transcript work on the
            // session's worker and returns promptly, so this loop keeps handling ping/close while a
            // response streams.
            Ok(AxumMessage::Text(text)) => session.handle_raw_message(&text).await,
            Ok(AxumMessage::Close(_)) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }

    session.handle_disconnect().await;
    writer.abort();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speech_engine::session::SpeechEngineCallbacks;
    use tokio::io::AsyncReadExt;

    fn test_server(options: SpeechEngineServerOptions) -> SpeechEngineServer {
        SpeechEngineServer::new(options, Arc::new(SpeechEngineCallbacks::default()))
    }

    async fn bound_addr(server: &SpeechEngineServer) -> SocketAddr {
        loop {
            if let Some(addr) = server.local_addr() {
                return addr;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    // stop() must not lose its signal even when it races ahead of the serve loop (notify_one).
    #[tokio::test]
    async fn stop_before_serve_loop_still_returns() {
        let server = test_server(SpeechEngineServerOptions {
            port: 0,
            api_key: Some("sk_test".to_string()),
            ..Default::default()
        });
        let serving = server.clone();
        let handle = tokio::spawn(async move { serving.serve().await });

        server.stop();

        let joined = tokio::time::timeout(Duration::from_secs(5), handle)
            .await
            .expect("serve() should return promptly after stop()");
        joined
            .expect("serve task panicked")
            .expect("serve returned an error");
    }

    // An accepted connection that never completes the WebSocket handshake is dropped after the
    // handshake deadline, so a silent peer cannot pin a socket and task indefinitely.
    #[tokio::test]
    async fn connection_that_never_handshakes_is_dropped() {
        let server = test_server(SpeechEngineServerOptions {
            port: 0,
            api_key: Some("sk_test".to_string()),
            handshake_timeout: Duration::from_millis(300),
            ..Default::default()
        });
        let serving = server.clone();
        tokio::spawn(async move { serving.serve().await });

        let addr = tokio::time::timeout(Duration::from_secs(5), bound_addr(&server))
            .await
            .expect("server should bind");
        let mut stream = tokio::net::TcpStream::connect(addr)
            .await
            .expect("connect should succeed");

        // Send nothing. Once the deadline passes the server drops the connection, so the read
        // returns 0 bytes (EOF).
        let mut buf = [0u8; 1];
        let n = tokio::time::timeout(Duration::from_secs(5), stream.read(&mut buf))
            .await
            .expect("read should not hang past the handshake deadline")
            .expect("read should succeed");
        assert_eq!(
            n, 0,
            "server should drop a connection that never handshakes"
        );

        server.stop();
    }
}
