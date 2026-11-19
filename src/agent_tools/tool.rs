//! `Tool`: any kind of tool, the loaders, and the machinery the kinds share.

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::agent_workflows::wire::canonical;
use crate::api::types::{ToolRequestModel, ToolRequestModelToolConfig, ToolResponseModel};

use super::client::ClientTool;
use super::errors::{Severity, ToolIssue, ToolIssueCode, ToolValidationError};
use super::json::{js_canonical, to_json_string};
use super::options::ToolOptions;
use super::schema::{container_schema, strip_nulls, WireMap};
use super::system::SystemTool;
use super::webhook::WebhookTool;

/// What kind of tool this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolKind {
    /// The server calls an HTTP endpoint.
    Webhook,
    /// The agent sends an event to the user's client.
    Client,
    /// A built-in tool, such as ending or transferring the call.
    System,
    /// A loaded tool of a type the builder has no constructor for (e.g. `mcp`). It round-trips
    /// but can't be edited.
    Other,
}

impl ToolKind {
    /// `"webhook"`, `"client"`, `"system"` or `"other"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Webhook => "webhook",
            Self::Client => "client",
            Self::System => "system",
            Self::Other => "other",
        }
    }
}

/// An agent tool, written as code: a webhook, client or system tool, or a loaded tool of another
/// type. Create one with [`Tool::webhook`], [`Tool::client`] or a [`SystemTool`] constructor
/// (each returns its own type, which converts into `Tool`), or load one with [`Tool::from_json`]
/// / [`Tool::from_response`].
#[derive(Debug, Clone, PartialEq)]
pub enum Tool {
    Webhook(WebhookTool),
    Client(ClientTool),
    System(SystemTool),
    Other(OtherTool),
}

impl From<WebhookTool> for Tool {
    fn from(tool: WebhookTool) -> Self {
        Tool::Webhook(tool)
    }
}

impl From<ClientTool> for Tool {
    fn from(tool: ClientTool) -> Self {
        Tool::Client(tool)
    }
}

impl From<SystemTool> for Tool {
    fn from(tool: SystemTool) -> Self {
        Tool::System(tool)
    }
}

impl Tool {
    /// A tool the server runs by calling an HTTP endpoint. Set the method and URL with `get()`,
    /// `post()`...
    pub fn webhook(name: impl Into<String>, description: impl Into<String>) -> WebhookTool {
        WebhookTool::new(name.into(), description.into())
    }

    /// A tool that sends an event to the user's client.
    pub fn client(name: impl Into<String>, description: impl Into<String>) -> ClientTool {
        ClientTool::new(name.into(), description.into())
    }

    /// Loads a tool stored in API format (snake_case), for example from `to_json_string()` or a
    /// CLI `tool_configs/*.json` file: a tool config, or a request or response with a
    /// `tool_config` key. Returns a tool of the matching kind; a type the builder has no
    /// constructor for loads as [`ToolKind::Other`] and is kept exactly as loaded.
    pub fn from_json(json: &Value) -> Tool {
        let raw = match json.get("tool_config") {
            Some(config @ Value::Object(_)) => config,
            _ => json,
        };
        let raw = raw.as_object().cloned().unwrap_or_default();
        let kind = raw.get("type").and_then(Value::as_str).map(str::to_string);
        if !matches!(kind.as_deref(), Some("webhook" | "client" | "system")) {
            return Tool::Other(OtherTool {
                raw,
                issues: Vec::new(),
            });
        }
        let mut config = match strip_nulls(Value::Object(raw)) {
            Value::Object(map) => map,
            _ => WireMap::new(),
        };
        config.remove("type");
        match kind.as_deref() {
            Some("webhook") => {
                let api = match config.remove("api_schema") {
                    Some(Value::Object(api)) => api,
                    _ => WireMap::new(),
                };
                Tool::Webhook(WebhookTool::load(config, api))
            }
            Some("client") => {
                // Params that aren't an object stay in `config`, written back as loaded, and
                // build() reports them (`invalid_model`).
                let parameters = match config.get("parameters") {
                    Some(Value::Object(params)) => Some(container_schema(params)),
                    _ => None,
                };
                if parameters.is_some() {
                    config.remove("parameters");
                }
                Tool::Client(ClientTool::load(config, parameters))
            }
            _ => {
                // Params that aren't an object with a `system_tool_type` (or are missing) stay
                // in `config`, written back as loaded, and build() reports them
                // (`invalid_model`): the generated params enum's catch-all would accept them.
                let well_formed = config
                    .get("params")
                    .and_then(Value::as_object)
                    .and_then(|params| params.get("system_tool_type"))
                    .and_then(Value::as_str)
                    .is_some_and(|t| !t.is_empty());
                if !well_formed {
                    return Tool::System(SystemTool::load_malformed(config));
                }
                let Some(Value::Object(mut params)) = config.remove("params") else {
                    unreachable!("checked above");
                };
                let system_tool_type = match params.remove("system_tool_type") {
                    Some(Value::String(t)) => t,
                    _ => unreachable!("checked above"),
                };
                Tool::System(SystemTool::load(system_tool_type, config, params))
            }
        }
    }

