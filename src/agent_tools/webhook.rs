//! `WebhookTool`: a tool the server runs by calling an HTTP endpoint.

use std::collections::HashMap;

use serde_json::{json, Value};

use crate::api::types::WebhookToolConfigInput;

use super::errors::{ToolIssue, ToolIssueCode};
use super::header::{AuthConnection, Header};
use super::options::{AssignOptions, ToolOptions};
use super::schema::{container_schema, kind_of, ObjectSchema, Schema, SchemaKind, WireMap};
use super::tool::{accepted_options, set_config_option, set_value, tool_methods};
use super::validate::{check_model_shape, check_schema};
use super::ToolKind;

pub(crate) const COMMON_OPTIONS: &[&str] = &[
    "name",
    "description",
    "response_timeout_secs",
    "interruption_mode",
    "pre_tool_speech",
    "tool_call_sound",
    "tool_call_sound_behavior",
    "tool_error_handling_mode",
];

const WEBHOOK_ONLY_OPTIONS: &[&str] = &[
    "dynamic_variable_placeholders",
    "execution_mode",
    "content_type",
    "response_filter",
    "follow_redirects",
    "follow_redirects_allowed_domains",
];

/// A tool the server runs by calling an HTTP endpoint. Create one with
/// [`Tool::webhook`](super::Tool::webhook). Every method takes the tool by value and returns it,
/// so a tool is written as one chain.
///
/// ```
/// use elevenlabs_rust_sdk::agent_tools::{Header, Schema, Tool, ToolOptions};
///
/// let get_order = Tool::webhook("get_order", "Look up an order by the number the caller gives")
///     .get("https://api.example.com/orders/{order_id}")
///     .path_param("order_id", Schema::string().describe("The order number, e.g. A-1042"))
///     .query_param("include", Schema::string().constant("items"))
///     .header("Authorization", Header::secret("sec_orders_api"))
///     .assign("order_status", "order.status", Default::default())
///     .options(ToolOptions::new().set("response_timeout_secs", 10));
/// let request = get_order.build().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct WebhookTool {
    /// The tool config fields not kept below.
    config: WireMap,
    /// Problems found while building, reported by `build()`.
    issues: Vec<ToolIssue>,
    /// The `api_schema` fields not kept below: url, method, auth, content type and so on.
    api: WireMap,
    path_params: Vec<(String, Schema)>,
    /// An object, unless a loaded config declared another type (reported by `build()`).
    query: Option<Schema>,
    body: Option<Schema>,
    response: Option<Schema>,
    headers: Vec<(String, Value)>,
    /// Whether a loaded `path_params_schema` / `request_headers` is written back when empty, so a
    /// loaded `{}` round-trips. Removing the last entry clears it: the field is then left out.
    keep_empty_path_params: bool,
    keep_empty_headers: bool,
}

impl WebhookTool {
    pub(crate) fn new(name: String, description: String) -> Self {
        let mut config = WireMap::new();
        config.insert("name".into(), json!(name));
        config.insert("description".into(), json!(description));
        Self::load(config, WireMap::new())
    }

    pub(crate) fn load(config: WireMap, mut api: WireMap) -> Self {
        let mut tool = WebhookTool {
            config,
            issues: Vec::new(),
            api: WireMap::new(),
            path_params: Vec::new(),
            query: None,
            body: None,
            response: None,
            headers: Vec::new(),
            keep_empty_path_params: false,
            keep_empty_headers: false,
        };
        // A loaded field of an unexpected shape stays in `api` and is written back as loaded.
        if let Some(Value::Object(params)) = api.get("path_params_schema") {
            for (name, model) in params {
                let model = model.as_object().cloned().unwrap_or_default();
                let kind = kind_of(&model);
                tool.path_params
                    .push((name.clone(), Schema::from_model(&model, false, kind)));
            }
            api.remove("path_params_schema");
            tool.keep_empty_path_params = true;
        }
        if let Some(Value::Object(query)) = api.get("query_params_schema") {
            tool.query = Some(container_schema(query));
            api.remove("query_params_schema");
        }
        if let Some(Value::Object(body)) = api.get("request_body_schema") {
            tool.body = Some(container_schema(body));
            api.remove("request_body_schema");
        }
        if let Some(Value::Object(response)) = api.get("response_body_schema") {
            tool.response = Some(container_schema(response));
            api.remove("response_body_schema");
        }
        if let Some(Value::Object(headers)) = api.get("request_headers") {
            tool.headers = headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            api.remove("request_headers");
            tool.keep_empty_headers = true;
        }
        tool.api = api;
        tool
    }

