//! Prelude module for convenient imports
//!
//! This module re-exports the most commonly used types and traits.
//! Import it with: `use elevenlabs_rust_sdk::prelude::*;`

// Client and configuration
pub use crate::client::ApiClient;
pub use crate::config::{ClientConfig, ClientConfigBuilder};
pub use crate::core::ByteStream;
pub use crate::core::{HttpClient, RequestOptions};
pub use crate::environment::Environment;
pub use crate::error::ApiError;
pub use crate::error::BuildError;

// Resource clients and model types
pub use crate::api::*;

// Commonly used external types
pub use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use std::collections::HashMap;
pub use std::fmt;