    /// Loads a tool from an API-format JSON string.
    pub fn from_json_str(json: &str) -> Result<Tool, serde_json::Error> {
        Ok(Tool::from_json(&serde_json::from_str(json)?))
    }

    /// Loads the result of `tools.get()`.
    pub fn from_response(response: &ToolResponseModel) -> Tool {
        Tool::from_json(&serde_json::to_value(&response.tool_config).unwrap_or(Value::Null))
    }

    /// `Webhook`, `Client`, `System`, or `Other` for a loaded tool of a type this SDK can't edit.
    pub fn kind(&self) -> ToolKind {
        match self {
            Tool::Webhook(_) => ToolKind::Webhook,
            Tool::Client(_) => ToolKind::Client,
            Tool::System(_) => ToolKind::System,
            Tool::Other(_) => ToolKind::Other,
        }
    }

    /// The tool's name, as the LLM sees it.
    pub fn name(&self) -> &str {
        match self {
            Tool::Webhook(t) => t.name(),
            Tool::Client(t) => t.name(),
            Tool::System(t) => t.name(),
            Tool::Other(t) => t.name(),
        }
    }

    /// Merges options for this kind of tool. A key that doesn't apply is reported by `build()`.
    pub fn options(self, options: ToolOptions) -> Tool {
        match self {
            Tool::Webhook(t) => Tool::Webhook(t.options(options)),
            Tool::Client(t) => Tool::Client(t.options(options)),
            Tool::System(t) => Tool::System(t.options(options)),
            Tool::Other(t) => Tool::Other(t.options(options)),
        }
    }

    /// This tool as a webhook tool, or `None` if it's another kind.
    pub fn as_webhook(&self) -> Option<&WebhookTool> {
        match self {
            Tool::Webhook(t) => Some(t),
            _ => None,
        }
    }

    /// This tool as a client tool, or `None` if it's another kind.
    pub fn as_client(&self) -> Option<&ClientTool> {
        match self {
            Tool::Client(t) => Some(t),
            _ => None,
        }
    }

    /// This tool as a system tool, or `None` if it's another kind.
    pub fn as_system(&self) -> Option<&SystemTool> {
        match self {
            Tool::System(t) => Some(t),
            _ => None,
        }
    }

    /// This tool as a webhook tool, for editing, or `None` if it's another kind.
    pub fn into_webhook(self) -> Option<WebhookTool> {
        match self {
            Tool::Webhook(t) => Some(t),
            _ => None,
        }
    }

    /// This tool as a client tool, for editing, or `None` if it's another kind.
    pub fn into_client(self) -> Option<ClientTool> {
        match self {
            Tool::Client(t) => Some(t),
            _ => None,
        }
    }

    /// This tool as a system tool, for editing, or `None` if it's another kind.
    pub fn into_system(self) -> Option<SystemTool> {
        match self {
            Tool::System(t) => Some(t),
            _ => None,
        }
    }

    /// Validates the tool and returns the model `tools.create()` / `tools.update()` accept.
    pub fn build(&self) -> Result<ToolRequestModel, ToolValidationError> {
        self.build_with_warnings(|_| {})
    }

    /// Like [`build`](Self::build), but `on_warning` is called once per non-fatal problem.
    pub fn build_with_warnings<F: FnMut(&ToolIssue)>(
        &self,
        on_warning: F,
    ) -> Result<ToolRequestModel, ToolValidationError> {
        Ok(request_model(self.build_config_with_warnings(on_warning)?))
    }