    tool_methods!(
        ToolKind::Webhook,
        WebhookToolConfigInput,
        WebhookToolConfigInput
    );

    /// Sends a `GET` to `url`. `{name}` placeholders in the URL are filled from path params.
    pub fn get(self, url: impl Into<String>) -> Self {
        self.request("GET", url.into())
    }

    /// Sends a `POST` to `url`.
    pub fn post(self, url: impl Into<String>) -> Self {
        self.request("POST", url.into())
    }

    /// Sends a `PUT` to `url`.
    pub fn put(self, url: impl Into<String>) -> Self {
        self.request("PUT", url.into())
    }

    /// Sends a `PATCH` to `url`.
    pub fn patch(self, url: impl Into<String>) -> Self {
        self.request("PATCH", url.into())
    }

    /// Sends a `DELETE` to `url`.
    pub fn delete(self, url: impl Into<String>) -> Self {
        self.request("DELETE", url.into())
    }

    /// Sets the path param `name`, filling the `{name}` placeholder in the URL. Literal schemas
    /// only. A param that exists is replaced in place.
    pub fn path_param(mut self, name: impl Into<String>, schema: impl Into<Schema>) -> Self {
        let name = name.into();
        let schema = schema.into();
        match self
            .path_params
            .iter_mut()
            .find(|(existing, _)| *existing == name)
        {
            Some(slot) => slot.1 = schema,
            None => self.path_params.push((name, schema)),
        }
        self
    }

    /// Sets the query param `name`. Required unless the schema is `optional()`. Literal schemas
    /// only.
    pub fn query_param(mut self, name: impl Into<String>, schema: impl Into<Schema>) -> Self {
        self.query = match self.query.take() {
            None => Some(Schema::object().property(name, schema).into()),
            Some(Schema::Object(query)) => Some(query.property(name, schema).into()),
            // A loaded query schema that isn't an object is kept as loaded; build() reports it.
            other => other,
        };
        self
    }

    /// Sets the request body schema. Must be an object.
    pub fn body(mut self, schema: impl Into<Schema>) -> Self {
        self.body = Some(schema.into());
        self
    }

    /// Sets the response body schema. Must be an object.
    pub fn response(mut self, schema: impl Into<Schema>) -> Self {
        self.response = Some(schema.into());
        self
    }

    /// Sets the request header `name`. A header that exists is replaced in place.
    pub fn header(mut self, name: impl Into<String>, value: Header) -> Self {
        let name = name.into();
        let value = value.to_wire();
        match self
            .headers
            .iter_mut()
            .find(|(existing, _)| *existing == name)
        {
            Some(slot) => slot.1 = value,
            None => self.headers.push((name, value)),
        }
        self
    }

    /// Authenticates requests with an auth connection.
    pub fn auth(mut self, connection: AuthConnection) -> Self {
        self.api
            .insert("auth_connection".into(), connection.to_wire());
        self
    }

    /// Stores the value at `value_path` in the response (dot notation, e.g. `"order.status"`) in
    /// the dynamic variable `variable`.
    pub fn assign(
        mut self,
        variable: impl Into<String>,
        value_path: impl Into<String>,
        options: AssignOptions,
    ) -> Self {
        let mut entry = WireMap::new();
        entry.insert("source".into(), json!("response"));
        entry.insert("dynamic_variable".into(), json!(variable.into()));
        entry.insert("value_path".into(), json!(value_path.into()));
        if let Some(sanitize) = options.sanitize {
            entry.insert("sanitize".into(), json!(sanitize));
        }
        if let Some(preserve) = options.preserve_native_type {
            entry.insert("preserve_native_type".into(), json!(preserve));
        }
        let mut assignments = match self.config.remove("assignments") {
            Some(Value::Array(assignments)) => assignments,
            _ => Vec::new(),
        };
        assignments.push(Value::Object(entry));
        self.config
            .insert("assignments".into(), Value::Array(assignments));
        self
    }

    /// Removes the path param `name`. Does nothing if there is none.
    pub fn remove_path_param(mut self, name: &str) -> Self {
        let before = self.path_params.len();
        self.path_params.retain(|(existing, _)| existing != name);
        // Removing the last path param leaves `path_params_schema` out.
        if self.path_params.len() < before && self.path_params.is_empty() {
            self.keep_empty_path_params = false;
        }
        self
    }

