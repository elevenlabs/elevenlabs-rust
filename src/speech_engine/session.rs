//! One Speech Engine conversation over a single WebSocket connection.
//!
//! The session is transport-agnostic: it writes outbound text frames to an [`mpsc`] channel and is
//! fed inbound frames through [`SpeechEngineSession::handle_raw_message`], so the same protocol
//! logic drives the bundled [`super::server::SpeechEngineServer`], the optional axum integration,
//! and any custom transport. Register work with [`SpeechEngineCallbacks`].
//!
//! Hand-written (see `.fernignore`), mirroring `SpeechEngineSession` in the Python/TS/PHP SDKs.
//! Events: `init` (conversation_id), `user_transcript` (transcript), `close`, `disconnected`,
//! `error`.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use futures::{Stream, StreamExt};
use serde_json::{json, Value};
use tokio::sync::mpsc;

use super::error::SpeechEngineError;

/// A single message in a Speech Engine conversation transcript.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationMessage {
    /// Either `"user"` or `"agent"`.
    pub role: String,
    /// The text content of the message.
    pub content: String,
}

/// A frame the session wants written to the connection. The transport drains these from the
/// receiver returned by [`SpeechEngineSession::new`].
#[derive(Debug, Clone)]
pub enum OutboundFrame {
    /// A JSON text frame to send.
    Text(String),
    /// A request to close the connection.
    Close,
}

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
type InitCb = Box<dyn Fn(Option<String>, SpeechEngineSession) -> BoxFuture + Send + Sync>;
type TranscriptCb =
    Box<dyn Fn(Vec<ConversationMessage>, SpeechEngineSession) -> BoxFuture + Send + Sync>;
type SessionCb = Box<dyn Fn(SpeechEngineSession) -> BoxFuture + Send + Sync>;
type ErrorCb = Box<dyn Fn(SpeechEngineError, SpeechEngineSession) -> BoxFuture + Send + Sync>;

/// Handlers wired onto a session. Build with [`SpeechEngineCallbacks::default`] and the `on_*`
/// combinators, which take `async` closures.
#[derive(Default)]
pub struct SpeechEngineCallbacks {
    on_init: Option<InitCb>,
    on_transcript: Option<TranscriptCb>,
    on_close: Option<SessionCb>,
    on_disconnect: Option<SessionCb>,
    on_error: Option<ErrorCb>,
}

impl SpeechEngineCallbacks {
    /// Called once the API assigns a conversation ID (the `init` message).
    pub fn on_init<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Option<String>, SpeechEngineSession) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_init = Some(Box::new(move |id, s| Box::pin(f(id, s))));
        self
    }

    /// Called for each user transcript. Reply from inside this handler with
    /// [`SpeechEngineSession::send_response`] or [`SpeechEngineSession::send_response_stream`].
    pub fn on_transcript<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Vec<ConversationMessage>, SpeechEngineSession) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_transcript = Some(Box::new(move |t, s| Box::pin(f(t, s))));
        self
    }

    /// Called when the API sends a `close` message.
    pub fn on_close<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(SpeechEngineSession) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_close = Some(Box::new(move |s| Box::pin(f(s))));
        self
    }

    /// Called once when the underlying connection drops at the transport level.
    pub fn on_disconnect<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(SpeechEngineSession) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_disconnect = Some(Box::new(move |s| Box::pin(f(s))));
        self
    }

    /// Called on a malformed inbound frame or an `error` message from the API.
    pub fn on_error<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(SpeechEngineError, SpeechEngineSession) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_error = Some(Box::new(move |e, s| Box::pin(f(e, s))));
        self
    }
}

/// One chunk of a streamed response. `From` is implemented for plain text and for
/// `serde_json::Value` (an OpenAI/Anthropic/Gemini stream event, whose text is auto-extracted), so
/// a stream of any of these can be passed to [`SpeechEngineSession::send_response_stream`].
pub struct StreamChunk(Value);

impl From<Value> for StreamChunk {
    fn from(v: Value) -> Self {
        StreamChunk(v)
    }
}
impl From<String> for StreamChunk {
    fn from(s: String) -> Self {
        StreamChunk(Value::String(s))
    }
}
impl From<&str> for StreamChunk {
    fn from(s: &str) -> Self {
        StreamChunk(Value::String(s.to_string()))
    }
}

/// A transcript turn handed to the session's worker task: the messages and the turn's event ID.
type TranscriptJob = (Vec<ConversationMessage>, Option<i64>);

