//! Workflow expressions, compiled to the API's expression AST (kept in wire format).

use serde_json::{json, Value};

/// A value accepted wherever an expression is expected. Plain values become literals.
pub enum ExprInput {
    Expr(Expr),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl From<Expr> for ExprInput {
    fn from(e: Expr) -> Self {
        ExprInput::Expr(e)
    }
}
impl From<&str> for ExprInput {
    fn from(s: &str) -> Self {
        ExprInput::Str(s.to_string())
    }
}
impl From<String> for ExprInput {
    fn from(s: String) -> Self {
        ExprInput::Str(s)
    }
}
impl From<i64> for ExprInput {
    fn from(v: i64) -> Self {
        ExprInput::Int(v)
    }
}
impl From<f64> for ExprInput {
    fn from(v: f64) -> Self {
        ExprInput::Float(v)
    }
}
impl From<bool> for ExprInput {
    fn from(v: bool) -> Self {
        ExprInput::Bool(v)
    }
}

/// The type of an LLM-extracted value.
#[derive(Debug, Clone, Copy)]
pub enum LlmValueType {
    Boolean,
    String,
    Integer,
    Number,
}

impl LlmValueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::String => "string",
            Self::Integer => "integer",
            Self::Number => "number",
        }
    }
}

/// A workflow expression, compiled to the API's expression AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    node: Value,
}

impl Expr {
    fn new(node: Value) -> Self {
        Self { node }
    }

    /// The value of a dynamic variable.
    pub fn variable(name: &str) -> Expr {
        Expr::new(json!({ "type": "dynamic_variable", "name": name }))
    }

    /// A string literal.
    pub fn literal_str(value: &str) -> Expr {
        Expr::new(json!({ "type": "string_literal", "value": value }))
    }
    /// An integer literal.
    pub fn literal_int(value: i64) -> Expr {
        Expr::new(json!({ "type": "number_literal", "value": value }))
    }
    /// A floating-point literal.
    pub fn literal_float(value: f64) -> Expr {
        Expr::new(json!({ "type": "number_literal", "value": value }))
    }
    /// A boolean literal.
    pub fn literal_bool(value: bool) -> Expr {
        Expr::new(json!({ "type": "boolean_literal", "value": value }))
    }
    /// A null literal.
    pub fn literal_null() -> Expr {
        Expr::new(json!({ "type": "null_literal" }))
    }

    /// A value the LLM extracts from the conversation. `type` defaults to boolean; `enum` lists the
    /// allowed values of a string.
    pub fn llm(description: &str, value_type: LlmValueType, r#enum: Option<&[&str]>) -> Expr {
        let mut schema = json!({ "type": value_type.as_str(), "description": description });
        if let Some(values) = r#enum {
            schema["enum"] = Value::Array(values.iter().map(|v| json!(v)).collect());
        }
        Expr::new(json!({ "type": "llm", "value_schema": schema }))
    }

    /// True when every expression is true.
    pub fn all_of<I>(exprs: I) -> Expr
    where
        I: IntoIterator<Item = ExprInput>,
    {
        Expr::new(json!({ "type": "and_operator", "children": children(exprs) }))
    }

    /// True when at least one expression is true.
    pub fn any_of<I>(exprs: I) -> Expr
    where
        I: IntoIterator<Item = ExprInput>,
    {
        Expr::new(json!({ "type": "or_operator", "children": children(exprs) }))
    }

    /// `when_true` if `condition` is true, otherwise `when_false`.
    pub fn if_else(
        condition: impl Into<ExprInput>,
        when_true: impl Into<ExprInput>,
        when_false: impl Into<ExprInput>,
    ) -> Expr {
        Expr::new(json!({
            "type": "conditional_operator",
            "condition": to_ast(condition.into()),
            "trueExpression": to_ast(when_true.into()),
            "falseExpression": to_ast(when_false.into()),
        }))
    }

    /// Wrap an existing AST node given as wire-format JSON.
    pub fn from_model(node: Value) -> Expr {
        Expr::new(node)
    }

    pub fn eq(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("eq_operator", other.into())
    }
    pub fn neq(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("neq_operator", other.into())
    }
    pub fn gt(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("gt_operator", other.into())
    }
    pub fn gte(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("gte_operator", other.into())
    }
    pub fn lt(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("lt_operator", other.into())
    }
    pub fn lte(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("lte_operator", other.into())
    }
    pub fn add(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("add_operator", other.into())
    }
    pub fn sub(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("sub_operator", other.into())
    }
    pub fn mul(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("mul_operator", other.into())
    }
    pub fn div(&self, other: impl Into<ExprInput>) -> Expr {
        self.binary("div_operator", other.into())
    }

    /// The expression in API wire format.
    pub fn to_wire(&self) -> Value {
        self.node.clone()
    }

    fn binary(&self, op: &str, other: ExprInput) -> Expr {
        Expr::new(json!({ "type": op, "left": self.to_wire(), "right": to_ast(other) }))
    }
}

fn children<I>(exprs: I) -> Vec<Value>
where
    I: IntoIterator<Item = ExprInput>,
{
    exprs.into_iter().map(to_ast).collect()
}

pub(crate) fn to_ast(value: ExprInput) -> Value {
    match value {
        ExprInput::Expr(e) => e.to_wire(),
        ExprInput::Str(s) => Expr::literal_str(&s).to_wire(),
        ExprInput::Int(v) => Expr::literal_int(v).to_wire(),
        ExprInput::Float(v) => Expr::literal_float(v).to_wire(),
        ExprInput::Bool(v) => Expr::literal_bool(v).to_wire(),
        ExprInput::Null => Expr::literal_null().to_wire(),
    }
}
