//! Mutable graph state shared by a `Workflow` and every handle created from it. Nodes and edges are
//! kept in wire format (`serde_json::Value`).

use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::agent_tools::Tool;

use super::destination::Destination;
use super::errors::{WorkflowIssue, WorkflowIssueCode};
use super::wire::{as_object, WireMap};

/// A tool a workflow node uses: an existing tool's id, or a [`Tool`] object whose id is filled in
/// when the workflow is built with tool ids (as `agents_as_code().push()` does). Converts from a
/// `&str`/`String` id and from any tool type, so `vec!["tool_1".into(), get_order.into()]` mixes
/// them.
// A `Tool` is much larger than an id, but refs only live while a node is built.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum ToolRef {
    /// An existing tool, by id.
    Id(String),
    /// A tool defined in code. It stands in the graph as `<tool:NAME>` until the workflow is built.
    Tool(Tool),
}

impl From<&str> for ToolRef {
    fn from(id: &str) -> Self {
        ToolRef::Id(id.to_string())
    }
}

impl From<String> for ToolRef {
    fn from(id: String) -> Self {
        ToolRef::Id(id)
    }
}

impl From<&String> for ToolRef {
    fn from(id: &String) -> Self {
        ToolRef::Id(id.clone())
    }
}

impl From<Tool> for ToolRef {
    fn from(tool: Tool) -> Self {
        ToolRef::Tool(tool)
    }
}

impl From<&Tool> for ToolRef {
    fn from(tool: &Tool) -> Self {
        ToolRef::Tool(tool.clone())
    }
}

impl From<crate::agent_tools::WebhookTool> for ToolRef {
    fn from(tool: crate::agent_tools::WebhookTool) -> Self {
        ToolRef::Tool(tool.into())
    }
}

impl From<crate::agent_tools::ClientTool> for ToolRef {
    fn from(tool: crate::agent_tools::ClientTool) -> Self {
        ToolRef::Tool(tool.into())
    }
}

impl From<crate::agent_tools::SystemTool> for ToolRef {
    fn from(tool: crate::agent_tools::SystemTool) -> Self {
        ToolRef::Tool(tool.into())
    }
}

/// The placeholder a [`Tool`] object stands under in the graph until the workflow is built.
pub(crate) fn tool_placeholder(name: &str) -> String {
    format!("<tool:{name}>")
}

/// A tool id as shown in messages: the name for a `Tool` object's placeholder.
pub(crate) fn display_tool(id: &str) -> &str {
    id.strip_prefix("<tool:")
        .and_then(|rest| rest.strip_suffix('>'))
        .unwrap_or(id)
}

/// The tool ids (or placeholders) a node uses: a tool node's tools, or a subagent's extra tools.
pub(crate) fn node_tool_ids(model: &WireMap) -> Vec<String> {
    let mut ids: Vec<String> = model
        .get("tools")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|t| t.get("tool_id").and_then(Value::as_str).map(str::to_string))
        .collect();
    ids.extend(
        model
            .get("additional_tool_ids")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|id| id.as_str().map(str::to_string)),
    );
    ids
}

/// Replaces the tool ids in a node's tools (tool nodes) or extra tools (subagents), in place.
pub(crate) fn map_node_tool_ids(model: &mut WireMap, map: &dyn Fn(&str) -> String) {
    match model.get("type").and_then(Value::as_str) {
        Some("tool") => {
            if let Some(Value::Array(tools)) = model.get_mut("tools") {
                for tool in tools {
                    if let Some(Value::String(id)) = tool.get_mut("tool_id") {
                        *id = map(id);
                    }
                }
            }
        }
        Some("override_agent") => {
            if let Some(Value::Array(ids)) = model.get_mut("additional_tool_ids") {
                for id in ids {
                    if let Value::String(value) = id {
                        *value = map(value);
                    }
                }
            }
        }
        _ => {}
    }
}