struct SessionInner {
    conversation_id: Mutex<Option<String>>,
    current_event_id: Mutex<Option<i64>>,
    in_transcript_handler: AtomicBool,
    closed: AtomicBool,
    // Whether on_disconnect has fired. Tracked separately from `closed` (which is also set by
    // close() and by an inbound `close` message) so the transport-ended callback fires exactly
    // once when the transport actually ends, even after a prior close.
    disconnect_fired: AtomicBool,
    outbound: mpsc::UnboundedSender<OutboundFrame>,
    // Transcript turns are queued here and run serially on a dedicated worker task, so a slow
    // transcript handler never blocks the transport's read loop from answering ping/close frames.
    transcript_tx: mpsc::UnboundedSender<TranscriptJob>,
    callbacks: Arc<SpeechEngineCallbacks>,
    debug: bool,
}

/// A handle to one Speech Engine conversation. Cheaply cloneable; all clones share the same
/// connection state.
#[derive(Clone)]
pub struct SpeechEngineSession {
    inner: Arc<SessionInner>,
}

impl SpeechEngineSession {
    /// Build a session and the receiver its transport drains for outbound frames.
    ///
    /// Must be called from within a Tokio runtime: it spawns a worker task that runs transcript
    /// handlers off the transport's read loop. The task exits on its own once the session is
    /// dropped (it holds only a [`Weak`] reference to the session's state).
    pub fn new(
        callbacks: Arc<SpeechEngineCallbacks>,
        debug: bool,
    ) -> (Self, mpsc::UnboundedReceiver<OutboundFrame>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let (transcript_tx, mut transcript_rx) = mpsc::unbounded_channel::<TranscriptJob>();
        let inner = Arc::new(SessionInner {
            conversation_id: Mutex::new(None),
            current_event_id: Mutex::new(None),
            in_transcript_handler: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            disconnect_fired: AtomicBool::new(false),
            outbound: tx,
            transcript_tx,
            callbacks,
            debug,
        });

        // The worker holds only a Weak handle, so once the session and its transport are dropped
        // the sender drops, recv() returns None, and the task ends — no reference cycle.
        let weak = Arc::downgrade(&inner);
        tokio::spawn(async move {
            while let Some((transcript, event_id)) = transcript_rx.recv().await {
                let Some(inner) = weak.upgrade() else { break };
                let session = Self { inner };
                session.run_transcript(transcript, event_id).await;
            }
        });

        (Self { inner }, rx)
    }

    /// The conversation ID assigned by the API, available after the `init` event.
    pub fn conversation_id(&self) -> Option<String> {
        self.inner.conversation_id.lock().unwrap().clone()
    }

    /// Whether the session is still open.
    pub fn is_open(&self) -> bool {
        !self.inner.closed.load(Ordering::SeqCst)
    }

    fn log(&self, msg: &str) {
        if self.inner.debug {
            eprintln!("[SpeechEngine] {msg}");
        }
    }

    /// Parse and dispatch one inbound text frame. A malformed frame surfaces on the `error`
    /// callback rather than returning an error, so one bad message never ends the loop.
    pub async fn handle_raw_message(&self, raw: &str) {
        let msg: Value = match serde_json::from_str(raw) {
            Ok(v) => v,
            Err(e) => {
                self.emit_error(SpeechEngineError::Protocol(format!("{e}")))
                    .await;
                return;
            }
        };
        if !msg.is_object() {
            self.emit_error(SpeechEngineError::Protocol(
                "Expected a JSON object message".to_string(),
            ))
            .await;
            return;
        }
        self.handle_message(msg).await;
    }

    async fn handle_message(&self, msg: Value) {
        match msg.get("type").and_then(Value::as_str) {
            Some("init") => {
                let cid = msg
                    .get("conversation_id")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                *self.inner.conversation_id.lock().unwrap() = cid.clone();
                self.log(&format!("session initialized, conversation_id={cid:?}"));
                if let Some(cb) = &self.inner.callbacks.on_init {
                    cb(cid, self.clone()).await;
                }
            }
            Some("user_transcript") => {
                // Hand the turn to the worker task and return at once, so the read loop stays
                // free to answer ping/close while the handler runs. Turns are queued, so they run
                // in order; each carries its own event ID.
                let event_id = msg.get("event_id").and_then(Value::as_i64);
                let transcript = parse_transcript(&msg);
                let _ = self.inner.transcript_tx.send((transcript, event_id));
            }
            Some("ping") => {
                self.send_json(json!({ "type": "pong" }));
            }
            Some("close") => {
                // Fire on_close while the session is still open (so a handler can react), then
                // close the transport — mirrors the PHP/Ruby SDKs. close() is idempotent.
                if let Some(cb) = &self.inner.callbacks.on_close {
                    cb(self.clone()).await;
                }
                self.close().await;
            }
            Some("error") => {
                let message = msg
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                self.emit_error(SpeechEngineError::Protocol(message)).await;
            }
            // Unknown types are ignored for forward compatibility.
            _ => {}
        }
    }

