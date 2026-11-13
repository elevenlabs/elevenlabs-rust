use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rand::Rng;
use serde::Serialize;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use tokio::sync::{mpsc, Mutex, Notify};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        http::{HeaderName, HeaderValue},
        protocol::Message,
    },
    MaybeTlsStream, WebSocketStream,
};

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsSource = SplitStream<WsStream>;

fn swap_http_scheme(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = url.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        url.to_string()
    }
}

/// Builds a socket URL from an HTTP base URL and a channel path.
///
/// The scheme swap is not cosmetic: tungstenite rejects an http:// URL outright, so a client
/// handed the SDK's own base_url unchanged would fail every connect. A base with any other
/// scheme -- including one already ws:// -- is passed through.
pub fn ws_url(base_url: &str, path: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let swapped = if let Some(rest) = base.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = base.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        base.to_string()
    };
    format!("{swapped}{path}")
}

/// Why a connection closed. code and reason are absent when the peer went away without
/// sending a Close frame, which is a normal thing for a network to do.
#[derive(Debug, Clone)]
pub struct DisconnectInfo {
    pub code: Option<u16>,
    pub reason: Option<String>,
}

/// A frame as it reaches the caller. Ping/Pong are handled by the transport and never surface --
/// a caller cannot do anything useful with them.
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    Text(String),
    Binary(Vec<u8>),
    Close(DisconnectInfo),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

type HandshakeHeadersFuture =
    Pin<Box<dyn Future<Output = Result<HashMap<String, String>, crate::error::ApiError>> + Send>>;

/// Headers worked out at handshake time rather than once up front.
///
/// Called before the first connect and before every reconnect, so a credential that expires --
/// an OAuth token -- is fetched fresh for each attempt instead of replayed stale.
#[derive(Clone)]
pub struct HandshakeHeaders(Arc<dyn Fn() -> HandshakeHeadersFuture + Send + Sync>);

impl HandshakeHeaders {
    pub fn new<F, Fut>(provider: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<HashMap<String, String>, crate::error::ApiError>>
            + Send
            + 'static,
    {
        Self(Arc::new(move || Box::pin(provider())))
    }

    async fn resolve(&self) -> Result<HashMap<String, String>, crate::error::ApiError> {
        (self.0)().await
    }
}

impl fmt::Debug for HandshakeHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HandshakeHeaders(..)")
    }
}

#[derive(Debug, Clone)]
pub struct WebSocketOptions {
    /// Stamped on the HTTP upgrade request. An empty value removes that header, in any casing,
    /// including one the SDK would add: an empty `Authorization` sends no credential and makes
    /// no token request.
    pub headers: HashMap<String, String>,
    /// Stamped on the HTTP upgrade request after `headers`, resolved again on every attempt.
    pub handshake_headers: Option<HandshakeHeaders>,
    /// Appended to the handshake URL. Carried separately from the URL so a credential can be added
    /// without the caller doing its own string surgery.
    pub query_params: Vec<(String, String)>,
    pub connection_timeout: Duration,
    /// Whether a dropped socket is re-established. On by default: a long-lived socket on a real
    /// network WILL drop, and a caller who has to write the retry loop themselves gets it wrong.
    pub reconnect: bool,
    pub max_reconnect_attempts: u32,
    pub min_reconnect_delay: Duration,
    pub max_reconnect_delay: Duration,
    pub reconnect_delay_growth_factor: f64,
    /// Cap on messages held while the socket is down. Bounded on purpose -- an unbounded queue
    /// turns a network outage into unbounded memory growth.
    pub max_queued_messages: usize,
}

impl Default for WebSocketOptions {
    fn default() -> Self {
        Self {
            headers: HashMap::new(),
            handshake_headers: None,
            query_params: Vec::new(),
            connection_timeout: Duration::from_secs(10),
            reconnect: true,
            max_reconnect_attempts: 5,
            min_reconnect_delay: Duration::from_secs(1),
            max_reconnect_delay: Duration::from_secs(10),
            reconnect_delay_growth_factor: 1.3,
            max_queued_messages: 1000,
        }
    }
}

