//! Agent workflows as code: a hand-written builder on top of the generated API client.
//!
//! The whole module is hand-written (every file is in `.fernignore`). It builds the SDK's generated
//! workflow request type ([`crate::AgentWorkflowRequestModel`]) for `agents.create()` /
//! `agents.update()`, and canonical API wire-format JSON for storing in git. It mirrors the
//! Python/TypeScript SDKs and passes the shared cross-SDK conformance fixtures byte-for-byte.
//!
//! ```no_run
//! use elevenlabs_rust_sdk::agent_workflows::{Chain, Destination, Workflow};
//!
//! let wf = Workflow::new(None);
//! let triage = wf.converse("triage", "Find out what the caller needs", Default::default());
//! triage
//!     .when("Caller wants a human".into(), None)
//!     .transfer("human", Destination::phone("+15551234567"), Default::default());
//! let json = wf.to_json_string().unwrap();
//! ```

mod condition;
mod destination;
mod errors;
mod expr;
mod graph;
mod layout;
mod node;
mod node_types;
mod validate;
// Shared with the agent_tools builder, which writes the same canonical JSON.
pub(crate) mod wire;
mod workflow;

pub use condition::{Condition, ConditionInput};
pub use destination::Destination;
pub use errors::{Severity, WorkflowIssue, WorkflowIssueCode, WorkflowValidationError};
pub use expr::{Expr, ExprInput, LlmValueType};
pub use node::{Chain, Options, WorkflowBranch, WorkflowNode};
pub use workflow::Workflow;