    /// Removes the query param `name`. Does nothing if there is none.
    pub fn remove_query_param(mut self, name: &str) -> Self {
        if let Some(Schema::Object(query)) = &self.query {
            if query.property_schema(name).is_some() {
                let query = query.clone().without(name);
                // Removing the last query param leaves `query_params_schema` out.
                self.query = if query.property_names().is_empty() {
                    None
                } else {
                    Some(query.into())
                };
            }
        }
        self
    }

    /// Removes the request header `name`. Does nothing if there is none.
    pub fn remove_header(mut self, name: &str) -> Self {
        let before = self.headers.len();
        self.headers.retain(|(existing, _)| existing != name);
        // Removing the last header leaves `request_headers` out.
        if self.headers.len() < before && self.headers.is_empty() {
            self.keep_empty_headers = false;
        }
        self
    }

    /// Removes every assignment to `variable`. Does nothing if there is none.
    pub fn remove_assignment(mut self, variable: &str) -> Self {
        if let Some(Value::Array(assignments)) = self.config.get_mut("assignments") {
            let before = assignments.len();
            assignments
                .retain(|a| a.get("dynamic_variable").and_then(Value::as_str) != Some(variable));
            // Removing the last assignment leaves `assignments` out; a loaded `[]` that is never
            // edited round-trips.
            if assignments.is_empty() && before > 0 {
                self.config.remove("assignments");
            }
        }
        self
    }

    /// The request body schema, for editing and setting again with `body()`.
    pub fn body_schema(&self) -> Option<&Schema> {
        self.body.as_ref()
    }

    /// The response body schema, for editing and setting again with `response()`.
    pub fn response_schema(&self) -> Option<&Schema> {
        self.response.as_ref()
    }

    /// Merges options for webhook tools (see [`ToolOptions`]). A key that doesn't apply is
    /// reported by `build()`.
    pub fn options(mut self, options: ToolOptions) -> Self {
        let allowed: Vec<&str> = COMMON_OPTIONS
            .iter()
            .chain(WEBHOOK_ONLY_OPTIONS)
            .copied()
            .collect();
        for (key, value) in accepted_options(options, &allowed, "webhook", &mut self.issues) {
            if key == "content_type" || key == "response_filter" {
                set_value(&mut self.api, &key, value);
            } else {
                set_config_option(&mut self.config, &key, value);
            }
        }
        self
    }

    fn request(mut self, method: &str, url: String) -> Self {
        self.api.insert("method".into(), json!(method));
        self.api.insert("url".into(), json!(url));
        self
    }

