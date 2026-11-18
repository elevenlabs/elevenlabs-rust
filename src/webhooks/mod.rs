//! Verify the signature on webhooks that ElevenLabs sends to your server.
//!
//! ElevenLabs signs each webhook with an HMAC-SHA256 digest, sent in the `ElevenLabs-Signature`
//! header as a compound value of the form `t=<unix_timestamp>,v0=<hex_digest>`. The signed message
//! is the timestamp and the raw request body joined with a period (`<timestamp>.<raw_body>`).
//!
//! This whole module is hand-written (every file is listed in `.fernignore`). The generator's
//! built-in webhook verifier cannot parse a compound header that carries both the timestamp and
//! the signature, so it is maintained by hand to match the Python, TypeScript, PHP and Ruby SDKs.
//!
//! ```rust
//! use elevenlabs_rust_sdk::webhooks::{construct_event, WebhookError};
//!
//! fn handle(raw_body: &[u8], sig_header: &str, secret: &str) -> Result<(), WebhookError> {
//!     let event = construct_event(raw_body, sig_header, secret)?;
//!     if event["type"] == "post_call_transcription" {
//!         println!("conversation for agent {}", event["data"]["agent_id"]);
//!     }
//!     Ok(())
//! }
//! ```

mod error;
mod verifier;

pub use error::WebhookError;
pub use verifier::{construct_event, TOLERANCE_SECONDS};
