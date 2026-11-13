use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

type ByteStream = Pin<Box<dyn Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send>>;

/// One decoded `text/event-stream` frame.
///
/// Fields follow the SSE specification: `data` lines are joined with a newline, and `event`,
/// `id` and `retry` carry the last value seen in the frame.
#[derive(Debug, Default, Clone)]
struct SseFrame {
    data: Vec<String>,
    event: Option<String>,
    id: Option<String>,
    retry: Option<u64>,
}

impl SseFrame {
    fn is_empty(&self) -> bool {
        self.data.is_empty() && self.event.is_none() && self.id.is_none() && self.retry.is_none()
    }

    /// The frame payload: SSE joins multiple `data:` lines of one frame with a newline.
    fn payload(&self) -> String {
        self.data.join("\n")
    }
}

/// Splits one frame line into its field name and value, tolerating both `field: value` and
/// `field:value`. A line with no colon is a bare field name with an empty value; a line
/// starting with a colon is a comment (keepalive) and is dropped by the caller.
fn parse_field(line: &str) -> Option<(&str, &str)> {
    if line.starts_with(':') {
        return None;
    }
    match line.find(':') {
        Some(idx) => {
            let value = &line[idx + 1..];
            Some((&line[..idx], value.strip_prefix(' ').unwrap_or(value)))
        }
        None => Some((line, "")),
    }
}

fn decode_frame(block: &str) -> SseFrame {
    let mut frame = SseFrame::default();
    for line in block.split('\n') {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let Some((field, value)) = parse_field(line) else {
            continue;
        };
        match field {
            "data" => frame.data.push(value.to_string()),
            "event" => frame.event = Some(value.to_string()),
            "id" => frame.id = Some(value.to_string()),
            "retry" => frame.retry = value.parse().ok(),
            _ => {}
        }
    }
    frame
}

/// Position just past the first frame boundary in `buffer`, with the boundary length.
///
/// A frame ends at a blank line, which is `\n\n` or `\r\n\r\n` on the wire. Both are matched so
/// a CRLF stream frames correctly rather than leaving stray `\r` on every field. A bare-CR
/// terminator (`\r\r`) is not recognised, matching the other generators — Ruby splits on
/// `/\r?\n\r?\n/` and the previous parser split on `\n` — so no SDK disagrees about framing.
///
/// A stream that is not SSE is newline-delimited JSON, so there every `\n` ends a frame; the
/// trailing `\r` of a CRLF line and the empty frame of a blank line are dropped when decoded.
///
/// `from` is where to resume. A frame can take many chunks to arrive, and rescanning the whole
/// accumulated buffer on every poll is quadratic in the frame size; the caller tracks how far
/// it has already looked. Resuming two bytes early is what keeps a boundary that straddles the
/// previous scan edge from being missed, since the longest boundary matched here is three bytes.
fn frame_boundary(buffer: &[u8], from: usize, is_sse: bool) -> Option<(usize, usize)> {
    let mut i = from.saturating_sub(2);
    if !is_sse {
        return buffer[i..]
            .iter()
            .position(|byte| *byte == b'\n')
            .map(|at| (i + at, 1));
    }
    while i < buffer.len() {
        if buffer[i] == b'\n' {
            if i + 1 < buffer.len() && buffer[i + 1] == b'\n' {
                return Some((i, 2));
            }
            if i + 2 < buffer.len() && buffer[i + 1] == b'\r' && buffer[i + 2] == b'\n' {
                return Some((i, 3));
            }
        }
        i += 1;
    }
    None
}

/// Injects the SSE `event:` value into the JSON payload under `field`.
///
/// Used where the union discriminant lives in the event field rather than inside the data, so
/// the payload is not self-describing until the two are combined. An existing key is left
/// alone: the payload's own value wins over the protocol-level one.
fn inject_discriminator(payload: &str, field: &str, value: &str) -> String {
    let Ok(mut parsed) = serde_json::from_str::<serde_json::Value>(payload) else {
        return payload.to_string();
    };
    let Some(object) = parsed.as_object_mut() else {
        return payload.to_string();
    };
    if object.contains_key(field) {
        return payload.to_string();
    }
    object.insert(
        field.to_string(),
        serde_json::Value::String(value.to_string()),
    );
    parsed.to_string()
}

/// Metadata from a Server-Sent Event
///
/// Contains the SSE protocol fields (event, id, retry) that accompany the data payload.
///
/// Per the SSE specification:
/// - `event` defaults to "message" when not specified by the server
/// - `id` is the last event id the server sent, used for reconnection (Last-Event-ID header)
/// - `retry` is the last reconnection time the server sent, in milliseconds
#[derive(Debug, Clone)]
pub struct SseMetadata {
    /// The event type (defaults to "message" per SSE spec if not specified by server)
    pub event: String,
    /// The event ID for reconnection support (None if not specified)
    pub id: Option<String>,
    /// Retry timeout in milliseconds (None if not specified)
    pub retry: Option<u64>,
}

