//! Problems found in a workflow. Errors stop `build()`; warnings go to its `on_warning` callback.

use std::fmt;

/// A validation issue code. Matches the shared cross-SDK vocabulary (see the workflow spec).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowIssueCode {
    DuplicateNodeId,
    UnknownNode,
    UnknownEdge,
    DuplicateEdge,
    SelfLoop,
    InvalidOption,
    EdgeIntoStart,
    EndHasOutgoingEdges,
    ToolEdgeNeedsResult,
    ResultEdgeNeedsTool,
    CannotRemoveStart,
    MissingInsertResult,
    UnreachableNode,
    UnknownNodeType,
}

impl WorkflowIssueCode {
    /// The snake_case code string used in fixtures and error messages.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateNodeId => "duplicate_node_id",
            Self::UnknownNode => "unknown_node",
            Self::UnknownEdge => "unknown_edge",
            Self::DuplicateEdge => "duplicate_edge",
            Self::SelfLoop => "self_loop",
            Self::InvalidOption => "invalid_option",
            Self::EdgeIntoStart => "edge_into_start",
            Self::EndHasOutgoingEdges => "end_has_outgoing_edges",
            Self::ToolEdgeNeedsResult => "tool_edge_needs_result",
            Self::ResultEdgeNeedsTool => "result_edge_needs_tool",
            Self::CannotRemoveStart => "cannot_remove_start",
            Self::MissingInsertResult => "missing_insert_result",
            Self::UnreachableNode => "unreachable_node",
            Self::UnknownNodeType => "unknown_node_type",
        }
    }
}

impl fmt::Display for WorkflowIssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Severity of a [`WorkflowIssue`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// A single problem found in a workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowIssue {
    pub code: WorkflowIssueCode,
    pub severity: Severity,
    pub message: String,
    pub node_id: Option<String>,
}

impl WorkflowIssue {
    pub(crate) fn error(
        code: WorkflowIssueCode,
        message: impl Into<String>,
        node_id: Option<String>,
    ) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            node_id,
        }
    }

    pub(crate) fn warning(
        code: WorkflowIssueCode,
        message: impl Into<String>,
        node_id: Option<String>,
    ) -> Self {
        Self {
            code,
            severity: Severity::Warning,
            message: message.into(),
            node_id,
        }
    }
}

/// Returned by [`crate::agent_workflows::Workflow::build`] when the workflow is invalid. Lists
/// every problem found, not just the first.
#[derive(Debug, Clone)]
pub struct WorkflowValidationError {
    pub issues: Vec<WorkflowIssue>,
}

impl WorkflowValidationError {
    /// The error codes, as a sorted list (how fixtures compare them).
    pub fn codes(&self) -> Vec<&'static str> {
        let mut codes: Vec<&'static str> = self.issues.iter().map(|i| i.code.as_str()).collect();
        codes.sort_unstable();
        codes
    }
}

impl fmt::Display for WorkflowValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Invalid workflow:")?;
        for issue in &self.issues {
            writeln!(f, "  - {}", issue.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for WorkflowValidationError {}
