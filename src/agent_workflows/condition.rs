//! When an edge may be followed (kept in wire format).

use serde_json::{json, Value};

use super::expr::Expr;

/// When an edge may be followed. A plain string is shorthand for [`Condition::llm`] and an [`Expr`]
/// for [`Condition::expr`]; both are accepted via [`ConditionInput`].
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    model: Value,
}

impl Condition {
    fn new(mut model: Value, label: Option<&str>) -> Condition {
        if let Some(label) = label {
            model["label"] = json!(label);
        }
        Condition { model }
    }

    /// Followed when the LLM judges `condition` to be true.
    pub fn llm(condition: &str, label: Option<&str>) -> Condition {
        Condition::new(json!({ "type": "llm", "condition": condition }), label)
    }

    /// Followed when `expression` evaluates to true.
    pub fn expr(expression: &Expr, label: Option<&str>) -> Condition {
        Condition::new(
            json!({ "type": "expression", "expression": expression.to_wire() }),
            label,
        )
    }

    /// Always followed.
    pub fn always(label: Option<&str>) -> Condition {
        Condition::new(json!({ "type": "unconditional" }), label)
    }

    /// Followed when every tool in a tool node succeeded.
    pub fn success(label: Option<&str>) -> Condition {
        Condition::new(json!({ "type": "result", "successful": true }), label)
    }

    /// Followed when any tool in a tool node failed.
    pub fn failure(label: Option<&str>) -> Condition {
        Condition::new(json!({ "type": "result", "successful": false }), label)
    }

    /// The condition in API wire format.
    pub fn to_wire(&self) -> Value {
        self.model.clone()
    }
}

/// A value accepted wherever a condition is expected: a [`Condition`], an [`Expr`] (becomes
/// `Condition::expr`), or a string (becomes `Condition::llm`).
pub enum ConditionInput {
    Condition(Condition),
    Expr(Expr),
    Str(String),
}

impl From<Condition> for ConditionInput {
    fn from(c: Condition) -> Self {
        ConditionInput::Condition(c)
    }
}
impl From<Expr> for ConditionInput {
    fn from(e: Expr) -> Self {
        ConditionInput::Expr(e)
    }
}
impl From<&str> for ConditionInput {
    fn from(s: &str) -> Self {
        ConditionInput::Str(s.to_string())
    }
}
impl From<String> for ConditionInput {
    fn from(s: String) -> Self {
        ConditionInput::Str(s)
    }
}

pub(crate) fn to_condition(value: ConditionInput) -> Condition {
    match value {
        ConditionInput::Condition(c) => c,
        ConditionInput::Expr(e) => Condition::expr(&e, None),
        ConditionInput::Str(s) => Condition::llm(&s, None),
    }
}