/// A Server-Sent Event with both data and metadata
#[derive(Debug)]
pub struct SseEvent<T> {
    /// The deserialized data payload
    pub data: T,
    /// SSE protocol metadata
    pub metadata: SseMetadata,
}

type EventStream<T> =
    Pin<Box<dyn Stream<Item = Result<SseEvent<T>, crate::error::ApiError>> + Send>>;

/// A typed stream of server-sent events.
///
/// Yields `Result<T, ApiError>` per event. Call [`SseStream::with_metadata`] to receive each
/// event's `event`, `id` and `retry` alongside its data.
pub struct SseStream<T> {
    inner: EventStream<T>,
}

impl<T> SseStream<T> {
    /// Convert this stream into one that yields events with metadata
    pub fn with_metadata(self) -> SseStreamWithMetadata<T> {
        SseStreamWithMetadata { inner: self.inner }
    }
}

impl<T> Stream for SseStream<T> {
    type Item = Result<T, crate::error::ApiError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.inner
            .as_mut()
            .poll_next(cx)
            .map(|item| item.map(|result| result.map(|event| event.data)))
    }
}

/// Stream wrapper that yields events with metadata
///
/// Created by calling [`SseStream::with_metadata()`].
pub struct SseStreamWithMetadata<T> {
    inner: EventStream<T>,
}

impl<T> Stream for SseStreamWithMetadata<T> {
    type Item = Result<SseEvent<T>, crate::error::ApiError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

struct SseState {
    stream: ByteStream,
    buffer: Vec<u8>,
    scanned: usize,
    finished: bool,
    seen_id: Option<String>,
    retry_ms: Option<u64>,
}

impl SseState {
    fn new(response: reqwest::Response) -> Self {
        Self {
            stream: Box::pin(response.bytes_stream()),
            buffer: Vec::new(),
            scanned: 0,
            finished: false,
            seen_id: None,
            retry_ms: None,
        }
    }

    fn observe(&mut self, block: &str) -> (Option<String>, Option<String>) {
        let frame = decode_frame(block);
        if frame.retry.is_some() {
            self.retry_ms = frame.retry;
        }
        if frame.id.is_some() {
            self.seen_id = frame.id.clone();
        }
        let id = frame.id.filter(|id| !id.is_empty() && !id.contains('\0'));
        (id, frame.event)
    }

    fn metadata(&self, event: Option<String>) -> SseMetadata {
        SseMetadata {
            event: event
                .filter(|event| !event.is_empty())
                .unwrap_or_else(|| "message".to_string()),
            id: self.seen_id.clone(),
            retry: self.retry_ms,
        }
    }
}

/// Decodes a streaming response body.
///
/// `stream_terminator` ends the stream when a frame's payload matches it exactly; `None` means
/// the spec declared none, so no sentinel is checked and the stream runs to EOF.
/// `event_discriminator` names the JSON property the `event:` field is injected into; `None`
/// leaves the payload untouched. Both come from the spec rather than being assumed.
///
/// A payload that fails to deserialize is surfaced as an `Err` item and the stream stays open:
/// a provider that interleaves a frame shape the spec does not declare still cannot terminate
/// a caller's stream, but neither does the event disappear. Every other generator reports such
/// a frame too -- TypeScript and C# throw, Go returns the error, and Python, Java, PHP and Ruby
/// hand back the raw text -- so dropping it was this generator alone.
///
/// `is_sse` comes from the spec, through the generator, rather than from the response
/// content-type header. The spec is what the generated code was shaped around -- the item type
/// is already fixed at generation time -- so re-deriving the framing from whatever the server
/// labelled the response could disagree with it, and an `x-fern-streaming.format` the caller
/// declared would be silently overruled by a mislabelled response. TypeScript and Go take it
/// from the spec for the same reason.
pub fn parse_sse_stream<T: serde::de::DeserializeOwned + Send + 'static>(
    response: reqwest::Response,
    is_sse: bool,
    raw_text: bool,
    stream_terminator: Option<&'static str>,
    event_discriminator: Option<&'static str>,
) -> Pin<Box<dyn Stream<Item = Result<T, crate::error::ApiError>> + Send>> {
    Box::pin(open_sse_stream(
        response,
        is_sse,
        raw_text,
        stream_terminator,
        event_discriminator,
    ))
}

/// Decodes a streaming response body into an [`SseStream`].
pub(crate) fn open_sse_stream<T: serde::de::DeserializeOwned + Send + 'static>(
    response: reqwest::Response,
    is_sse: bool,
    raw_text: bool,
    stream_terminator: Option<&'static str>,
    event_discriminator: Option<&'static str>,
) -> SseStream<T> {
    SseStream {
        inner: sse_stream(
            SseState::new(response),
            is_sse,
            raw_text,
            stream_terminator,
            event_discriminator,
        ),
    }
}