    fn validate(&self) -> Vec<ToolIssue> {
        let mut issues = Vec::new();
        let error =
            |code, path: Option<&str>, message: String| ToolIssue::error(code, path, message);
        // The API reads an empty URL as unset.
        let url = self
            .api
            .get("url")
            .and_then(Value::as_str)
            .filter(|url| !url.is_empty());
        if url.is_none() {
            issues.push(error(
                ToolIssueCode::MissingUrl,
                None,
                "Set the method and URL with get(), post(), put(), patch() or delete()".into(),
            ));
        }
        // As the API does: env var templates aren't placeholders, and the auth connection fills
        // `auth_resolved_params`.
        let placeholders = placeholders(&strip_env_templates(url.unwrap_or("")));
        let auth_resolved: Vec<&str> = self
            .api
            .get("auth_resolved_params")
            .and_then(Value::as_array)
            .map(|names| names.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        for name in &placeholders {
            let has_param = self
                .path_params
                .iter()
                .any(|(existing, _)| existing == name);
            if !has_param && !auth_resolved.contains(&name.as_str()) {
                issues.push(error(
                    ToolIssueCode::MissingPathParam,
                    Some(&format!("path_params.{name}")),
                    format!("The URL has {{{name}}} but no path_param(\"{name}\")"),
                ));
            }
        }
        for (name, schema) in &self.path_params {
            let path = format!("path_params.{name}");
            if url.is_some() && !placeholders.contains(name) {
                issues.push(error(
                    ToolIssueCode::UnknownPathParam,
                    Some(&path),
                    format!("path_param(\"{name}\") has no {{{name}}} in the URL"),
                ));
            } else if auth_resolved.contains(&name.as_str()) {
                issues.push(error(
                    ToolIssueCode::UnknownPathParam,
                    Some(&path),
                    format!("{{{name}}} is filled by the auth connection, not a path_param"),
                ));
            }
            if !schema.kind().is_literal() {
                issues.push(error(
                    ToolIssueCode::ParamNotLiteral,
                    Some(&path),
                    format!("Path param \"{name}\" must be a string, integer, number or bool"),
                ));
            }
            // A path param is always required (the URL needs it), whatever `optional()` says:
            // the API schema has no `required` list for path params to leave it out of.
            if schema.state().omitted {
                issues.push(error(
                    ToolIssueCode::OmittedRequired,
                    Some(&path),
                    format!("Path param \"{name}\" can't use omit(): the URL always needs it"),
                ));
            }
            check_schema(schema.state(), &path, false, &mut issues);
        }
        if let Some(query) = &self.query {
            if query.kind() != SchemaKind::Object {
                issues.push(error(
                    ToolIssueCode::BodyNotObject,
                    Some("query"),
                    "The query params schema must be an object".into(),
                ));
            }
        }
        if let Some(query) = self.query.as_ref().and_then(Schema::as_object) {
            for name in query.property_names() {
                let schema = query.property_schema(name).expect("listed property exists");
                let path = format!("query.{name}");
                if !schema.kind().is_literal() {
                    issues.push(error(
                        ToolIssueCode::ParamNotLiteral,
                        Some(&path),
                        format!("Query param \"{name}\" must be a string, integer, number or bool"),
                    ));
                }
                check_schema(schema.state(), &path, true, &mut issues);
            }
        }
        for (label, schema) in [("body", &self.body), ("response", &self.response)] {
            let Some(schema) = schema else { continue };
            if schema.kind() != SchemaKind::Object {
                issues.push(error(
                    ToolIssueCode::BodyNotObject,
                    Some(label),
                    format!("The {label} schema must be an object"),
                ));
            }
            check_schema(schema.state(), label, false, &mut issues);
        }
        let method = self.api.get("method").and_then(Value::as_str);
        if method == Some("GET") && self.body.is_some() {
            issues.push(error(
                ToolIssueCode::BodyNotAllowed,
                Some("body"),
                "A GET request can't have a body".into(),
            ));
        }
        if let Some(method @ ("POST" | "PUT" | "PATCH")) = method {
            if self.body.is_none() {
                issues.push(error(
                    ToolIssueCode::MissingBody,
                    Some("body"),
                    format!("A {method} request needs a body"),
                ));
            }
        }
        let mut seen_in: HashMap<&str, &str> = HashMap::new();
        let path_names: Vec<&str> = self.path_params.iter().map(|(n, _)| n.as_str()).collect();
        let query_names = self
            .query
            .as_ref()
            .and_then(Schema::as_object)
            .map(ObjectSchema::property_names)
            .unwrap_or_default();
        let body_names = self
            .body
            .as_ref()
            .and_then(Schema::as_object)
            .map(ObjectSchema::property_names)
            .unwrap_or_default();
        for (location, names) in [
            ("path_params", path_names),
            ("query", query_names),
            ("body", body_names),
        ] {
            for name in names {
                match seen_in.get(name) {
                    Some(first) => issues.push(error(
                        ToolIssueCode::DuplicateParam,
                        Some(&format!("{location}.{name}")),
                        format!("\"{name}\" is in both {first} and {location}; param names must be unique across path, query and body"),
                    )),
                    None => {
                        seen_in.insert(name, location);
                    }
                }
            }
        }
        let mut seen_headers: HashMap<String, &str> = HashMap::new();
        for (name, _) in &self.headers {
            if let Some(other) = seen_headers.get(&name.to_lowercase()) {
                issues.push(error(
                    ToolIssueCode::DuplicateHeader,
                    Some(&format!("headers.{name}")),
                    format!("Headers \"{other}\" and \"{name}\" differ only in case"),
                ));
            }
            seen_headers.insert(name.to_lowercase(), name);
        }
        if let Some(timeout) = self
            .config
            .get("response_timeout_secs")
            .and_then(Value::as_f64)
        {
            if !(5.0..=300.0).contains(&timeout) {
                issues.push(error(
                    ToolIssueCode::InvalidOption,
                    Some("response_timeout_secs"),
                    "A webhook tool's response_timeout_secs must be 5–300".into(),
                ));
            }
        }
        let has_domains = self
            .config
            .get("follow_redirects_allowed_domains")
            .and_then(Value::as_array)
            .is_some_and(|domains| !domains.is_empty());
        if self.config.get("follow_redirects") == Some(&Value::Bool(true)) && !has_domains {
            issues.push(error(
                ToolIssueCode::InvalidOption,
                Some("follow_redirects"),
                "follow_redirects needs follow_redirects_allowed_domains".into(),
            ));
        }
        issues
    }

    fn model_issues(&self) -> Vec<ToolIssue> {
        let mut issues = Vec::new();
        for (name, schema) in &self.path_params {
            check_model_shape(schema.state(), &format!("path_params.{name}"), &mut issues);
        }
        for (label, schema) in [
            ("query", &self.query),
            ("body", &self.body),
            ("response", &self.response),
        ] {
            if let Some(schema) = schema {
                check_model_shape(schema.state(), label, &mut issues);
            }
        }
        issues
    }

    fn to_wire(&self) -> WireMap {
        let mut api = self.api.clone();
        if !self.path_params.is_empty() || self.keep_empty_path_params {
            let params: WireMap = self
                .path_params
                .iter()
                .map(|(name, schema)| (name.clone(), Value::Object(schema.to_model())))
                .collect();
            api.insert("path_params_schema".into(), Value::Object(params));
        }
        if let Some(query) = &self.query {
            // The API's query schema has no `type`: leave out the one the builder writes, and
            // keep every loaded field (a loaded `type` included).
            let mut model = query.to_model();
            if !query.state().type_loaded {
                model.remove("type");
            }
            api.insert("query_params_schema".into(), Value::Object(model));
        }
        if let Some(body) = &self.body {
            api.insert("request_body_schema".into(), Value::Object(body.to_model()));
        }
        if let Some(response) = &self.response {
            api.insert(
                "response_body_schema".into(),
                Value::Object(response.to_model()),
            );
        }
        if !self.headers.is_empty() || self.keep_empty_headers {
            let headers: WireMap = self.headers.iter().cloned().collect();
            api.insert("request_headers".into(), Value::Object(headers));
        }
        let mut wire = self.config.clone();
        wire.insert("type".into(), json!("webhook"));
        wire.insert("api_schema".into(), Value::Object(api));
        wire
    }
}

/// Replaces each environment variable template (`{{system__env_<label>}}`, label matching
/// `[a-z][a-z0-9_]*`) with `env`, so its braces aren't read as a path placeholder.
fn strip_env_templates(url: &str) -> String {
    const PREFIX: &[u8] = b"{{system__env_";
    let bytes = url.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(PREFIX) {
            let start = i + PREFIX.len();
            if bytes.get(start).is_some_and(u8::is_ascii_lowercase) {
                let mut end = start + 1;
                while bytes
                    .get(end)
                    .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'_')
                {
                    end += 1;
                }
                if bytes[end..].starts_with(b"}}") {
                    out.extend_from_slice(b"env");
                    i = end + 2;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    // Only whole ASCII sequences were replaced, so the bytes are still valid UTF-8.
    String::from_utf8(out).unwrap_or_default()
}

/// The `{name}` placeholders in a URL, in order: `name` is one or more characters other than `{`
/// and `}`, as the API matches them.
fn placeholders(url: &str) -> Vec<String> {
    let bytes = url.as_bytes();
    let mut names = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'{' && bytes[j] != b'}' {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'}' && j > i + 1 {
                names.push(url[i + 1..j].to_string());
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    names
}

#[cfg(test)]
mod tests {
    use super::{placeholders, strip_env_templates};

    #[test]
    fn finds_placeholders_like_the_api() {
        assert_eq!(placeholders("https://x/{a}/{b}"), ["a", "b"]);
        assert_eq!(placeholders("https://x/{}/{a{b}/{ü}"), ["b", "ü"]);
        assert!(placeholders("https://x/{open").is_empty());
    }

    #[test]
    fn env_templates_are_not_placeholders() {
        let url = strip_env_templates("https://{{system__env_api_host}}/{tenant}");
        assert_eq!(url, "https://env/{tenant}");
        assert_eq!(placeholders(&url), ["tenant"]);
        // Not a valid label: left as is, so its inner braces are a placeholder.
        let url = strip_env_templates("https://{{system__env_Host}}/x");
        assert_eq!(placeholders(&url), ["system__env_Host"]);
    }
}
