//! Schema checks shared by every tool kind.

use serde_json::Value;

use super::errors::{ToolIssue, ToolIssueCode};
use super::schema::{is_nullable, SchemaKind, SchemaState};

/// Checks a schema and everything under it. `path` names it in messages, e.g. `"body.reason"`.
/// `is_property` is whether the schema is a property (or a param), so `omit()` needs it optional.
pub(crate) fn check_schema(
    s: &SchemaState,
    path: &str,
    is_property: bool,
    issues: &mut Vec<ToolIssue>,
) {
    let error = |code, message: String| ToolIssue::error(code, Some(path), message);
    // The API reads "" as unset, so an empty description or variable name isn't a value source.
    let described = s.description.as_deref().is_some_and(|d| !d.is_empty());
    let sources = [
        described,
        s.dynamic_variable.as_deref().is_some_and(|v| !v.is_empty()),
        s.constant.is_some(),
        s.system_provided,
        s.omitted,
    ]
    .iter()
    .filter(|set| **set)
    .count();
    if sources > 1 {
        issues.push(error(
            ToolIssueCode::ConflictingValueSource,
            format!("\"{path}\" sets more than one of describe(), from_variable(), constant(), system_provided() and omit()"),
        ));
    }
    if s.omitted && is_property && !s.optional {
        issues.push(error(
            ToolIssueCode::OmittedRequired,
            format!("\"{path}\" uses omit(), so it must be optional()"),
        ));
    }
    if s.choices.is_some() && s.kind != SchemaKind::String {
        issues.push(error(
            ToolIssueCode::InvalidChoices,
            format!("\"{path}\" uses choices(), which only applies to strings"),
        ));
    }
    if s.allowed_from.is_some() && !described {
        issues.push(error(
            ToolIssueCode::AllowedValuesNeedsDescription,
            format!("\"{path}\" uses allowed_from(), which needs describe() so the LLM provides the value"),
        ));
    }
    if let Some(constant) = &s.constant {
        if !matches_kind(s.kind, constant, is_nullable(s.literal_type.as_ref())) {
            issues.push(error(
                ToolIssueCode::ConstantTypeMismatch,
                format!("\"{path}\" has a constant that isn't a {}", s.kind.as_str()),
            ));
        } else if constant.as_str() == Some("") {
            issues.push(error(
                ToolIssueCode::ConstantTypeMismatch,
                format!("\"{path}\" can't have \"\" as a constant: the API reads it as unset"),
            ));
        } else if s.kind.is_literal() && constant.as_u64().is_some_and(|u| !fits_model(u)) {
            issues.push(error(
                ToolIssueCode::ConstantTypeMismatch,
                format!("\"{path}\" has the constant {constant}, which is above {} and can't be sent exactly", i64::MAX),
            ));
        }
    }
    // The API rejects a literal with no value source wherever it appears, response schemas too.
    if sources == 0 && s.kind.is_literal() {
        issues.push(error(
            ToolIssueCode::MissingValueSource,
            format!("\"{path}\" needs one of describe(), from_variable(), constant(), system_provided() or omit()"),
        ));
    }

    match s.kind {
        SchemaKind::Object => {
            for group in s.any_of.iter().chain(&s.all_of) {
                for name in group {
                    if !s.properties.iter().any(|(existing, _)| existing == name) {
                        issues.push(error(
                            ToolIssueCode::UnknownRequiredProperty,
                            format!("\"{path}\" requires \"{name}\" in require_any_of()/require_all_of(), but has no such property"),
                        ));
                    }
                }
            }
            for (name, child) in &s.properties {
                check_schema(child.state(), &format!("{path}.{name}"), true, issues);
            }
        }
        SchemaKind::Array => match &s.items {
            // Loaded items of the wrong shape are reported by build() as invalid_model.
            None if s.malformed.contains(&"items") => {}
            None => issues.push(error(
                ToolIssueCode::MissingArrayItems,
                format!("Array \"{path}\" has no items schema"),
            )),
            Some(items) => check_schema(items.state(), &format!("{path}[]"), false, issues),
        },
        _ => {}
    }
}

/// Whether the request model can hold the literal constant `u` exactly. It holds one as an i64 or
/// an f64, so an integer above `i64::MAX` is sent as the nearest f64: exact for a value such as
/// `10000000000000000000` (`1e19`, which `to_json_string()` writes for a whole float as JavaScript
/// does), but rounded, so `build()` would send a different value from `to_json()`, for one such as
/// `9223372036854775809`.
fn fits_model(u: u64) -> bool {
    if i64::try_from(u).is_ok() {
        return true;
    }
    let f = u as f64;
    // `u64::MAX as f64` is 2^64, one past the range, where `as u64` would saturate.
    f < u64::MAX as f64 && f as u64 == u
}

fn matches_kind(kind: SchemaKind, value: &Value, nullable: bool) -> bool {
    if value.is_null() && nullable {
        return true;
    }
    match kind {
        SchemaKind::String => value.is_string(),
        // As in JavaScript, a whole float such as 3.0 is an integer.
        SchemaKind::Integer => match value {
            Value::Number(n) => {
                n.is_i64()
                    || n.is_u64()
                    || n.as_f64()
                        .is_some_and(|f| f.is_finite() && f.fract() == 0.0)
            }
            _ => false,
        },
        SchemaKind::Number => value.is_number(),
        SchemaKind::Boolean => value.is_boolean(),
        SchemaKind::Object => value.is_object(),
        SchemaKind::Array => value.is_array(),
    }
}

/// Reports each loaded field under a schema that has the wrong shape for the generated request
/// model (kept as loaded, so `to_json()` writes it back), as `invalid_model`.
pub(crate) fn check_model_shape(s: &SchemaState, path: &str, issues: &mut Vec<ToolIssue>) {
    for key in &s.malformed {
        issues.push(ToolIssue::error(
            ToolIssueCode::InvalidModel,
            Some(path),
            format!("\"{path}\" has a loaded {key} of the wrong shape, which the generated tool config model can't take. Fix it in the source JSON"),
        ));
    }
    for (name, child) in &s.properties {
        check_model_shape(child.state(), &format!("{path}.{name}"), issues);
    }
    if let Some(items) = &s.items {
        check_model_shape(items.state(), &format!("{path}[]"), issues);
    }
}