fn sse_stream<T: serde::de::DeserializeOwned + Send + 'static>(
    state: SseState,
    is_sse: bool,
    raw_text: bool,
    stream_terminator: Option<&'static str>,
    event_discriminator: Option<&'static str>,
) -> EventStream<T> {
    Box::pin(futures::stream::unfold(
        state,
        move |mut state| async move {
            if state.finished {
                return None;
            }
            loop {
                while let Some((end, boundary)) =
                    frame_boundary(&state.buffer, state.scanned, is_sse)
                {
                    let block = String::from_utf8_lossy(&state.buffer[..end]).to_string();
                    state.buffer.drain(..end + boundary);
                    state.scanned = 0;
                    let (_, frame_event) = if is_sse {
                        state.observe(&block)
                    } else {
                        (None, None)
                    };
                    match decode_payload::<T>(
                        &block,
                        is_sse,
                        raw_text,
                        stream_terminator,
                        event_discriminator,
                    ) {
                        FrameOutcome::Item(data) => {
                            let metadata = state.metadata(frame_event);
                            return Some((Ok(SseEvent { data, metadata }), state));
                        }
                        FrameOutcome::Terminate => return None,
                        FrameOutcome::Skip => continue,
                        FrameOutcome::Malformed(message) => {
                            return Some((
                                Err(crate::error::ApiError::SseParseError(message)),
                                state,
                            ));
                        }
                    }
                }
                state.scanned = state.buffer.len();
                match state.stream.next().await {
                    Some(Ok(chunk)) => state.buffer.extend_from_slice(&chunk),
                    Some(Err(error)) => {
                        state.finished = true;
                        return Some((Err(crate::error::ApiError::Network(error)), state));
                    }
                    None if is_sse => {
                        return None;
                    }
                    None => {
                        state.finished = true;
                        let remaining = String::from_utf8_lossy(&state.buffer).to_string();
                        state.buffer.clear();
                        return match decode_payload::<T>(
                            &remaining,
                            is_sse,
                            raw_text,
                            stream_terminator,
                            event_discriminator,
                        ) {
                            FrameOutcome::Item(data) => {
                                let metadata = state.metadata(None);
                                Some((Ok(SseEvent { data, metadata }), state))
                            }
                            FrameOutcome::Malformed(message) => {
                                Some((Err(crate::error::ApiError::SseParseError(message)), state))
                            }
                            FrameOutcome::Terminate | FrameOutcome::Skip => None,
                        };
                    }
                }
            }
        },
    ))
}

/// What one frame block amounts to.
///
/// A named outcome rather than a nested `Option`, whose two levels only read as "skip" and
/// "terminate" if you have the doc comment in front of you.
enum FrameOutcome<T> {
    /// Nothing to emit and nothing lost: a keepalive, or a frame carrying no `data:`.
    Skip,
    /// A payload that would not deserialize into the item type. Kept apart from `Skip`: that
    /// one means the frame held nothing, this one means it held something undeliverable, and
    /// the difference is the whole point -- a caller is told rather than left to infer it from
    /// a stream that simply stops.
    Malformed(String),
    /// The declared terminator was seen; the stream is over.
    Terminate,
    Item(T),
}

/// Decodes one frame block.
fn decode_payload<T: serde::de::DeserializeOwned>(
    block: &str,
    is_sse: bool,
    raw_text: bool,
    stream_terminator: Option<&str>,
    event_discriminator: Option<&str>,
) -> FrameOutcome<T> {
    let (payload, event) = if is_sse {
        let frame = decode_frame(block);
        // A comment-only keepalive and a frame with fields but no `data:` both carry nothing.
        if frame.is_empty() || frame.data.is_empty() {
            return FrameOutcome::Skip;
        }
        (frame.payload(), frame.event)
    } else {
        (block.trim().to_string(), None)
    };

    if payload.is_empty() {
        return FrameOutcome::Skip;
    }
    if stream_terminator == Some(payload.as_str()) {
        return FrameOutcome::Terminate;
    }

    let payload = match (event_discriminator, event) {
        (Some(field), Some(value)) => inject_discriminator(&payload, field, &value),
        _ => payload,
    };

    // An untyped (`type: string`) payload is handed over verbatim: the spec does not say what
    // is inside `data:`, so parsing it as JSON would fail on an object or on bare text, and
    // would quietly unquote a JSON string literal. Routed through `Value::String` rather than
    // returned directly because the item type is only `String` by construction here.
    if raw_text {
        return match serde_json::from_value::<T>(serde_json::Value::String(payload)) {
            Ok(parsed) => FrameOutcome::Item(parsed),
            Err(error) => FrameOutcome::Malformed(error.to_string()),
        };
    }

    match serde_json::from_str::<T>(&payload) {
        Ok(parsed) => FrameOutcome::Item(parsed),
        Err(error) => FrameOutcome::Malformed(error.to_string()),
    }
}