    /// Send a plain-text LLM response back for TTS synthesis. Only valid inside an
    /// `on_transcript` handler; called elsewhere it logs and does nothing. Errors if the session
    /// is already closed.
    pub async fn send_response(&self, text: impl AsRef<str>) -> Result<(), SpeechEngineError> {
        if self.inner.closed.load(Ordering::SeqCst) {
            return Err(SpeechEngineError::Closed);
        }
        if !self.guard_transcript_handler() {
            return Ok(());
        }
        let event_id = *self.inner.current_event_id.lock().unwrap();
        self.send_agent_response(text.as_ref(), false, event_id);
        self.send_agent_response("", true, event_id);
        Ok(())
    }

    /// Send a streamed LLM response. Each item is a string chunk or an OpenAI/Anthropic/Gemini
    /// stream event (as `serde_json::Value`) whose text is auto-extracted. A trailing final frame
    /// is emitted once the stream ends. Only valid inside an `on_transcript` handler.
    pub async fn send_response_stream<S, I>(&self, stream: S) -> Result<(), SpeechEngineError>
    where
        S: Stream<Item = I>,
        I: Into<StreamChunk>,
    {
        if self.inner.closed.load(Ordering::SeqCst) {
            return Err(SpeechEngineError::Closed);
        }
        if !self.guard_transcript_handler() {
            return Ok(());
        }
        let event_id = *self.inner.current_event_id.lock().unwrap();
        futures::pin_mut!(stream);
        while let Some(item) = stream.next().await {
            if self.inner.closed.load(Ordering::SeqCst) {
                return Ok(());
            }
            if let Some(text) = extract_text(&item.into().0) {
                if !text.is_empty() {
                    self.send_agent_response(&text, false, event_id);
                }
            }
        }
        if !self.inner.closed.load(Ordering::SeqCst) {
            self.send_agent_response("", true, event_id);
        }
        Ok(())
    }

    /// Run one queued transcript turn on the worker task. Runs serially, so `current_event_id`
    /// is stable for the duration of each handler (and thus correct for its `send_response`
    /// calls) even after a later transcript has been queued.
    async fn run_transcript(&self, transcript: Vec<ConversationMessage>, event_id: Option<i64>) {
        *self.inner.current_event_id.lock().unwrap() = event_id;
        self.inner
            .in_transcript_handler
            .store(true, Ordering::SeqCst);
        if let Some(cb) = &self.inner.callbacks.on_transcript {
            cb(transcript, self.clone()).await;
        }
        self.inner
            .in_transcript_handler
            .store(false, Ordering::SeqCst);
    }

    fn guard_transcript_handler(&self) -> bool {
        if !self.inner.in_transcript_handler.load(Ordering::SeqCst) {
            eprintln!(
                "[SpeechEngine] send_response called outside an on_transcript handler; responses \
                 can only be sent in reply to a user transcript. To have the agent speak first, \
                 set a first message in your Speech Engine conversation config."
            );
            return false;
        }
        true
    }

    /// Notify the session that the underlying connection dropped at the transport level. Fires
    /// `disconnected` once. Called by the bundled server; not needed when driving the session
    /// yourself through [`handle_raw_message`](Self::handle_raw_message).
    pub async fn handle_disconnect(&self) {
        // Keyed on disconnect_fired, not closed: a prior close() or inbound `close` message leaves
        // the session closed, but the transport-ended callback must still fire once here so cleanup
        // registered in on_disconnect runs.
        if self.inner.disconnect_fired.swap(true, Ordering::SeqCst) {
            return;
        }
        self.inner.closed.store(true, Ordering::SeqCst);
        if let Some(cb) = &self.inner.callbacks.on_disconnect {
            cb(self.clone()).await;
        }
    }

