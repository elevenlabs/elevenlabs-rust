//! Problems found in a tool. Errors stop `build()`; warnings go to its `on_warning` callback.

use std::fmt;

pub use crate::agent_workflows::Severity;

/// A validation issue code. Matches the shared cross-SDK vocabulary (see the tool spec).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolIssueCode {
    MissingUrl,
    MissingPathParam,
    UnknownPathParam,
    ParamNotLiteral,
    BodyNotObject,
    BodyNotAllowed,
    MissingBody,
    DuplicateParam,
    ConflictingValueSource,
    OmittedRequired,
    InvalidChoices,
    AllowedValuesNeedsDescription,
    ConstantTypeMismatch,
    UnknownRequiredProperty,
    MissingArrayItems,
    DuplicateHeader,
    InvalidDestination,
    NoTransfers,
    InvalidOption,
    MissingValueSource,
    UnknownToolType,
    /// Rust-specific: the validated tool still doesn't deserialize into the generated request
    /// model (e.g. a loaded field has the wrong shape). Other SDKs surface their native
    /// deserialization error here instead.
    InvalidModel,
}

impl ToolIssueCode {
    /// The snake_case code string used in fixtures and error messages.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingUrl => "missing_url",
            Self::MissingPathParam => "missing_path_param",
            Self::UnknownPathParam => "unknown_path_param",
            Self::ParamNotLiteral => "param_not_literal",
            Self::BodyNotObject => "body_not_object",
            Self::BodyNotAllowed => "body_not_allowed",
            Self::MissingBody => "missing_body",
            Self::DuplicateParam => "duplicate_param",
            Self::ConflictingValueSource => "conflicting_value_source",
            Self::OmittedRequired => "omitted_required",
            Self::InvalidChoices => "invalid_choices",
            Self::AllowedValuesNeedsDescription => "allowed_values_needs_description",
            Self::ConstantTypeMismatch => "constant_type_mismatch",
            Self::UnknownRequiredProperty => "unknown_required_property",
            Self::MissingArrayItems => "missing_array_items",
            Self::DuplicateHeader => "duplicate_header",
            Self::InvalidDestination => "invalid_destination",
            Self::NoTransfers => "no_transfers",
            Self::InvalidOption => "invalid_option",
            Self::MissingValueSource => "missing_value_source",
            Self::UnknownToolType => "unknown_tool_type",
            Self::InvalidModel => "invalid_model",
        }
    }
}

impl fmt::Display for ToolIssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A single problem found in a tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolIssue {
    pub code: ToolIssueCode,
    pub severity: Severity,
    pub message: String,
    /// Where the problem is, e.g. `"body.reason"` or `"path_params.order_id"`.
    pub path: Option<String>,
}

impl ToolIssue {
    pub(crate) fn error(
        code: ToolIssueCode,
        path: Option<&str>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            path: path.map(str::to_string),
        }
    }

    pub(crate) fn warning(code: ToolIssueCode, message: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Warning,
            message: message.into(),
            path: None,
        }
    }
}

/// Returned by a tool's `build()` when the tool is invalid. Lists every problem found, not just
/// the first.
#[derive(Debug, Clone)]
pub struct ToolValidationError {
    /// The tool's name.
    pub tool_name: String,
    pub issues: Vec<ToolIssue>,
}

impl ToolValidationError {
    /// The error codes, as a sorted list (how fixtures compare them).
    pub fn codes(&self) -> Vec<&'static str> {
        let mut codes: Vec<&'static str> = self.issues.iter().map(|i| i.code.as_str()).collect();
        codes.sort_unstable();
        codes
    }
}

impl fmt::Display for ToolValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid tool \"{}\":", self.tool_name)?;
        for issue in &self.issues {
            write!(f, "\n  - {}", issue.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ToolValidationError {}
