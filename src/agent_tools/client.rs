//! `ClientTool`: a tool that sends an event to the user's client.

use serde_json::{json, Value};

use crate::api::types::ClientToolConfigInput;

use super::errors::{ToolIssue, ToolIssueCode};
use super::options::ToolOptions;
use super::schema::{ObjectSchema, Schema, WireMap};
use super::tool::{accepted_options, set_config_option, tool_methods};
use super::validate::check_schema;
use super::webhook::COMMON_OPTIONS;
use super::ToolKind;

const CLIENT_ONLY_OPTIONS: &[&str] = &[
    "dynamic_variable_placeholders",
    "execution_mode",
    "expects_response",
];

/// A tool that sends an event to the user's client. Create one with
/// [`Tool::client`](super::Tool::client).
///
/// ```
/// use elevenlabs_rust_sdk::agent_tools::{Schema, Tool, ToolOptions};
///
/// let show_map = Tool::client("show_store_map", "Show the caller a map of the nearest store")
///     .param("store_id", Schema::string().describe("The store to show"))
///     .options(ToolOptions::new().set("expects_response", false));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ClientTool {
    /// The tool config fields other than `parameters`.
    config: WireMap,
    /// Problems found while building, reported by `build()`.
    issues: Vec<ToolIssue>,
    parameters: Option<ObjectSchema>,
}

impl ClientTool {
    pub(crate) fn new(name: String, description: String) -> Self {
        let mut config = WireMap::new();
        config.insert("name".into(), json!(name));
        config.insert("description".into(), json!(description));
        Self::load(config, None)
    }

    pub(crate) fn load(config: WireMap, parameters: Option<ObjectSchema>) -> Self {
        ClientTool {
            config,
            issues: Vec::new(),
            parameters,
        }
    }

    tool_methods!(ToolKind::Client, Client, ClientToolConfigInput);

    /// Sets the param `name`, or replaces it in place. Required unless the schema is
    /// `optional()`. Any schema kind.
    pub fn param(mut self, name: impl Into<String>, schema: impl Into<Schema>) -> Self {
        let parameters = self.parameters.take().unwrap_or_else(Schema::object);
        self.parameters = Some(parameters.property(name, schema));
        self
    }

    /// Removes the param `name`. Does nothing if there is none.
    pub fn remove_param(mut self, name: &str) -> Self {
        self.parameters = self.parameters.map(|parameters| parameters.without(name));
        self
    }

    /// The params, as an object schema.
    pub fn parameters_schema(&self) -> Option<&ObjectSchema> {
        self.parameters.as_ref()
    }

    /// Merges options for client tools (see [`ToolOptions`]). A key that doesn't apply is
    /// reported by `build()`.
    pub fn options(mut self, options: ToolOptions) -> Self {
        let allowed: Vec<&str> = COMMON_OPTIONS
            .iter()
            .chain(CLIENT_ONLY_OPTIONS)
            .copied()
            .collect();
        for (key, value) in accepted_options(options, &allowed, "client", &mut self.issues) {
            set_config_option(&mut self.config, &key, value);
        }
        self
    }

    fn validate(&self) -> Vec<ToolIssue> {
        let mut issues = Vec::new();
        if let Some(parameters) = &self.parameters {
            // Checks the params object's own require_any_of()/require_all_of() and every param.
            check_schema(parameters.state(), "params", false, &mut issues);
        }
        let timeout = self
            .config
            .get("response_timeout_secs")
            .and_then(Value::as_f64);
        if let Some(timeout) = timeout {
            if timeout != -1.0 && !(1.0..=120.0).contains(&timeout) {
                issues.push(ToolIssue::error(
                    ToolIssueCode::InvalidOption,
                    Some("response_timeout_secs"),
                    "A client tool's response_timeout_secs must be 1–120, or -1",
                ));
            }
        }
        if timeout == Some(-1.0) && self.config.get("expects_response") != Some(&Value::Bool(true))
        {
            issues.push(ToolIssue::error(
                ToolIssueCode::InvalidOption,
                Some("response_timeout_secs"),
                "response_timeout_secs: -1 needs expects_response: true",
            ));
        }
        issues
    }

    fn to_wire(&self) -> WireMap {
        let mut wire = self.config.clone();
        wire.insert("type".into(), json!("client"));
        if let Some(parameters) = &self.parameters {
            wire.insert("parameters".into(), Value::Object(parameters.to_model()));
        }
        wire
    }
}