    /// Close the session and its underlying connection. Idempotent.
    pub async fn close(&self) {
        if self.inner.closed.swap(true, Ordering::SeqCst) {
            return;
        }
        let _ = self.inner.outbound.send(OutboundFrame::Close);
    }

    async fn emit_error(&self, error: SpeechEngineError) {
        if let Some(cb) = &self.inner.callbacks.on_error {
            cb(error, self.clone()).await;
        } else if self.inner.debug {
            eprintln!("[SpeechEngine] unhandled error: {error}");
        }
    }

    fn send_agent_response(&self, content: &str, is_final: bool, event_id: Option<i64>) {
        self.send_json(json!({
            "type": "agent_response",
            "content": content,
            "event_id": event_id,
            "is_final": is_final,
        }));
    }

    fn send_json(&self, value: Value) {
        if self.inner.closed.load(Ordering::SeqCst) {
            return;
        }
        if let Ok(text) = serde_json::to_string(&value) {
            // Send failure means the transport is gone; the read loop detects it.
            let _ = self.inner.outbound.send(OutboundFrame::Text(text));
        }
    }
}

fn parse_transcript(msg: &Value) -> Vec<ConversationMessage> {
    let mut out = Vec::new();
    if let Some(arr) = msg.get("user_transcript").and_then(Value::as_array) {
        for m in arr {
            if let (Some(role), Some(content)) = (
                m.get("role").and_then(Value::as_str),
                m.get("content").and_then(Value::as_str),
            ) {
                out.push(ConversationMessage {
                    role: role.to_string(),
                    content: content.to_string(),
                });
            }
        }
    }
    out
}