    /// Validates the tool and returns only its config. Webhook and client tools go to
    /// `tools.create()` and are attached to an agent by id in `prompt.tool_ids`; for a system tool,
    /// [`SystemTool::build_config`] returns its `prompt.built_in_tools` entry.
    pub fn build_config(&self) -> Result<ToolRequestModelToolConfig, ToolValidationError> {
        self.build_config_with_warnings(|_| {})
    }

    /// Like [`build_config`](Self::build_config), but `on_warning` is called once per non-fatal
    /// problem.
    pub fn build_config_with_warnings<F: FnMut(&ToolIssue)>(
        &self,
        on_warning: F,
    ) -> Result<ToolRequestModelToolConfig, ToolValidationError> {
        Ok(match self {
            Tool::Webhook(t) => ToolRequestModelToolConfig::Webhook {
                data: t.build_config_with_warnings(on_warning)?,
            },
            Tool::Client(t) => ToolRequestModelToolConfig::Client {
                data: t.build_config_with_warnings(on_warning)?,
            },
            Tool::System(t) => ToolRequestModelToolConfig::System {
                data: t.build_config_with_warnings(on_warning)?,
            },
            Tool::Other(t) => t.build_config_with_warnings(on_warning)?,
        })
    }

    /// The tool config in API format (snake_case keys, sorted at every level), the shape of a
    /// CLI `tool_configs/*.json` file. Validates like `build()`.
    pub fn to_json(&self) -> Result<Value, ToolValidationError> {
        match self {
            Tool::Webhook(t) => t.to_json(),
            Tool::Client(t) => t.to_json(),
            Tool::System(t) => t.to_json(),
            Tool::Other(t) => t.to_json(),
        }
    }

    /// `to_json()` as a string with 2-space indentation and a trailing newline: the canonical
    /// file format.
    pub fn to_json_string(&self) -> Result<String, ToolValidationError> {
        Ok(to_json_string(&self.to_json()?))
    }
}

/// A loaded tool of a type the builder has no constructor for, such as `mcp`. Kept exactly as
/// loaded; `build()` warns with `unknown_tool_type`.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherTool {
    raw: WireMap,
    issues: Vec<ToolIssue>,
}

impl OtherTool {
    /// The tool's name, as the LLM sees it.
    pub fn name(&self) -> &str {
        name_of(&self.raw)
    }

    /// The loaded `type`, e.g. `"mcp"`.
    pub fn tool_type(&self) -> Option<&str> {
        self.raw.get("type").and_then(Value::as_str)
    }

    /// No option applies to a tool of a type the builder can't edit: every key is reported by
    /// `build()` as `invalid_option`.
    pub fn options(mut self, options: ToolOptions) -> Self {
        accepted_options(options, &[], "other", &mut self.issues);
        self
    }

    /// Validates (warning `unknown_tool_type`) and returns the model `tools.create()` accepts.
    pub fn build(&self) -> Result<ToolRequestModel, ToolValidationError> {
        self.build_with_warnings(|_| {})
    }

    /// Like [`build`](Self::build), but `on_warning` is called once per non-fatal problem.
    pub fn build_with_warnings<F: FnMut(&ToolIssue)>(
        &self,
        on_warning: F,
    ) -> Result<ToolRequestModel, ToolValidationError> {
        Ok(request_model(self.build_config_with_warnings(on_warning)?))
    }

    /// The loaded config as the generated tool config: its own variant if the generated types
    /// model it, else the catch-all variant holding the loaded JSON.
    pub fn build_config(&self) -> Result<ToolRequestModelToolConfig, ToolValidationError> {
        self.build_config_with_warnings(|_| {})
    }

    /// Like [`build_config`](Self::build_config), but `on_warning` is called once per non-fatal
    /// problem.
    pub fn build_config_with_warnings<F: FnMut(&ToolIssue)>(
        &self,
        on_warning: F,
    ) -> Result<ToolRequestModelToolConfig, ToolValidationError> {
        check(self.name(), self.all_issues(), on_warning)?;
        let raw = Value::Object(self.raw.clone());
        Ok(serde_json::from_value(canonical(&raw))
            .unwrap_or(ToolRequestModelToolConfig::__Unknown(raw)))
    }

