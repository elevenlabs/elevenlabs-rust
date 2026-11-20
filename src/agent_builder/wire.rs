//! Helpers for agent configs in API format (snake_case JSON).

use serde_json::{Map, Value};

use crate::agent_tools::json::{js_canonical, to_json_string_indented};
use crate::agent_tools::Tool;

pub(crate) type WireMap = Map<String, Value>;

pub(crate) const TOOL_IDS_PATH: &[&str] = &["conversation_config", "agent", "prompt", "tool_ids"];
pub(crate) const BUILT_IN_TOOLS_PATH: &[&str] =
    &["conversation_config", "agent", "prompt", "built_in_tools"];

/// The value at `path`, or `None` if any step is missing.
pub(crate) fn get_path<'a, S: AsRef<str>>(target: &'a Value, path: &[S]) -> Option<&'a Value> {
    let mut current = target;
    for key in path {
        current = current.as_object()?.get(key.as_ref())?;
    }
    Some(current)
}

/// [`get_path`] on an object map.
pub(crate) fn get_path_in<'a, S: AsRef<str>>(target: &'a WireMap, path: &[S]) -> Option<&'a Value> {
    let (first, rest) = path.split_first()?;
    get_path(target.get(first.as_ref())?, rest)
}

/// Sets the value at `path`, creating objects along the way (replacing non-objects). `None`
/// deletes the key, and creates nothing.
pub(crate) fn set_path<S: AsRef<str>>(target: &mut WireMap, path: &[S], value: Option<Value>) {
    let Some((last, parents)) = path.split_last() else {
        return;
    };
    let mut current = target;
    for key in parents {
        let key = key.as_ref();
        if !current.get(key).is_some_and(Value::is_object) {
            if value.is_none() {
                return;
            }
            current.insert(key.to_string(), Value::Object(Map::new()));
        }
        current = current
            .get_mut(key)
            .and_then(Value::as_object_mut)
            .expect("just made an object");
    }
    match value {
        Some(value) => {
            current.insert(last.as_ref().to_string(), value);
        }
        None => {
            current.remove(last.as_ref());
        }
    }
}

/// Merges `source` into `target`: objects merge key by key, anything else (arrays included)
/// replaces.
pub(crate) fn deep_merge(target: &mut WireMap, source: &WireMap) {
    for (key, value) in source {
        match (target.get_mut(key), value) {
            (Some(Value::Object(existing)), Value::Object(incoming)) => {
                deep_merge(existing, incoming);
            }
            _ => {
                target.insert(key.clone(), value.clone());
            }
        }
    }
}

/// Whether every value in `local` is in `remote` at the same place. Extra keys in `remote`
/// (server defaults) are fine, and a `null` in `local` matches anything.
pub(crate) fn is_subset(local: &Value, remote: &Value) -> bool {
    match (local, remote) {
        (Value::Array(l), Value::Array(r)) => {
            l.len() == r.len() && l.iter().zip(r).all(|(l, r)| is_subset(l, r))
        }
        (Value::Array(_), _) => false,
        (Value::Object(l), Value::Object(r)) => l
            .iter()
            .all(|(k, v)| v.is_null() || r.get(k).is_some_and(|rv| is_subset(v, rv))),
        (Value::Object(_), _) => false,
        (Value::Number(l), Value::Number(r)) => l == r || l.as_f64() == r.as_f64(),
        _ => local == remote,
    }
}

/// The tool ids an API-format workflow's nodes use: tool nodes' `tools` and subagents'
/// `additional_tool_ids`, in node order.
pub(crate) fn raw_workflow_tool_ids(workflow: Option<&Value>) -> Vec<String> {
    let Some(nodes) = workflow
        .and_then(|w| w.get("nodes"))
        .and_then(Value::as_object)
    else {
        return Vec::new();
    };
    let mut ids = Vec::new();
    for node in nodes.values().filter_map(Value::as_object) {
        for tool in node
            .get("tools")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(id) = tool.get("tool_id").and_then(Value::as_str) {
                ids.push(id.to_string());
            }
        }
        for id in node
            .get("additional_tool_ids")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(id) = id.as_str() {
                ids.push(id.to_string());
            }
        }
    }
    ids
}