/// Extract text from one LLM stream chunk. Handles plain strings and the common streaming shapes:
/// OpenAI Responses (`response.output_text.delta`), OpenAI Chat Completions
/// (`choices[0].delta.content`), Anthropic (`content_block_delta` / `text_delta`), and Google
/// Gemini (text parts of `candidates[0].content.parts`).
fn extract_text(chunk: &Value) -> Option<String> {
    if let Some(s) = chunk.as_str() {
        return Some(s.to_string());
    }
    if !chunk.is_object() {
        return None;
    }

    // OpenAI Responses API
    if chunk.get("type").and_then(Value::as_str) == Some("response.output_text.delta") {
        if let Some(delta) = chunk.get("delta").and_then(Value::as_str) {
            return Some(delta.to_string());
        }
    }

    // OpenAI Chat Completions API
    if let Some(content) = chunk
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|c| c.first())
        .and_then(|c| c.get("delta"))
        .and_then(|d| d.get("content"))
        .and_then(Value::as_str)
    {
        return Some(content.to_string());
    }

    // Anthropic Messages API
    if chunk.get("type").and_then(Value::as_str) == Some("content_block_delta") {
        if let Some(delta) = chunk.get("delta") {
            if delta.get("type").and_then(Value::as_str) == Some("text_delta") {
                if let Some(text) = delta.get("text").and_then(Value::as_str) {
                    return Some(text.to_string());
                }
            }
        }
    }

    // Google Gemini API
    if let Some(parts) = chunk
        .get("candidates")
        .and_then(Value::as_array)
        .and_then(|c| c.first())
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(Value::as_array)
    {
        let mut texts = String::new();
        for part in parts {
            if part.get("thought").and_then(Value::as_bool) == Some(true) {
                continue;
            }
            if let Some(text) = part.get("text").and_then(Value::as_str) {
                texts.push_str(text);
            }
        }
        if !texts.is_empty() {
            return Some(texts);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain_text(rx: &mut mpsc::UnboundedReceiver<OutboundFrame>) -> Vec<Value> {
        let mut out = Vec::new();
        while let Ok(frame) = rx.try_recv() {
            if let OutboundFrame::Text(t) = frame {
                out.push(serde_json::from_str(&t).unwrap());
            }
        }
        out
    }

    // Transcript responses are produced on the worker task, so await them rather than draining
    // synchronously. Collects up to `n` text frames (ignoring Close), with a timeout guard.
    async fn recv_text(rx: &mut mpsc::UnboundedReceiver<OutboundFrame>, n: usize) -> Vec<Value> {
        let mut out = Vec::new();
        while out.len() < n {
            match tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv()).await {
                Ok(Some(OutboundFrame::Text(t))) => out.push(serde_json::from_str(&t).unwrap()),
                Ok(Some(OutboundFrame::Close)) => {}
                _ => break,
            }
        }
        out
    }

    #[tokio::test]
    async fn ping_is_answered_with_pong() {
        let (session, mut rx) =
            SpeechEngineSession::new(Arc::new(SpeechEngineCallbacks::default()), false);
        session.handle_raw_message(r#"{"type":"ping"}"#).await;
        let frames = drain_text(&mut rx);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0]["type"], "pong");
    }

    #[tokio::test]
    async fn init_sets_conversation_id_and_fires_callback() {
        let seen = Arc::new(Mutex::new(None));
        let seen2 = Arc::clone(&seen);
        let callbacks = SpeechEngineCallbacks::default().on_init(move |id, _s| {
            let seen = Arc::clone(&seen2);
            async move {
                *seen.lock().unwrap() = id;
            }
        });
        let (session, _rx) = SpeechEngineSession::new(Arc::new(callbacks), false);
        session
            .handle_raw_message(r#"{"type":"init","conversation_id":"conv_42"}"#)
            .await;
        assert_eq!(session.conversation_id().as_deref(), Some("conv_42"));
        assert_eq!(seen.lock().unwrap().as_deref(), Some("conv_42"));
    }

    #[tokio::test]
    async fn transcript_handler_can_send_a_string_response() {
        let callbacks =
            SpeechEngineCallbacks::default().on_transcript(|transcript, session| async move {
                assert_eq!(transcript.len(), 1);
                assert_eq!(transcript[0].role, "user");
                assert_eq!(transcript[0].content, "hello");
                session.send_response("hi there").await.unwrap();
            });
        let (session, mut rx) = SpeechEngineSession::new(Arc::new(callbacks), false);
        session
            .handle_raw_message(
                r#"{"type":"user_transcript","event_id":7,"user_transcript":[{"role":"user","content":"hello"}]}"#,
            )
            .await;
        let frames = recv_text(&mut rx, 2).await;
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0]["type"], "agent_response");
        assert_eq!(frames[0]["content"], "hi there");
        assert_eq!(frames[0]["event_id"], 7);
        assert_eq!(frames[0]["is_final"], false);
        assert_eq!(frames[1]["content"], "");
        assert_eq!(frames[1]["is_final"], true);
        assert_eq!(frames[1]["event_id"], 7);
    }

    #[tokio::test]
    async fn transcript_handler_can_stream_mixed_chunks() {
        let callbacks = SpeechEngineCallbacks::default().on_transcript(|_t, session| async move {
            let chunks: Vec<StreamChunk> = vec![
                "plain ".into(),
                serde_json::json!({ "type": "response.output_text.delta", "delta": "openai " })
                    .into(),
                serde_json::json!({
                    "type": "content_block_delta",
                    "delta": { "type": "text_delta", "text": "anthropic " }
                })
                .into(),
                serde_json::json!({
                    "candidates": [{ "content": { "parts": [{ "text": "gemini" }] } }]
                })
                .into(),
            ];
            session
                .send_response_stream(futures::stream::iter(chunks))
                .await
                .unwrap();
        });
        let (session, mut rx) = SpeechEngineSession::new(Arc::new(callbacks), false);
        session
            .handle_raw_message(r#"{"type":"user_transcript","event_id":3,"user_transcript":[]}"#)
            .await;
        // Four content chunks plus the trailing final frame.
        let frames = recv_text(&mut rx, 5).await;
        let contents: Vec<&str> = frames
            .iter()
            .filter(|f| f["is_final"] == false)
            .map(|f| f["content"].as_str().unwrap())
            .collect();
        assert_eq!(contents, vec!["plain ", "openai ", "anthropic ", "gemini"]);
        assert_eq!(frames.last().unwrap()["is_final"], true);
    }

    #[tokio::test]
    async fn send_response_outside_transcript_handler_is_a_noop() {
        let (session, mut rx) =
            SpeechEngineSession::new(Arc::new(SpeechEngineCallbacks::default()), false);
        // Mark open but never entered a transcript handler.
        session.send_response("nope").await.unwrap();
        assert!(drain_text(&mut rx).is_empty());
    }

    #[tokio::test]
    async fn close_message_fires_close_and_marks_closed() {
        let fired = Arc::new(AtomicBool::new(false));
        let fired2 = Arc::clone(&fired);
        let callbacks = SpeechEngineCallbacks::default().on_close(move |_s| {
            let fired = Arc::clone(&fired2);
            async move {
                fired.store(true, Ordering::SeqCst);
            }
        });
        let (session, _rx) = SpeechEngineSession::new(Arc::new(callbacks), false);
        session.handle_raw_message(r#"{"type":"close"}"#).await;
        assert!(fired.load(Ordering::SeqCst));
        assert!(!session.is_open());
    }

    #[tokio::test]
    async fn disconnect_fires_once() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let count2 = Arc::clone(&count);
        let callbacks = SpeechEngineCallbacks::default().on_disconnect(move |_s| {
            let count = Arc::clone(&count2);
            async move {
                count.fetch_add(1, Ordering::SeqCst);
            }
        });
        let (session, _rx) = SpeechEngineSession::new(Arc::new(callbacks), false);
        session.handle_disconnect().await;
        session.handle_disconnect().await;
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn ping_is_answered_while_a_transcript_handler_is_running() {
        // The handler blocks until released; the read loop must still answer a ping meanwhile.
        let (release_tx, release_rx) = tokio::sync::oneshot::channel::<()>();
        let release_rx = Arc::new(tokio::sync::Mutex::new(Some(release_rx)));
        let started = Arc::new(tokio::sync::Notify::new());
        let started2 = Arc::clone(&started);
        let callbacks = SpeechEngineCallbacks::default().on_transcript(move |_t, _s| {
            let release_rx = Arc::clone(&release_rx);
            let started = Arc::clone(&started2);
            async move {
                started.notify_one();
                if let Some(rx) = release_rx.lock().await.take() {
                    let _ = rx.await;
                }
            }
        });
        let (session, mut rx) = SpeechEngineSession::new(Arc::new(callbacks), false);

        session
            .handle_raw_message(r#"{"type":"user_transcript","event_id":1,"user_transcript":[]}"#)
            .await;
        // Wait until the (blocked) handler is actually running on the worker.
        tokio::time::timeout(std::time::Duration::from_secs(5), started.notified())
            .await
            .unwrap();

        // handle_raw_message returns immediately and the ping is answered despite the blocked worker.
        session.handle_raw_message(r#"{"type":"ping"}"#).await;
        let frames = recv_text(&mut rx, 1).await;
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0]["type"], "pong");

        let _ = release_tx.send(());
    }

    #[tokio::test]
    async fn disconnect_fires_after_a_close_message() {
        // A `close` message marks the session closed; the transport ending afterwards must still
        // fire on_disconnect so cleanup registered there runs.
        let disconnected = Arc::new(AtomicBool::new(false));
        let disconnected2 = Arc::clone(&disconnected);
        let callbacks = SpeechEngineCallbacks::default().on_disconnect(move |_s| {
            let disconnected = Arc::clone(&disconnected2);
            async move {
                disconnected.store(true, Ordering::SeqCst);
            }
        });
        let (session, _rx) = SpeechEngineSession::new(Arc::new(callbacks), false);

        session.handle_raw_message(r#"{"type":"close"}"#).await;
        assert!(!session.is_open());
        session.handle_disconnect().await;

        assert!(
            disconnected.load(Ordering::SeqCst),
            "on_disconnect must fire when the transport ends even after a close message"
        );
    }

    #[tokio::test]
    async fn malformed_frame_surfaces_on_error() {
        let fired = Arc::new(AtomicBool::new(false));
        let fired2 = Arc::clone(&fired);
        let callbacks = SpeechEngineCallbacks::default().on_error(move |_e, _s| {
            let fired = Arc::clone(&fired2);
            async move {
                fired.store(true, Ordering::SeqCst);
            }
        });
        let (session, _rx) = SpeechEngineSession::new(Arc::new(callbacks), false);
        session.handle_raw_message("not json").await;
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn extract_text_handles_each_provider() {
        assert_eq!(extract_text(&Value::String("x".into())), Some("x".into()));
        assert_eq!(
            extract_text(&serde_json::json!({"choices":[{"delta":{"content":"c"}}]})),
            Some("c".into())
        );
        // A Gemini "thought" part is skipped.
        assert_eq!(
            extract_text(&serde_json::json!({
                "candidates": [{ "content": { "parts": [
                    { "thought": true, "text": "reasoning" },
                    { "text": "answer" }
                ] } }]
            })),
            Some("answer".into())
        );
        assert_eq!(extract_text(&serde_json::json!({"unknown":"shape"})), None);
    }
}
