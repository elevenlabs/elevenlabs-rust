//! Agent tools as code: a hand-written builder on top of the generated API client.
//!
//! The whole module is hand-written (every file is in `.fernignore`). It builds webhook, client and
//! system tools as the SDK's generated tool request type ([`crate::ToolRequestModel`]) for
//! `tools.create()` / `tools.update()`, and canonical API wire-format JSON (the CLI's
//! `tool_configs/*.json` format) for storing in git. It mirrors the TypeScript/Python SDKs and
//! passes the shared cross-SDK conformance fixtures byte-for-byte.
//!
//! ```no_run
//! use elevenlabs_rust_sdk::agent_tools::{Header, Schema, Tool, ToolOptions};
//!
//! let get_order = Tool::webhook("get_order", "Look up an order by the number the caller gives")
//!     .get("https://api.example.com/orders/{order_id}")
//!     .path_param("order_id", Schema::string().describe("The order number, e.g. A-1042"))
//!     .header("Authorization", Header::secret("sec_orders_api"))
//!     .options(ToolOptions::new().set("pre_tool_speech", "force"));
//! let request = get_order.build().unwrap(); // for client.tools.create(&request, None)
//! let json = get_order.to_json_string().unwrap(); // for tool_configs/get_order.json
//! ```

mod client;
mod errors;
mod header;
// Shared with the agent_builder module, which writes the same JSON with a 4-space indent.
pub(crate) mod json;
mod options;
mod schema;
mod system;
mod tool;
mod validate;
mod webhook;

#[cfg(test)]
mod tests;

pub use crate::agent_workflows::Destination;
pub use client::ClientTool;
pub use errors::{Severity, ToolIssue, ToolIssueCode, ToolValidationError};
pub use header::{AuthConnection, Header};
pub use options::{
    AssignOptions, KnowledgeBaseToolOptions, LanguageDetectionOptions, PlayKeypadTouchToneOptions,
    SkipTurnOptions, ToolOptions, TransferOptions, TransferToNumberOptions,
    VoicemailDetectionOptions,
};
pub use schema::{ArraySchema, LiteralSchema, ObjectSchema, Schema, SchemaKind};
pub use system::SystemTool;
pub use tool::{OtherTool, Tool, ToolKind};
pub use webhook::WebhookTool;
