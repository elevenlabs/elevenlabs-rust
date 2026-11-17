//! A handle to a Speech Engine instance, with the server-side helpers for accepting inbound
//! connections.
//!
//! Hand-written (see `.fernignore`), mirroring `SpeechEngineResource` in the Python/TS/PHP SDKs.
//! Once the generated root client is wired to return it, `speech_engine.create()` /
//! `speech_engine.get()` hand back one of these; today it is constructed directly.

use std::sync::Arc;

use tokio::sync::mpsc;

use super::error::SpeechEngineError;
use super::jwt::verify_speech_engine_jwt;
use super::server::{SpeechEngineServer, SpeechEngineServerOptions};
use super::session::{OutboundFrame, SpeechEngineCallbacks, SpeechEngineSession};
use crate::api::types::SpeechEngineResponse;

const AUTH_HEADER: &str = "X-ElevenLabs-Speech-Engine-Authorization";

/// A handle to a Speech Engine instance.
///
/// Use [`serve`](Self::serve) to run a standalone WebSocket server for this engine, or
/// [`create_session`](Self::create_session) to drive a session over a connection you manage
/// yourself. [`verify_request`](Self::verify_request) checks an inbound request's authorization
/// header when you handle the WebSocket upgrade.
pub struct SpeechEngineResource {
    /// The speech engine ID.
    pub engine_id: String,
    /// The full API response, or `None` when constructed without an API call.
    pub config: Option<SpeechEngineResponse>,
    api_key: Option<String>,
}

impl SpeechEngineResource {
    /// Construct a resource for `engine_id`, verifying inbound connections with `api_key`.
    pub fn new(
        engine_id: impl Into<String>,
        api_key: Option<String>,
        config: Option<SpeechEngineResponse>,
    ) -> Self {
        Self {
            engine_id: engine_id.into(),
            config,
            api_key: api_key.filter(|k| !k.is_empty()),
        }
    }

    /// Verify that an incoming request carries a valid Speech Engine authorization JWT. Header
    /// names are matched case-insensitively. Only needed when managing the WebSocket upgrade
    /// yourself; [`serve`](Self::serve) verifies automatically.
    pub fn verify_request<'a, I>(&self, headers: I) -> bool
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let api_key = match self.api_key.as_deref() {
            Some(k) => k,
            None => return false,
        };
        let token = headers
            .into_iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(AUTH_HEADER))
            .map(|(_, value)| value);
        match token {
            Some(t) if !t.is_empty() => verify_speech_engine_jwt(t, api_key).is_ok(),
            _ => false,
        }
    }

    /// Create a session over a connection you manage yourself (any transport). Returns the session
    /// and the receiver your transport drains for outbound frames.
    pub fn create_session(
        &self,
        callbacks: SpeechEngineCallbacks,
        debug: bool,
    ) -> (SpeechEngineSession, mpsc::UnboundedReceiver<OutboundFrame>) {
        SpeechEngineSession::new(Arc::new(callbacks), debug)
    }

    /// Start a standalone WebSocket server for this engine. Runs until the server is stopped.
    /// Verification uses the engine's API key unless `disable_auth` is set.
    pub async fn serve(
        &self,
        port: u16,
        path: Option<String>,
        debug: bool,
        disable_auth: bool,
        callbacks: SpeechEngineCallbacks,
    ) -> Result<(), SpeechEngineError> {
        let server = SpeechEngineServer::new(
            SpeechEngineServerOptions {
                port,
                path,
                api_key: self.api_key.clone(),
                debug,
                disable_auth,
                ..Default::default()
            },
            Arc::new(callbacks),
        );
        server.serve().await
    }

    /// Build an [`axum::Router`] that accepts inbound Speech Engine connections for this engine at
    /// `path`. Merge or nest it into an existing router.
    #[cfg(feature = "axum")]
    pub fn axum_router(
        &self,
        path: &str,
        debug: bool,
        disable_auth: bool,
        callbacks: SpeechEngineCallbacks,
    ) -> axum::Router {
        super::server::speech_engine_router(
            path,
            SpeechEngineServerOptions {
                port: 0,
                path: None,
                api_key: self.api_key.clone(),
                debug,
                disable_auth,
                ..Default::default()
            },
            Arc::new(callbacks),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN_API_KEY: &str = "sk_0123456789abcdef0123456789abcdef";
    const GOLDEN_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL2FwaS5lbGV2ZW5sYWJzLmlvL2NvbnZhaS9zcGVlY2gtZW5naW5lIiwic3ViIjoiY29udmFpX3NwZWVjaF9lbmdpbmVfdXBzdHJlYW0iLCJpYXQiOjEyMzQ1Njc4OTAsImV4cCI6NDEwMjQ0NDgwMH0.UObM4hx1W8NvGViU3aqPv1-IEXhmXgWDgoS9RZZWILk";

    #[test]
    fn verify_request_accepts_the_golden_token_case_insensitively() {
        let resource =
            SpeechEngineResource::new("seng_1", Some(GOLDEN_API_KEY.to_string()), None);
        assert!(resource.verify_request([(
            "x-elevenlabs-speech-engine-authorization",
            GOLDEN_TOKEN,
        )]));
    }

    #[test]
    fn verify_request_rejects_a_missing_header() {
        let resource =
            SpeechEngineResource::new("seng_1", Some(GOLDEN_API_KEY.to_string()), None);
        assert!(!resource.verify_request([("content-type", "application/json")]));
    }

    #[test]
    fn verify_request_rejects_without_an_api_key() {
        let resource = SpeechEngineResource::new("seng_1", None, None);
        assert!(!resource.verify_request([(AUTH_HEADER, GOLDEN_TOKEN)]));
    }
}
