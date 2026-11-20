//! Problems found in an agent, a project, or a push.

use std::collections::BTreeMap;
use std::fmt;

pub use crate::agent_workflows::Severity;

/// A validation issue code, from the shared cross-SDK vocabulary (see the agents-as-code spec).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentIssueCode {
    /// The name is empty.
    MissingName,
    /// No `prompt` after assembling the config (a warning).
    MissingPrompt,
    /// Two different tools share a name. The same tool used twice is fine.
    DuplicateToolName,
    /// A tool's own `build()` fails. `path` is `tools.<name>`.
    InvalidTool,
    /// The workflow's own `build()` fails.
    InvalidWorkflow,
    /// The raw `conversation_config` or `platform_settings` doesn't serialize.
    InvalidConfig,
    /// The raw config sets a field that a typed option, `tools` or a system tool also sets.
    OptionConflict,
    /// `to_json()` / `build()` without an id for a managed tool. Never reported by `validate()`.
    UnresolvedTool,
    /// A workflow tool node or subagent references a system tool.
    SystemToolInWorkflow,
    /// Rust-specific: the assembled config doesn't deserialize into the generated create-agent
    /// request model (`build()` only). `to_json()`, and so `push`, still send it as it is.
    InvalidModel,
}

impl AgentIssueCode {
    /// The snake_case code string, as in the spec.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingName => "missing_name",
            Self::MissingPrompt => "missing_prompt",
            Self::DuplicateToolName => "duplicate_tool_name",
            Self::InvalidTool => "invalid_tool",
            Self::InvalidWorkflow => "invalid_workflow",
            Self::InvalidConfig => "invalid_config",
            Self::OptionConflict => "option_conflict",
            Self::UnresolvedTool => "unresolved_tool",
            Self::SystemToolInWorkflow => "system_tool_in_workflow",
            Self::InvalidModel => "invalid_model",
        }
    }
}

impl fmt::Display for AgentIssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A single problem found in an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentIssue {
    pub code: AgentIssueCode,
    pub severity: Severity,
    pub message: String,
    /// Where the problem is, e.g. `"tools.get_order"` or `"workflow"`.
    pub path: Option<String>,
}

impl AgentIssue {
    pub(crate) fn error(
        code: AgentIssueCode,
        message: impl Into<String>,
        path: Option<String>,
    ) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            path,
        }
    }
}

/// Returned by `Agent::validate()`, `to_json()` and `build()` when the agent is invalid. Lists
/// every problem found, not just the first.
#[derive(Debug, Clone)]
pub struct AgentValidationError {
    /// The agent's name.
    pub agent_name: String,
    pub issues: Vec<AgentIssue>,
}

impl AgentValidationError {
    /// The error codes, in the order they were found.
    pub fn codes(&self) -> Vec<&'static str> {
        self.issues.iter().map(|i| i.code.as_str()).collect()
    }
}

impl fmt::Display for AgentValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid agent \"{}\":", self.agent_name)?;
        for issue in &self.issues {
            write!(f, "\n  - {}", issue.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for AgentValidationError {}

/// A project's files can't be read or written: a missing or malformed file, a path outside the
/// project, or a format the SDK can't read yet (YAML and TOML configs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProjectError {
    pub message: String,
}

impl AgentProjectError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AgentProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for AgentProjectError {}

/// What `push` did to one tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPushAction {
    Created,
    Updated,
    /// The server already had this definition; no request was made.
    Unchanged,
    /// The agent used it before and no longer does. It isn't deleted.
    Detached,
}

impl ToolPushAction {
    /// `"created"`, `"updated"`, `"unchanged"` or `"detached"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Unchanged => "unchanged",
            Self::Detached => "detached",
        }
    }
}

/// What `push` did to the agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentPushAction {
    Created,
    /// Updated in place (its main branch, or the given branch).
    Updated,
}

impl AgentPushAction {
    /// `"created"` or `"updated"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
        }
    }
}

/// One tool `push` touched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolPush {
    pub id: String,
    pub action: ToolPushAction,
}

/// What `push` did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentPushResult {
    pub agent_id: String,
    pub action: AgentPushAction,
    /// Each tool `push` touched, by name.
    pub tools: BTreeMap<String, ToolPush>,
}

/// What `push` wrote before it failed: the tools always, the agent if it got that far.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PartialPushResult {
    pub agent_id: Option<String>,
    pub action: Option<AgentPushAction>,
    pub tools: BTreeMap<String, ToolPush>,
}

/// An API call failed after `push` started writing. `result` lists what was written before the
/// failure, so nothing created goes unreported; with a project opened from a directory, those ids
/// are saved to it too.
#[derive(Debug)]
pub struct AgentPushError {
    /// The step that failed, e.g. `Couldn't create tool "get_order"`.
    pub message: String,
    pub result: PartialPushResult,
    pub cause: Box<dyn std::error::Error + Send + Sync>,
}

impl fmt::Display for AgentPushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.cause)
    }
}

impl std::error::Error for AgentPushError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}

/// Why `agents_as_code().push()` failed.
#[derive(Debug, thiserror::Error)]
pub enum PushError {
    /// The agent is invalid. Nothing was sent.
    #[error(transparent)]
    Validation(#[from] AgentValidationError),
    /// The project can't place the agent or a tool (e.g. a branch it has no file for, or two
    /// entries with the same name). Nothing was sent.
    #[error(transparent)]
    Project(#[from] AgentProjectError),
    /// The options were rejected before any request, e.g. `agent_id` together with `project`.
    #[error("{0}")]
    InvalidOptions(String),
    /// A request failed partway; the error carries what was written before.
    #[error(transparent)]
    Push(#[from] AgentPushError),
}

/// Why `agents_as_code().pull()` failed.
#[derive(Debug, thiserror::Error)]
pub enum PullError {
    /// The project can't take the agent, or couldn't be saved.
    #[error(transparent)]
    Project(#[from] AgentProjectError),
    /// A request failed.
    #[error(transparent)]
    Api(#[from] crate::error::ApiError),
}