impl WebSocketOptions {
    /// Whether the caller already set `name` in `headers`, in any casing, an empty value included.
    pub fn supplies_header(&self, name: &str) -> bool {
        self.headers
            .keys()
            .any(|existing| existing.eq_ignore_ascii_case(name))
    }

    /// Sets `name`, replacing a value already set under any casing of it.
    pub fn set_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        self.headers
            .retain(|existing, _| !existing.eq_ignore_ascii_case(&name));
        self.headers.insert(name, value.into());
    }
}

enum ReconnectOutcome {
    Reconnected,
    Closed,
}

/// The Arc-wrapped state the read loop and the reconnect path both need.
struct ReconnectContext {
    sink: Arc<Mutex<Option<WsSink>>>,
    state: Arc<Mutex<WebSocketState>>,
    close_notify: Arc<Notify>,
    incoming_tx: mpsc::UnboundedSender<Result<WebSocketMessage, crate::error::ApiError>>,
    url: String,
    options: WebSocketOptions,
    text_queue: Arc<Mutex<VecDeque<String>>>,
    binary_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

/// The transport a generated channel client is built on.
///
/// Receiving is a channel rather than a method on this type: a socket delivers frames whenever the
/// server sends them, and a read loop owning the stream half is what lets send stay callable
/// while a receive is outstanding. The caller gets the receiver back from connect.
pub struct WebSocketClient {
    url: String,
    options: WebSocketOptions,
    sink: Arc<Mutex<Option<WsSink>>>,
    state: Arc<Mutex<WebSocketState>>,
    close_notify: Arc<Notify>,
    text_queue: Arc<Mutex<VecDeque<String>>>,
    binary_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

impl WebSocketClient {
    /// Opens the socket and spawns the read loop.
    ///
    /// The handshake is bounded by options.connection_timeout: without it a server that accepts
    /// the TCP connection and never completes the upgrade would hang the caller forever.
    pub async fn connect(
        url: &str,
        options: WebSocketOptions,
    ) -> Result<
        (
            Self,
            mpsc::UnboundedReceiver<Result<WebSocketMessage, crate::error::ApiError>>,
        ),
        crate::error::ApiError,
    > {
        let url = swap_http_scheme(url);
        let url = url.as_str();
        let (incoming_tx, incoming_rx) = mpsc::unbounded_channel();
        let full_url = Self::apply_query_params(url, &options.query_params)?;
        let sink = Arc::new(Mutex::new(None));
        let read = Self::open_connection(&full_url, &options, &sink).await?;

        let client = Self {
            url: url.to_string(),
            options: options.clone(),
            sink: Arc::clone(&sink),
            state: Arc::new(Mutex::new(WebSocketState::Open)),
            close_notify: Arc::new(Notify::new()),
            text_queue: Arc::new(Mutex::new(VecDeque::new())),
            binary_queue: Arc::new(Mutex::new(VecDeque::new())),
        };

        let ctx = ReconnectContext {
            sink,
            state: Arc::clone(&client.state),
            close_notify: Arc::clone(&client.close_notify),
            incoming_tx,
            url: client.url.clone(),
            options: client.options.clone(),
            text_queue: Arc::clone(&client.text_queue),
            binary_queue: Arc::clone(&client.binary_queue),
        };

        tokio::spawn(async move {
            Self::read_loop(read, ctx).await;
        });

        Ok((client, incoming_rx))
    }

    /// The handshake, shared by the first connect and every reconnect so the two cannot drift --
    /// a reconnect that forgot the auth headers would be a very quiet bug.
    async fn open_connection(
        full_url: &str,
        options: &WebSocketOptions,
        sink: &Arc<Mutex<Option<WsSink>>>,
    ) -> Result<WsSource, crate::error::ApiError> {
        let mut request = full_url.into_client_request().map_err(|err| {
            crate::error::ApiError::WebSocketError(format!("invalid websocket url: {err}"))
        })?;

        let mut headers = options.headers.clone();
        if let Some(provider) = &options.handshake_headers {
            for (name, value) in provider.resolve().await? {
                headers.retain(|existing, _| !existing.eq_ignore_ascii_case(&name));
                headers.insert(name, value);
            }
        }
        headers.retain(|_, value| !value.is_empty());

        for (name, value) in &headers {
            let header_name = HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| crate::error::ApiError::InvalidHeader)?;
            let header_value =
                HeaderValue::from_str(value).map_err(|_| crate::error::ApiError::InvalidHeader)?;
            request.headers_mut().insert(header_name, header_value);
        }