/// An edge in the graph. Compared by identity (`uid`): two edges with the same fields are still two
/// edges, as in the Python/TS references.
#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub uid: u64,
    pub source: String,
    pub target: String,
    pub forward_condition: Option<Value>,
    pub backward_condition: Option<Value>,
    /// Set for edges loaded from an existing workflow; generated at build time otherwise.
    pub id: Option<String>,
    /// Other fields of a loaded edge, such as `reroutes`, written back as they were.
    pub extra: WireMap,
}

const TRANSFER_TYPES: [&str; 2] = ["phone_number", "standalone_agent"];

fn node_kind(node_type: Option<&str>) -> String {
    match node_type {
        Some("start") => "start".into(),
        Some("end") => "end".into(),
        Some("override_agent") => "subagent".into(),
        Some("tool") => "tool".into(),
        Some("phone_number") => "phone/SIP transfer".into(),
        Some("standalone_agent") => "agent transfer".into(),
        Some(other) => format!("\"{other}\""),
        None => "\"None\"".into(),
    }
}

/// The API field name a canonical option maps to for a node type, or `None` if it doesn't apply.
fn api_field(node_type: Option<&str>, key: &str) -> Option<&'static str> {
    match node_type {
        Some("start") | Some("end") => match key {
            "position" => Some("position"),
            _ => None,
        },
        Some("override_agent") => match key {
            "position" => Some("position"),
            "prompt" => Some("additional_prompt"),
            "label" => Some("label"),
            "tool_ids" => Some("additional_tool_ids"),
            "knowledge_base" => Some("additional_knowledge_base"),
            "conversation_config" => Some("conversation_config"),
            "entry_behavior" => Some("entry_behavior"),
            _ => None,
        },
        Some("tool") => match key {
            "position" => Some("position"),
            _ => None,
        },
        Some("phone_number") => match key {
            "position" => Some("position"),
            "transfer_type" => Some("transfer_type"),
            "custom_sip_headers" => Some("custom_sip_headers"),
            "sip_refer_play_dialtone" => Some("sip_refer_play_dialtone"),
            "uui" => Some("uui"),
            "post_dial_digits" => Some("post_dial_digits"),
            _ => None,
        },
        Some("standalone_agent") => match key {
            "position" => Some("position"),
            "delay_ms" => Some("delay_ms"),
            "transfer_message" => Some("transfer_message"),
            "enable_transferred_agent_first_message" => {
                Some("enable_transferred_agent_first_message")
            }
            "preserve_client_tts_overrides" => Some("preserve_client_tts_overrides"),
            "preserve_voice_settings" => Some("preserve_voice_settings"),
            _ => None,
        },
        // Node kinds without a dedicated field map (e.g. generated `say`/`update_state`): position is
        // common to every node kind, so keep it rather than rejecting it as an invalid option.
        _ => match key {
            "position" => Some("position"),
            _ => None,
        },
    }
}

/// Options applied to a node, mirroring the Python `apply_options` parameter set. The special
/// options (`destination`, `tool_ids`, `schema_overrides`) are handled separately from the generic
/// `rest`, which carries canonical snake_case keys to already-wire values.
#[derive(Default)]
pub struct AppliedOptions {
    pub destination: Option<Destination>,
    pub tool_ids: Option<Vec<ToolRef>>,
    pub schema_overrides: Option<Value>,
    pub rest: Vec<(String, Value)>,
}

#[derive(Clone)]
pub struct WorkflowGraph {
    pub nodes: HashMap<String, WireMap>,
    pub node_order: Vec<String>,
    pub edges: Vec<GraphEdge>,
    pub issues: Vec<WorkflowIssue>,
    pub start_id: String,
    /// Each loaded node's edge_order, as edge uids.
    pub loaded_order: HashMap<String, Vec<u64>>,
    pub prevent_subagent_loops: Option<bool>,
    /// `Tool` objects referenced by nodes, by placeholder, in the order they were first used.
    pub tool_refs: Vec<(String, Tool)>,
    uid_counter: u64,
}

