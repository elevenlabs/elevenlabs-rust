//! Mutable graph state shared by a `Workflow` and every handle created from it. Nodes and edges are
//! kept in wire format (`serde_json::Value`).

use std::collections::HashMap;

use serde_json::{json, Map, Value};

use super::destination::Destination;
use super::errors::{WorkflowIssue, WorkflowIssueCode};
use super::wire::{as_object, WireMap};

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
        _ => None,
    }
}

/// Options applied to a node, mirroring the Python `apply_options` parameter set. The special
/// options (`destination`, `tool_ids`, `schema_overrides`) are handled separately from the generic
/// `rest`, which carries canonical snake_case keys to already-wire values.
#[derive(Default)]
pub struct AppliedOptions {
    pub destination: Option<Destination>,
    pub tool_ids: Option<Vec<String>>,
    pub schema_overrides: Option<Value>,
    pub rest: Vec<(String, Value)>,
}

pub struct WorkflowGraph {
    pub nodes: HashMap<String, WireMap>,
    pub node_order: Vec<String>,
    pub edges: Vec<GraphEdge>,
    pub issues: Vec<WorkflowIssue>,
    pub start_id: String,
    /// Each loaded node's edge_order, as edge uids.
    pub loaded_order: HashMap<String, Vec<u64>>,
    pub prevent_subagent_loops: Option<bool>,
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
            uid_counter: 0,
        }
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
                                "Node \"{id}\" has schema overrides for tool \"{tool_id}\", which it doesn't run"
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
        let forward_condition = rest.remove("forward_condition");
        let backward_condition = rest.remove("backward_condition");
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