    /// The loaded config in API format, keys sorted. Validates like `build()`.
    pub fn to_json(&self) -> Result<Value, ToolValidationError> {
        check(self.name(), self.all_issues(), |_| {})?;
        Ok(js_canonical(&Value::Object(self.raw.clone())))
    }

    /// `to_json()` as a string with 2-space indentation and a trailing newline.
    pub fn to_json_string(&self) -> Result<String, ToolValidationError> {
        Ok(to_json_string(&self.to_json()?))
    }

    fn all_issues(&self) -> Vec<ToolIssue> {
        let mut issues = self.issues.clone();
        issues.push(ToolIssue::warning(
            ToolIssueCode::UnknownToolType,
            format!(
                "Tool \"{}\" has type \"{}\", which this SDK can't edit. It round-trips through to_json(), but build()'s output may be rejected by tools.create() and tools.update()",
                self.name(),
                self.tool_type().unwrap_or("")
            ),
        ));
        issues
    }
}

pub(crate) fn request_model(tool_config: ToolRequestModelToolConfig) -> ToolRequestModel {
    ToolRequestModel {
        tool_config,
        response_mocks: None,
    }
}

pub(crate) fn name_of(config: &WireMap) -> &str {
    config.get("name").and_then(Value::as_str).unwrap_or("")
}

/// Sets `key` in `map`, or removes it when `value` is `null`.
pub(crate) fn set_value(map: &mut WireMap, key: &str, value: Value) {
    if value.is_null() {
        map.remove(key);
    } else {
        map.insert(key.to_string(), value);
    }
}

/// Splits `options` into the ones in `allowed`, returned in order, and the rest, recorded as
/// `invalid_option` issues.
pub(crate) fn accepted_options(
    options: ToolOptions,
    allowed: &[&str],
    kind: &str,
    issues: &mut Vec<ToolIssue>,
) -> Vec<(String, Value)> {
    let mut accepted = Vec::new();
    for (key, value) in options.raw {
        if allowed.contains(&key.as_str()) {
            accepted.push((key, value));
        } else {
            issues.push(ToolIssue::error(
                ToolIssueCode::InvalidOption,
                Some(&key),
                format!("Option \"{key}\" doesn't apply to {kind} tools"),
            ));
        }
    }
    accepted
}

/// Sets an option that lives on the tool config itself.
pub(crate) fn set_config_option(config: &mut WireMap, key: &str, value: Value) {
    if key == "dynamic_variable_placeholders" {
        let mut variables = match config.remove("dynamic_variables") {
            Some(Value::Object(variables)) => variables,
            _ => WireMap::new(),
        };
        set_value(&mut variables, "dynamic_variable_placeholders", value);
        if !variables.is_empty() {
            config.insert("dynamic_variables".into(), Value::Object(variables));
        }
    } else {
        set_value(config, key, value);
    }
}

/// Reports `issues`: errors stop the build, warnings go to `on_warning`.
pub(crate) fn check<F: FnMut(&ToolIssue)>(
    name: &str,
    issues: Vec<ToolIssue>,
    mut on_warning: F,
) -> Result<(), ToolValidationError> {
    let (errors, warnings): (Vec<ToolIssue>, Vec<ToolIssue>) = issues
        .into_iter()
        .partition(|issue| issue.severity == Severity::Error);
    if !errors.is_empty() {
        return Err(ToolValidationError {
            tool_name: name.to_string(),
            issues: errors,
        });
    }
    for warning in &warnings {
        on_warning(warning);
    }
    Ok(())
}