        let (stream, _response) =
            tokio::time::timeout(options.connection_timeout, connect_async(request))
                .await
                .map_err(|_| {
                    crate::error::ApiError::WebSocketError(format!(
                        "websocket handshake timed out after {:?}",
                        options.connection_timeout
                    ))
                })?
                .map_err(|err| {
                    crate::error::ApiError::WebSocketError(format!(
                        "websocket handshake failed: {err}"
                    ))
                })?;

        let (write, read) = stream.split();
        *sink.lock().await = Some(write);
        Ok(read)
    }

    /// Sends a value as a JSON text frame.
    pub async fn send_json<T: Serialize>(&self, message: &T) -> Result<(), crate::error::ApiError> {
        let payload =
            serde_json::to_string(message).map_err(crate::error::ApiError::Serialization)?;
        self.send_raw(payload).await
    }

    /// Sends a text frame, or queues it if the socket is down.
    ///
    /// The sink lock is held across the whole operation. That is what preserves ordering: the
    /// queue is flushed under the same lock after a reconnect, so a queued message can never be
    /// overtaken by a send issued after it.
    pub async fn send_raw(&self, message: String) -> Result<(), crate::error::ApiError> {
        let mut guard = self.sink.lock().await;
        match guard.as_mut() {
            Some(sink) => sink
                .send(Message::Text(message.into()))
                .await
                .map_err(|err| {
                    crate::error::ApiError::WebSocketError(format!("websocket send failed: {err}"))
                }),
            None => {
                if Self::is_terminal(&self.state).await {
                    return Err(crate::error::ApiError::WebSocketError(
                        "websocket is closed".to_string(),
                    ));
                }
                let mut queue = self.text_queue.lock().await;
                if queue.len() >= self.options.max_queued_messages {
                    return Err(crate::error::ApiError::WebSocketError(
                        "websocket send queue is full".to_string(),
                    ));
                }
                queue.push_back(message);
                Ok(())
            }
        }
    }

    /// Sends a binary frame, or queues it if the socket is down. See send_raw on ordering.
    pub async fn send_binary(&self, data: &[u8]) -> Result<(), crate::error::ApiError> {
        let mut guard = self.sink.lock().await;
        match guard.as_mut() {
            Some(sink) => sink
                .send(Message::Binary(data.to_vec().into()))
                .await
                .map_err(|err| {
                    crate::error::ApiError::WebSocketError(format!("websocket send failed: {err}"))
                }),
            None => {
                if Self::is_terminal(&self.state).await {
                    return Err(crate::error::ApiError::WebSocketError(
                        "websocket is closed".to_string(),
                    ));
                }
                let mut queue = self.binary_queue.lock().await;
                if queue.len() >= self.options.max_queued_messages {
                    return Err(crate::error::ApiError::WebSocketError(
                        "websocket send queue is full".to_string(),
                    ));
                }
                queue.push_back(data.to_vec());
                Ok(())
            }
        }
    }

    /// How many messages are waiting because the socket is down.
    ///
    /// Exposed because a caller who queues while disconnected has no other way to know whether
    /// their sends went out or are still pending.
    pub async fn queued_message_count(&self) -> usize {
        self.text_queue.lock().await.len() + self.binary_queue.lock().await.len()
    }

    /// Closes the socket. Idempotent: a caller cannot reliably know whether the server closed
    /// first, so closing twice must not be an error.
    pub async fn close(&self) -> Result<(), crate::error::ApiError> {
        {
            let mut state = self.state.lock().await;
            if matches!(*state, WebSocketState::Closing | WebSocketState::Closed) {
                return Ok(());
            }
            *state = WebSocketState::Closing;
        }

        // Stop the read loop before closing the sink, or it would see the stream end and try to
        // reconnect the socket the caller just asked to close.
        self.close_notify.notify_waiters();

        if let Some(mut sink) = self.sink.lock().await.take() {
            let _ = sink.send(Message::Close(None)).await;
            let _ = sink.close().await;
        }

        *self.state.lock().await = WebSocketState::Closed;
        Ok(())
    }

