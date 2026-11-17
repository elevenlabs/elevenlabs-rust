use thiserror::Error;

/// Errors raised by the hand-written Speech Engine server surface.
///
/// Hand-written (see `.fernignore`): the generator has no concept of the inbound
/// Speech Engine handshake, so these live outside [`crate::error::ApiError`].
#[derive(Debug, Error)]
pub enum SpeechEngineError {
    /// The inbound authorization JWT was missing, malformed, or failed verification.
    /// The message mirrors the Python/TS/PHP SDKs so every SDK reports the same reason.
    #[error("{0}")]
    Jwt(String),

    /// A protocol-level problem: an inbound frame was not JSON, not an object, or the
    /// API reported an `error` message.
    #[error("{0}")]
    Protocol(String),

    /// An operation was attempted on a session whose connection has already closed.
    #[error("session is closed")]
    Closed,

    /// The underlying WebSocket transport failed (bind, accept, or handshake).
    #[error("websocket error: {0}")]
    WebSocket(String),
}