/// Deserializes validated wire output into a generated type. Validation covers the builder's
/// rules, not every loaded field's shape, so a loaded field of the wrong shape can still fail
/// here; that is reported as a validation error, never a panic.
pub(crate) fn to_model<T: DeserializeOwned + Serialize>(
    name: &str,
    mut wire: WireMap,
) -> Result<T, ToolValidationError> {
    // The generated config structs are the variants of a `type`-tagged enum, so they have no
    // `type` field of their own.
    wire.remove("type");
    // Whole floats become integers here (and only here, not in `to_json()`), since the generated
    // integer fields don't accept a float such as 5.0.
    let wire = canonical(&Value::Object(wire));
    let model: T = serde_json::from_value(wire.clone()).map_err(|err| ToolValidationError {
        tool_name: name.to_string(),
        issues: vec![ToolIssue::error(
            ToolIssueCode::InvalidModel,
            None,
            format!("The tool doesn't match the generated tool config model: {err}. A loaded field likely has the wrong shape; fix it in the source JSON"),
        )],
    })?;
    // The generated model must send every constant exactly as `to_json()` writes it. It doesn't
    // for a nullable literal's `null` constant (it reads `null` as unset), and validation can't
    // foresee every other conversion, so compare: never change or drop a constant silently.
    let wanted = constants(&wire);
    if !wanted.is_empty() {
        let sent = serde_json::to_value(&model).map_or_else(|_| Vec::new(), |v| constants(&v));
        let issues: Vec<ToolIssue> = wanted
            .into_iter()
            .filter_map(|(path, value)| {
                let sent = sent.iter().find(|(p, _)| *p == path).map(|(_, v)| v);
                if sent.is_some_and(|sent| same_value(sent, &value)) {
                    return None;
                }
                let at = display_path(&path);
                let message = match sent {
                    _ if value.is_null() => format!("\"{at}\" has the constant null, which this SDK's generated request model can't send (it reads null as unset). Send the to_json() output directly, or use another constant"),
                    Some(sent) => format!("\"{at}\" has the constant {value}, which this SDK's generated request model would send as {sent}. Use a constant it can represent exactly"),
                    None => format!("\"{at}\" has the constant {value}, which this SDK's generated request model would drop. Send the to_json() output directly, or use another constant"),
                };
                Some(ToolIssue::error(
                    ToolIssueCode::ConstantTypeMismatch,
                    Some(&at),
                    message,
                ))
            })
            .collect();
        if !issues.is_empty() {
            return Err(ToolValidationError {
                tool_name: name.to_string(),
                issues,
            });
        }
    }
    Ok(model)
}

/// Whether `a` and `b` are the same JSON value, numbers compared by their exact value: the
/// generated literal constant holds an integer above `i64::MAX` (e.g. `10000000000000000000`) as
/// the f64 it equals, which is the same number, not a changed one.
fn same_value(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => {
            a == b || exact(a).is_some_and(|a| Some(a) == exact(b))
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_value(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, a)| b.get(key).is_some_and(|b| same_value(a, b)))
        }
        _ => a == b,
    }
}

/// An integer, or a whole float, as the exact integer it is.
fn exact(n: &serde_json::Number) -> Option<i128> {
    if let Some(i) = n.as_i64() {
        return Some(i.into());
    }
    if let Some(u) = n.as_u64() {
        return Some(u.into());
    }
    // Every whole f64 below 2^127 in magnitude converts to i128 exactly.
    n.as_f64()
        .filter(|f| f.fract() == 0.0 && f.abs() < 2f64.powi(127))
        .map(|f| f as i128)
}

/// A step from a tool config down to one of its schemas' constants.
#[derive(Debug, Clone, PartialEq)]
enum Step {
    /// A config or schema field.
    Key(String),
    /// A property or path param name: never a schema field, even when it is spelled like one.
    Name(String),
    Index(usize),
}