impl WorkflowGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            node_order: Vec::new(),
            edges: Vec::new(),
            issues: Vec::new(),
            start_id: "start_node".to_string(),
            loaded_order: HashMap::new(),
            prevent_subagent_loops: None,
            tool_refs: Vec::new(),
            uid_counter: 0,
        }
    }

    /// The `Tool` object standing under `placeholder`, if any.
    pub fn tool_ref(&self, placeholder: &str) -> Option<&Tool> {
        self.tool_refs
            .iter()
            .find(|(key, _)| key == placeholder)
            .map(|(_, tool)| tool)
    }

    /// Remembers `tool` under `placeholder`, replacing (in place) a tool already there.
    pub fn set_tool_ref(&mut self, placeholder: String, tool: Tool) {
        match self
            .tool_refs
            .iter_mut()
            .find(|(key, _)| *key == placeholder)
        {
            Some(slot) => slot.1 = tool,
            None => self.tool_refs.push((placeholder, tool)),
        }
    }

    /// Swaps `Tool` objects for placeholders, remembering each one. A different tool of the same
    /// name is a `duplicate_tool_name` if this node or another node still uses that name; one left
    /// behind by a removed or edited node doesn't count. Tools are values here, so "different"
    /// means another definition: the same tool used twice is fine.
    pub fn ref_tools(&mut self, node_id: &str, refs: Vec<ToolRef>) -> Vec<String> {
        let mut in_this_node: Vec<(String, Tool)> = Vec::new();
        let mut ids = Vec::with_capacity(refs.len());
        for r in refs {
            let tool = match r {
                ToolRef::Id(id) => {
                    ids.push(id);
                    continue;
                }
                ToolRef::Tool(tool) => tool,
            };
            let key = tool_placeholder(tool.name());
            let mut clash = in_this_node
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, t)| t.clone());
            if clash.is_none() && self.uses_placeholder(&key, node_id) {
                clash = self.tool_ref(&key).cloned();
            }
            if clash.as_ref().is_some_and(|other| !other.same_as(&tool)) {
                self.issue(
                    WorkflowIssueCode::DuplicateToolName,
                    format!(
                        "Node \"{node_id}\" uses a different tool also named \"{}\"",
                        tool.name()
                    ),
                    Some(node_id),
                );
            }
            match in_this_node.iter_mut().find(|(k, _)| *k == key) {
                Some(slot) => slot.1 = tool.clone(),
                None => in_this_node.push((key.clone(), tool.clone())),
            }
            self.set_tool_ref(key.clone(), tool);
            ids.push(key);
        }
        ids
    }

    fn uses_placeholder(&self, key: &str, except_node_id: &str) -> bool {
        self.nodes
            .iter()
            .filter(|(id, _)| id.as_str() != except_node_id)
            .any(|(_, model)| node_tool_ids(model).iter().any(|id| id == key))
    }

    pub fn next_uid(&mut self) -> u64 {
        let uid = self.uid_counter;
        self.uid_counter += 1;
        uid
    }

    pub fn issue(
        &mut self,
        code: WorkflowIssueCode,
        message: impl Into<String>,
        node_id: Option<&str>,
    ) {
        self.issues.push(WorkflowIssue::error(
            code,
            message,
            node_id.map(str::to_string),
        ));
    }

    pub fn node_type(&self, id: &str) -> Option<String> {
        self.nodes
            .get(id)
            .and_then(|m| m.get("type"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
    }

    pub fn add_node(&mut self, id: &str, model: WireMap) {
        match self.nodes.get(id) {
            None => {
                self.nodes.insert(id.to_string(), model);
                self.node_order.push(id.to_string());
            }
            Some(existing) => {
                let existing_end = existing.get("type").and_then(|v| v.as_str()) == Some("end");
                let new_end = model.get("type").and_then(|v| v.as_str()) == Some("end");
                if !(existing_end && new_end) {
                    self.issue(
                        WorkflowIssueCode::DuplicateNodeId,
                        format!("Node id \"{id}\" is used by more than one node"),
                        Some(id),
                    );
                }
            }
        }
    }

    fn invalid_option(&mut self, id: &str, option: &str, node_type: Option<&str>) {
        let kind = node_kind(node_type);
        self.issue(
            WorkflowIssueCode::InvalidOption,
            format!("Option \"{option}\" doesn't apply to {kind} node \"{id}\""),
            Some(id),
        );
    }

    /// Apply canonical options to a node model, recording `invalid_option` for keys that don't
    /// apply. Returns the updated model.
    pub fn apply_options(&mut self, id: &str, model: WireMap, opts: AppliedOptions) -> WireMap {
        let AppliedOptions {
            destination,
            tool_ids,
            schema_overrides,
            mut rest,
        } = opts;
        let tool_ids = tool_ids.map(|refs| self.ref_tools(id, refs));
        // Overrides for a `Tool` object may be keyed by its name: key them by its placeholder.
        let schema_overrides = schema_overrides.map(|overrides| match overrides {
            Value::Object(map) => Value::Object(
                map.into_iter()
                    .map(|(key, value)| {
                        let placeholder = tool_placeholder(&key);
                        if self.tool_ref(&placeholder).is_some() {
                            (placeholder, value)
                        } else {
                            (key, value)
                        }
                    })
                    .collect(),
            ),
            other => other,
        });
        let mut node = model;

        if let Some(dest) = destination {
            let node_type = node.get("type").and_then(|v| v.as_str()).map(str::to_string);
            if node_type.as_deref().is_some_and(|t| TRANSFER_TYPES.contains(&t)) {
                let dest_wire = as_object(&dest.to_wire());
                let dest_type = dest_wire.get("type").and_then(|v| v.as_str());
                if dest_type != node_type.as_deref() {
                    let position = node.get("position").cloned();
                    node = dest_wire;
                    if let Some(pos) = position {
                        node.insert("position".to_string(), pos);
                    }
                } else {
                    if node_type.as_deref() == Some("standalone_agent") {
                        for key in ["agent_id", "node_id", "branch_id"] {
                            node.remove(key);
                        }
                    }
                    for (k, v) in dest_wire {
                        node.insert(k, v);
                    }
                }
            } else {
                self.invalid_option(id, "destination", node_type.as_deref());
            }
        }

        let node_type = node.get("type").and_then(|v| v.as_str()).map(str::to_string);
        if node_type.as_deref() == Some("tool") {
            let existing: Vec<Value> = node
                .get("tools")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let ids: Vec<String> = match &tool_ids {
                Some(t) => t.clone(),
                None => existing
                    .iter()
                    .filter_map(|t| {
                        t.get("tool_id").and_then(|v| v.as_str()).map(str::to_string)
                    })
                    .collect(),
            };
            if let Some(overrides) = schema_overrides.as_ref().and_then(|v| v.as_object()) {
                for tool_id in overrides.keys() {
                    if !ids.contains(tool_id) {
                        self.issue(
                            WorkflowIssueCode::InvalidOption,
                            format!(
                                "Node \"{id}\" has schema overrides for tool \"{}\", which it doesn't run",
                                display_tool(tool_id)
                            ),
                            Some(id),
                        );
                    }
                }
            }
            let mut tools = Vec::new();
            for tool_id in &ids {
                let overrides = schema_overrides
                    .as_ref()
                    .and_then(|so| so.get(tool_id).cloned())
                    .or_else(|| {
                        existing
                            .iter()
                            .find(|t| {
                                t.get("tool_id").and_then(|v| v.as_str()) == Some(tool_id.as_str())
                            })
                            .and_then(|t| t.get("schema_overrides").cloned())
                    });
                let mut tool = Map::new();
                tool.insert("tool_id".to_string(), json!(tool_id));
                if let Some(ov) = overrides {
                    tool.insert("schema_overrides".to_string(), ov);
                }
                tools.push(Value::Object(tool));
            }
            node.insert("tools".to_string(), Value::Array(tools));
        } else {
            if schema_overrides.is_some() {
                self.invalid_option(id, "schema_overrides", node_type.as_deref());
            }
            if let Some(t) = &tool_ids {
                rest.push(("tool_ids".to_string(), json!(t)));
            }
        }

        for (key, value) in rest {
            match api_field(node_type.as_deref(), &key) {
                None => self.invalid_option(id, &key, node_type.as_deref()),
                Some(field) => {
                    node.insert(field.to_string(), value);
                }
            }
        }
        node
    }

    /// Replace the graph's contents with an existing wire-format workflow, keeping ids, positions
    /// and edge order (in document order).
    pub fn load(&mut self, workflow: &Value) {
        self.nodes.clear();
        self.node_order.clear();
        self.edges.clear();
        self.issues.clear();
        self.loaded_order.clear();
        self.tool_refs.clear();
        self.prevent_subagent_loops = workflow
            .get("prevent_subagent_loops")
            .and_then(|v| v.as_bool());

        let nodes = workflow.get("nodes").and_then(|v| v.as_object());
        let edges_map = workflow.get("edges").and_then(|v| v.as_object());

        if let Some(nodes) = nodes {
            for (id, node) in nodes {
                let model: WireMap = as_object(node)
                    .into_iter()
                    .filter(|(k, _)| k != "edge_order")
                    .collect();
                self.nodes.insert(id.clone(), model);
                self.node_order.push(id.clone());
            }
        }
        // start_id is the first node whose type is "start", in document order.
        self.start_id = nodes
            .and_then(|nodes| {
                nodes
                    .iter()
                    .find(|(_, n)| n.get("type").and_then(|v| v.as_str()) == Some("start"))
                    .map(|(id, _)| id.clone())
            })
            .unwrap_or_else(|| "start_node".to_string());
        if !self.nodes.contains_key(&self.start_id) {
            let mut m = Map::new();
            m.insert("type".to_string(), json!("start"));
            self.nodes.insert(self.start_id.clone(), m);
            self.node_order.push(self.start_id.clone());
        }

        // Rebuild the edge list in each node's edge_order (so build() reproduces it), then any
        // edges not mentioned, in document order. `placed` maps edge id -> uid.
        let mut placed: HashMap<String, u64> = HashMap::new();

        if let (Some(nodes), Some(edges_map)) = (nodes, edges_map) {
            for (_id, node) in nodes {
                if let Some(order) = node.get("edge_order").and_then(|v| v.as_array()) {
                    for edge_id in order.iter().filter_map(|v| v.as_str()) {
                        self.place_edge(edge_id, edges_map, &mut placed);
                    }
                }
            }
            for (id, node) in nodes {
                let order: Vec<u64> = node
                    .get("edge_order")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .filter_map(|edge_id| placed.get(edge_id).copied())
                            .collect()
                    })
                    .unwrap_or_default();
                self.loaded_order.insert(id.clone(), order);
            }
            for edge_id in edges_map.keys() {
                self.place_edge(edge_id, edges_map, &mut placed);
            }
        }
    }

    fn place_edge(
        &mut self,
        edge_id: &str,
        edges_map: &Map<String, Value>,
        placed: &mut HashMap<String, u64>,
    ) {
        if placed.contains_key(edge_id) {
            return;
        }
        let Some(edge) = edges_map.get(edge_id) else {
            return;
        };
        let mut rest = as_object(edge);
        let source = rest
            .remove("source")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default();
        let target = rest
            .remove("target")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default();
        // A nullable wire field present as JSON `null` means "no condition", the same as being absent.
        // Keep it as `None` so a loaded `"backward_condition": null` doesn't look like a reverse
        // transition (which would wrongly trip edge_into_start and the reverse-edge merge).
        let forward_condition = rest.remove("forward_condition").filter(|v| !v.is_null());
        let backward_condition = rest.remove("backward_condition").filter(|v| !v.is_null());
        let uid = self.next_uid();
        placed.insert(edge_id.to_string(), uid);
        self.edges.push(GraphEdge {
            uid,
            source,
            target,
            forward_condition,
            backward_condition,
            id: Some(edge_id.to_string()),
            extra: rest,
        });
    }
}
