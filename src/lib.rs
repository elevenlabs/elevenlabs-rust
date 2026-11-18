//! # ApiClient
//!
//! This is the documentation for the ElevenLabs API. You can use this API to use our service programmatically, this is done by using your API key. You can find your API key in the dashboard at https://elevenlabs.io/app/settings/api-keys.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ClientConfig::builder()
//!     .build()?;
//! let client = ApiClient::new(config)?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

// Hand-written agent-workflows-as-code builder (see `.fernignore`). A crate-root module must be
// declared here; this line is part of the hand-written wiring Fern does not emit.
pub mod agent_workflows;

// Hand-written Speech Engine inbound-server surface (see `.fernignore`). Declared here because a
// crate-root module must be; this one line is part of the hand-written wiring Fern does not emit.
#[cfg(feature = "websocket")]
pub mod speech_engine;

// Hand-written webhook signature verifier (see `.fernignore`). Declared here because a crate-root
// module must be; this one line is part of the hand-written wiring Fern does not emit.
pub mod webhooks;

// Hand-written flows `create_and_wait` helpers (see `.fernignore`). Declared here because a
// crate-root module must be; this one line is part of the hand-written wiring Fern does not emit.
pub mod flows;

pub use api::*;
pub use client::ApiClient;
pub use client::ApiClientBuilder;
pub use config::ClientConfig;
pub use config::ClientConfigBuilder;
pub use core::config_manager::ConfigManager;
#[cfg(feature = "websocket")]
pub use core::{
    DisconnectInfo, WebSocketClient, WebSocketMessage, WebSocketOptions, WebSocketState,
};
pub use core::{HttpClient, RequestOptions};
pub use core::{SseEvent, SseMetadata, SseStream, SseStreamWithMetadata};
pub use environment::Environment;
pub use error::ApiError;
pub use reqwest;
pub use serde_json;
