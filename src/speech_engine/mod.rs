//! Server-side Speech Engine support: accept the inbound WebSocket connections the ElevenLabs
//! Speech Engine API opens to a developer-owned server, verify them, and drive one conversation
//! per connection.
//!
//! This whole module is hand-written (every file is listed in `.fernignore`). The generated client
//! only provides the *upstream* channel that dials out to `/speech-engine/upstream`
//! (see [`crate::api::websocket::speech_engine`]); the generator has no concept of the inbound
//! server handshake, so it is maintained by hand to match the Python, TypeScript and PHP SDKs.
//!
//! * [`verify_speech_engine_jwt`] — verify the `X-ElevenLabs-Speech-Engine-Authorization` JWT.
//! * [`SpeechEngineSession`] — transport-agnostic protocol handling for one conversation.
//! * [`SpeechEngineServer`] — a standalone `tokio-tungstenite` server (and, behind the `axum`
//!   feature, [`speech_engine_router`] for an existing axum app).
//! * [`SpeechEngineResource`] — a per-engine handle with `verify_request` / `create_session` /
//!   `serve`.

mod error;
mod jwt;
mod resource;
mod server;
mod session;

pub use error::SpeechEngineError;
pub use jwt::verify_speech_engine_jwt;
pub use resource::SpeechEngineResource;
pub use server::{SpeechEngineServer, SpeechEngineServerOptions};
pub use session::{
    ConversationMessage, OutboundFrame, SpeechEngineCallbacks, SpeechEngineSession, StreamChunk,
};

#[cfg(feature = "axum")]
pub use server::speech_engine_router;