    pub async fn state(&self) -> WebSocketState {
        *self.state.lock().await
    }

    async fn is_terminal(state: &Mutex<WebSocketState>) -> bool {
        matches!(
            *state.lock().await,
            WebSocketState::Closing | WebSocketState::Closed
        )
    }

    /// Flushes queued messages, text before binary.
    ///
    /// A message that fails to send goes back to the FRONT of its queue rather than being dropped:
    /// the socket has evidently gone again, and losing the message would be worse than retrying it
    /// on the next reconnect.
    async fn flush_queues(ctx: &ReconnectContext) {
        let mut guard = ctx.sink.lock().await;
        let sink = match guard.as_mut() {
            Some(sink) => sink,
            None => return,
        };

        {
            let mut queue = ctx.text_queue.lock().await;
            while let Some(message) = queue.pop_front() {
                if sink
                    .send(Message::Text(message.clone().into()))
                    .await
                    .is_err()
                {
                    queue.push_front(message);
                    return;
                }
            }
        }
        {
            let mut queue = ctx.binary_queue.lock().await;
            while let Some(message) = queue.pop_front() {
                if sink
                    .send(Message::Binary(message.clone().into()))
                    .await
                    .is_err()
                {
                    queue.push_front(message);
                    return;
                }
            }
        }
    }

    /// Exponential backoff with jitter.
    ///
    /// The jitter is not decoration: without it every client disconnected by the same server
    /// outage retries in lockstep and the reconnect storm is what keeps the server down.
    fn reconnect_delay(options: &WebSocketOptions, attempt: u32) -> Duration {
        let base = if attempt <= 1 {
            options.min_reconnect_delay.as_millis() as f64
        } else {
            let grown = options.min_reconnect_delay.as_millis() as f64
                * options
                    .reconnect_delay_growth_factor
                    .powi((attempt - 1) as i32);
            grown.min(options.max_reconnect_delay.as_millis() as f64)
        };
        let jitter = rand::rng().random_range(0.0..=0.25);
        Duration::from_millis((base * (1.0 + jitter)) as u64)
    }

    /// Resets the attempt counter once a connection has held for a while.
    ///
    /// Without this a socket that drops once an hour eventually exhausts its budget and stops
    /// reconnecting, even though it was never actually unhealthy.
    fn maybe_reset_attempts(last_connected_at: &Option<tokio::time::Instant>, attempts: &mut u32) {
        if let Some(connected_at) = last_connected_at {
            if connected_at.elapsed() >= Duration::from_secs(5) {
                *attempts = 0;
            }
        }
    }

    async fn try_reconnect(
        read: &mut WsSource,
        attempts: &mut u32,
        last_connected_at: &mut Option<tokio::time::Instant>,
        ctx: &ReconnectContext,
    ) -> Result<ReconnectOutcome, crate::error::ApiError> {
        if !ctx.options.reconnect {
            *ctx.state.lock().await = WebSocketState::Closed;
            return Err(crate::error::ApiError::WebSocketError(
                "websocket reconnect is disabled".to_string(),
            ));
        }

        let full_url = Self::apply_query_params(&ctx.url, &ctx.options.query_params)?;
        let mut last_error: Option<crate::error::ApiError> = None;

        loop {
            if *attempts >= ctx.options.max_reconnect_attempts {
                *ctx.state.lock().await = WebSocketState::Closed;
                return Err(last_error.unwrap_or_else(|| {
                    crate::error::ApiError::WebSocketError(format!(
                        "websocket reconnect gave up after {} attempts",
                        ctx.options.max_reconnect_attempts
                    ))
                }));
            }

            *attempts += 1;
            let closed = ctx.close_notify.notified();
            tokio::pin!(closed);
            {
                let mut state = ctx.state.lock().await;
                if matches!(*state, WebSocketState::Closing | WebSocketState::Closed) {
                    return Ok(ReconnectOutcome::Closed);
                }
                *state = WebSocketState::Connecting;
            }
            tokio::select! {
                _ = &mut closed => return Ok(ReconnectOutcome::Closed),
                _ = tokio::time::sleep(Self::reconnect_delay(&ctx.options, *attempts)) => {}
            }
            if Self::is_terminal(&ctx.state).await {
                return Ok(ReconnectOutcome::Closed);
            }

            match Self::open_connection(&full_url, &ctx.options, &ctx.sink).await {
                Ok(new_read) => {
                    if Self::is_terminal(&ctx.state).await {
                        if let Some(mut sink) = ctx.sink.lock().await.take() {
                            let _ = sink.send(Message::Close(None)).await;
                            let _ = sink.close().await;
                        }
                        return Ok(ReconnectOutcome::Closed);
                    }
                    *read = new_read;
                    *last_connected_at = Some(tokio::time::Instant::now());
                    *ctx.state.lock().await = WebSocketState::Open;
                    Self::flush_queues(ctx).await;
                    return Ok(ReconnectOutcome::Reconnected);
                }
                Err(err) => last_error = Some(err),
            }
        }
    }

