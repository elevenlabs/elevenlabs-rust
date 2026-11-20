//! Agents as code: an [`Agent`] builder, `push` / `pull` on the client, and [`AgentProject`]
//! (the ElevenLabs CLI's project layout). A hand-written layer on top of the generated client.
//!
//! The whole module is hand-written (it is listed in `.fernignore`). An [`Agent`] composes an
//! agent's settings, its [`Tool`](crate::agent_tools::Tool) objects and its
//! [`Workflow`](crate::agent_workflows::Workflow). `client.agents_as_code().push(&agent, ..)`
//! creates or updates the tools and the agent in one call. [`AgentProject`] stores agents in the
//! CLI's layout, so a repo works with both the SDK and the CLI. It mirrors the TypeScript and
//! Python SDKs and passes the shared project fixture.
//!
//! ```no_run
//! # async fn run(client: &elevenlabs_rust_sdk::ApiClient) -> Result<(), Box<dyn std::error::Error>> {
//! use elevenlabs_rust_sdk::agent_builder::{Agent, AgentOptions, PushOptions};
//! use elevenlabs_rust_sdk::agent_tools::{Schema, SystemTool, Tool};
//! use elevenlabs_rust_sdk::agent_workflows::{Chain, Workflow};
//!
//! let get_order = Tool::webhook("get_order", "Look up an order by its number")
//!     .get("https://api.example.com/orders/{order_id}")
//!     .path_param("order_id", Schema::string().describe("The order number"));
//! let wf = Workflow::new(None);
//! wf.converse("triage", "Find out what the caller needs", Default::default())
//!     .when("Caller gave an order number".into(), None)
//!     .tool("lookup", vec![get_order.into()], Default::default());
//! let agent = Agent::new(
//!     "Support line",
//!     AgentOptions::new()
//!         .prompt("You are a support agent for Acme.")
//!         .first_message("Hi, how can I help?")
//!         .tools(vec![SystemTool::end_call().into()])
//!         .workflow(wf),
//! );
//!
//! let created = client.agents_as_code().push(&agent, PushOptions::default()).await?;
//! client
//!     .agents_as_code()
//!     .push(&agent, PushOptions { agent_id: Some(created.agent_id), ..Default::default() })
//!     .await?;
//! # Ok(())
//! # }
//! ```

mod agent;
mod errors;
mod format;
mod fs;
mod project;
mod push;
mod wire;

#[cfg(test)]
mod tests;

pub use agent::{Agent, AgentOptions};
pub use errors::{
    AgentIssue, AgentIssueCode, AgentProjectError, AgentPushAction, AgentPushError,
    AgentPushResult, AgentValidationError, PartialPushResult, PullError, PushError, Severity,
    ToolPush, ToolPushAction,
};
pub use format::{sanitize_filename, unique_path};
pub use project::{AgentProject, AGENTS_FILE, AGENT_CONFIGS_DIR, TOOLS_FILE, TOOL_CONFIGS_DIR};
pub use push::{AgentsAsCode, PullOptions, PushOptions};