/// Replaces the tool ids in an API-format workflow's tool nodes and subagents, in place.
pub(crate) fn map_raw_workflow_tool_ids(workflow: &mut Value, map: &dyn Fn(&str) -> String) {
    let Some(nodes) = workflow.get_mut("nodes").and_then(Value::as_object_mut) else {
        return;
    };
    for node in nodes.values_mut().filter_map(Value::as_object_mut) {
        if let Some(Value::Array(tools)) = node.get_mut("tools") {
            for tool in tools {
                if let Some(Value::String(id)) = tool.get_mut("tool_id") {
                    *id = map(id);
                }
            }
        }
        if let Some(Value::Array(ids)) = node.get_mut("additional_tool_ids") {
            for id in ids {
                if let Value::String(value) = id {
                    *value = map(value);
                }
            }
        }
    }
}

/// Every tool id an agent config uses, in the agent's `tool_ids` and the workflow's nodes, once
/// each, in order.
pub(crate) fn tool_ids_in(config: &Value) -> Vec<String> {
    let mut ids: Vec<String> = get_path(config, TOOL_IDS_PATH)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|id| id.as_str().map(str::to_string))
        .collect();
    ids.extend(raw_workflow_tool_ids(config.get("workflow")));
    let mut seen = std::collections::HashSet::new();
    ids.retain(|id| seen.insert(id.clone()));
    ids
}

/// A tool's definition as this SDK models it, as a string to compare: its config passed through
/// the generated request model, which leaves out the fields the model doesn't declare. The tool
/// builder keeps such fields from a loaded config, but a tool written in code can't have them, so
/// they don't count when telling whether two definitions are the same tool. `None` if the tool
/// doesn't build.
pub(crate) fn tool_definition(tool: &Tool) -> Option<String> {
    let config = tool.build_config().ok()?;
    let wire = serde_json::to_value(config).ok()?;
    Some(js_canonical(&wire).to_string())
}

/// The CLI's file format: 4-space indent, no trailing newline.
pub(crate) fn format_index(value: &Value) -> String {
    to_json_string_indented(value, b"    ")
}

/// Config files: like [`format_index`], with keys sorted at every level, as the CLI writes them.
pub(crate) fn format_config(value: &Value) -> String {
    to_json_string_indented(&js_canonical(value), b"    ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn set_path_creates_and_deletes() {
        let mut map = WireMap::new();
        set_path(&mut map, &["a", "b"], Some(json!(1)));
        assert_eq!(Value::Object(map.clone()), json!({ "a": { "b": 1 } }));
        set_path(&mut map, &["a", "b"], None);
        assert_eq!(Value::Object(map.clone()), json!({ "a": {} }));
        set_path(&mut map, &["x", "y"], None);
        assert_eq!(Value::Object(map), json!({ "a": {} }));
    }

    #[test]
    fn deep_merge_merges_objects_and_replaces_the_rest() {
        let mut target = json!({ "a": { "b": 1, "c": [1, 2] }, "d": 1 })
            .as_object()
            .cloned()
            .unwrap();
        let source = json!({ "a": { "c": [3], "e": true }, "d": { "f": 1 } });
        deep_merge(&mut target, source.as_object().unwrap());
        assert_eq!(
            Value::Object(target),
            json!({ "a": { "b": 1, "c": [3], "e": true }, "d": { "f": 1 } })
        );
    }

    #[test]
    fn is_subset_allows_server_extras_only() {
        assert!(is_subset(&json!({ "a": 1 }), &json!({ "a": 1, "b": 2 })));
        assert!(is_subset(&json!({ "a": null }), &json!({})));
        assert!(!is_subset(&json!({ "a": [1] }), &json!({ "a": [1, 2] })));
        assert!(is_subset(&json!(2), &json!(2.0)));
        assert!(!is_subset(&json!({ "a": 1 }), &json!({ "a": 2 })));
    }
}