    async fn read_loop(mut read: WsSource, ctx: ReconnectContext) {
        let mut attempts: u32 = 0;
        let mut last_connected_at: Option<tokio::time::Instant> = None;

        loop {
            let frame = tokio::select! {
                // A caller-initiated close wins over anything still arriving, so the loop does not
                // reconnect a socket that was deliberately shut.
                _ = ctx.close_notify.notified() => break,
                frame = read.next() => frame,
            };

            match frame {
                Some(Ok(Message::Text(text))) => {
                    if ctx
                        .incoming_tx
                        .send(Ok(WebSocketMessage::Text(text.to_string())))
                        .is_err()
                    {
                        break;
                    }
                }
                Some(Ok(Message::Binary(bytes))) => {
                    if ctx
                        .incoming_tx
                        .send(Ok(WebSocketMessage::Binary(bytes.to_vec())))
                        .is_err()
                    {
                        break;
                    }
                }
                Some(Ok(Message::Close(frame))) => {
                    let info = DisconnectInfo {
                        code: frame.as_ref().map(|f| u16::from(f.code)),
                        reason: frame
                            .as_ref()
                            .map(|f| f.reason.to_string())
                            .filter(|reason| !reason.is_empty()),
                    };
                    let _ = ctx.incoming_tx.send(Ok(WebSocketMessage::Close(info)));
                    break;
                }
                // Ping/Pong are the transport's business. tungstenite answers a Ping itself, so
                // forwarding them would only give the caller frames it must ignore.
                Some(Ok(_)) => {}
                // Both a transport error and a clean end of stream mean the socket is gone, and
                // reconnecting is the same response to either.
                Some(Err(_)) | None => {
                    ctx.sink.lock().await.take();
                    Self::maybe_reset_attempts(&last_connected_at, &mut attempts);
                    match Self::try_reconnect(
                        &mut read,
                        &mut attempts,
                        &mut last_connected_at,
                        &ctx,
                    )
                    .await
                    {
                        Ok(ReconnectOutcome::Reconnected) => {}
                        Ok(ReconnectOutcome::Closed) => break,
                        Err(err) => {
                            let _ = ctx.incoming_tx.send(Err(err));
                            break;
                        }
                    }
                }
            }
        }

        *ctx.state.lock().await = WebSocketState::Closed;
        ctx.sink.lock().await.take();
    }

    fn apply_query_params(
        url: &str,
        params: &[(String, String)],
    ) -> Result<String, crate::error::ApiError> {
        if params.is_empty() {
            return Ok(url.to_string());
        }

        let mut parsed = url::Url::parse(url).map_err(|err| {
            crate::error::ApiError::WebSocketError(format!("invalid websocket url {url}: {err}"))
        })?;
        let appended = params
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}={}",
                    urlencoding::encode(key),
                    urlencoding::encode(value)
                )
            })
            .collect::<Vec<_>>()
            .join("&");
        let query = match parsed.query() {
            Some(existing) if !existing.is_empty() => format!("{existing}&{appended}"),
            _ => appended,
        };
        parsed.set_query(Some(&query));
        Ok(parsed.to_string())
    }
}