/// Every schema constant in a tool config (wire format, or the generated model serialized), by
/// where it is. Free-form JSON values (constants, dynamic variable placeholders, headers) aren't
/// schemas, so they aren't searched.
fn constants(value: &Value) -> Vec<(Vec<Step>, Value)> {
    fn walk(value: &Value, path: &mut Vec<Step>, out: &mut Vec<(Vec<Step>, Value)>) {
        match value {
            Value::Object(map) => {
                for (key, v) in map {
                    match key.as_str() {
                        "constant_value" => out.push((path.clone(), v.clone())),
                        "dynamic_variable_placeholders" | "request_headers" => {}
                        // Names mapped to schemas: walk each schema, so a param named
                        // `constant_value` isn't read as a constant.
                        "properties" | "path_params_schema" => {
                            path.push(Step::Key(key.clone()));
                            for (name, schema) in v.as_object().into_iter().flatten() {
                                path.push(Step::Name(name.clone()));
                                walk(schema, path, out);
                                path.pop();
                            }
                            path.pop();
                        }
                        _ => {
                            path.push(Step::Key(key.clone()));
                            walk(v, path, out);
                            path.pop();
                        }
                    }
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    path.push(Step::Index(index));
                    walk(item, path, out);
                    path.pop();
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(value, &mut Vec::new(), &mut out);
    out
}

/// A constant's location as validation names it, e.g. `"body.order.id"` or `"params.tags[]"`.
fn display_path(path: &[Step]) -> String {
    let mut out = String::new();
    for step in path {
        let label = match step {
            Step::Name(name) => name.as_str(),
            Step::Index(index) => {
                out.push_str(&format!("[{index}]"));
                continue;
            }
            Step::Key(key) => match key.as_str() {
                "api_schema" | "properties" => continue,
                "items" => {
                    out.push_str("[]");
                    continue;
                }
                "path_params_schema" => "path_params",
                "query_params_schema" => "query",
                "request_body_schema" => "body",
                "response_body_schema" => "response",
                "parameters" => "params",
                other => other,
            },
        };
        if !out.is_empty() {
            out.push('.');
        }
        out.push_str(label);
    }
    out
}

/// The public methods every editable tool kind shares. The type it's used in provides `config`
/// and `issues` fields and `validate()` / `to_wire()` / `model_issues()` methods. `validate()`
/// covers the builder's rules (checked by `to_json()` too); `model_issues()` covers loaded input
/// that `to_json()` writes back as loaded but the generated request model can't take, so only
/// `build()` reports it.
macro_rules! tool_methods {
    ($kind:expr, $variant:ident, $config:ty) => {
        /// The tool's name, as the LLM sees it.
        pub fn name(&self) -> &str {
            $crate::agent_tools::tool::name_of(&self.config)
        }

        /// The kind of tool this is.
        pub fn kind(&self) -> $crate::agent_tools::ToolKind {
            $kind
        }

        /// Validates the tool and returns the model `tools.create()` / `tools.update()` accept.
        pub fn build(
            &self,
        ) -> Result<$crate::api::types::ToolRequestModel, $crate::agent_tools::ToolValidationError>
        {
            self.build_with_warnings(|_| {})
        }

        /// Like [`build`](Self::build), but `on_warning` is called once per non-fatal problem.
        pub fn build_with_warnings<F: FnMut(&$crate::agent_tools::ToolIssue)>(
            &self,
            on_warning: F,
        ) -> Result<$crate::api::types::ToolRequestModel, $crate::agent_tools::ToolValidationError>
        {
            let data = self.build_config_with_warnings(on_warning)?;
            Ok($crate::agent_tools::tool::request_model(
                $crate::api::types::ToolRequestModelToolConfig::$variant { data },
            ))
        }

        /// Validates the tool and returns only its config, as the generated config type.
        pub fn build_config(&self) -> Result<$config, $crate::agent_tools::ToolValidationError> {
            self.build_config_with_warnings(|_| {})
        }

        /// Like [`build_config`](Self::build_config), but `on_warning` is called once per
        /// non-fatal problem.
        pub fn build_config_with_warnings<F: FnMut(&$crate::agent_tools::ToolIssue)>(
            &self,
            on_warning: F,
        ) -> Result<$config, $crate::agent_tools::ToolValidationError> {
            let wire = self.checked_wire(on_warning)?;
            $crate::agent_tools::tool::check(self.name(), self.model_issues(), |_| {})?;
            $crate::agent_tools::tool::to_model(self.name(), wire)
        }

        /// The tool config in API format (snake_case keys, sorted at every level), the shape of a
        /// CLI `tool_configs/*.json` file. Validates like `build()`.
        pub fn to_json(
            &self,
        ) -> Result<serde_json::Value, $crate::agent_tools::ToolValidationError> {
            let wire = self.checked_wire(|_| {})?;
            Ok($crate::agent_tools::json::js_canonical(
                &serde_json::Value::Object(wire),
            ))
        }

        /// `to_json()` as a string with 2-space indentation and a trailing newline: the canonical
        /// file format.
        pub fn to_json_string(&self) -> Result<String, $crate::agent_tools::ToolValidationError> {
            Ok($crate::agent_tools::json::to_json_string(&self.to_json()?))
        }

        fn checked_wire<F: FnMut(&$crate::agent_tools::ToolIssue)>(
            &self,
            on_warning: F,
        ) -> Result<$crate::agent_tools::schema::WireMap, $crate::agent_tools::ToolValidationError>
        {
            let mut issues = self.issues.clone();
            issues.extend(self.validate());
            $crate::agent_tools::tool::check(self.name(), issues, on_warning)?;
            Ok(self.to_wire())
        }
    };
}

pub(crate) use tool_methods;
