//! The `Workflow` itself: the start node, the editing verbs, and the deterministic wire output.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use serde_json::{json, Map, Value};

use crate::agent_tools::Tool;
use crate::api::types::AgentWorkflowRequestModel;

use super::condition::{Condition, ConditionInput};
use super::errors::{Severity, WorkflowIssue, WorkflowIssueCode, WorkflowValidationError};
use super::graph::{map_node_tool_ids, node_tool_ids, tool_placeholder, GraphEdge, WorkflowGraph};
use super::layout::layout;
use super::node::{branch, Chain, EdgeOrigin, WorkflowBranch, WorkflowNode};
use super::validate::{reverse_edge_merges, validate};
use super::wire::{canonical, to_json_string as wire_to_json_string};

/// An agent workflow, written as code. The workflow itself is the start node: calling a verb on it
/// branches from the start.
pub struct Workflow {
    graph: Rc<RefCell<WorkflowGraph>>,
}

impl Workflow {
    /// An empty workflow. `prevent_subagent_loops` stops the conversation from re-entering a
    /// subagent it already visited. The start node is created with id `"start_node"`.
    pub fn new(prevent_subagent_loops: Option<bool>) -> Self {
        let mut graph = WorkflowGraph::new();
        let start_id = graph.start_id.clone();
        let mut start = Map::new();
        start.insert("type".to_string(), json!("start"));
        graph.add_node(&start_id, start);
        graph.prevent_subagent_loops = prevent_subagent_loops;
        Self {
            graph: Rc::new(RefCell::new(graph)),
        }
    }

    /// Load a workflow from wire-format JSON (a parsed value), either a workflow or a whole agent
    /// config with a `workflow` key.
    pub fn from_json(data: &Value) -> Self {
        let raw = data
            .as_object()
            .and_then(|o| o.get("workflow"))
            .unwrap_or(data);
        let wf = Workflow::new(None);
        wf.graph.borrow_mut().load(raw);
        wf
    }

    /// Load a workflow from a wire-format JSON string.
    pub fn from_json_str(data: &str) -> Result<Self, serde_json::Error> {
        let value: Value = serde_json::from_str(data)?;
        Ok(Workflow::from_json(&value))
    }

    /// Like [`from_json`](Self::from_json), with tool ids found in `tools_by_id` swapped for those
    /// [`Tool`] objects, so the loaded workflow refers to them as a built one would. Other ids stay
    /// as they are.
    pub fn from_json_with_tools(data: &Value, tools_by_id: &HashMap<String, Tool>) -> Self {
        let wf = Workflow::from_json(data);
        {
            let mut g = wf.graph.borrow_mut();
            let mut bound: Vec<(String, Tool)> = Vec::new();
            let swap = |id: &str| -> String {
                match tools_by_id.get(id) {
                    Some(tool) => tool_placeholder(tool.name()),
                    None => id.to_string(),
                }
            };
            for id in g.node_order.clone() {
                if let Some(model) = g.nodes.get_mut(&id) {
                    for tool_id in node_tool_ids(model) {
                        if let Some(tool) = tools_by_id.get(&tool_id) {
                            bound.push((tool_placeholder(tool.name()), tool.clone()));
                        }
                    }
                    map_node_tool_ids(model, &swap);
                }
            }
            for (key, tool) in bound {
                g.set_tool_ref(key, tool);
            }
        }
        wf
    }

    /// The [`Tool`] objects the workflow's nodes use, in the order they were first used.
    pub fn tools(&self) -> Vec<Tool> {
        let g = self.graph.borrow();
        let used: HashSet<String> = g.nodes.values().flat_map(node_tool_ids).collect();
        g.tool_refs
            .iter()
            .filter(|(key, _)| used.contains(key))
            .map(|(_, tool)| tool.clone())
            .collect()
    }

    /// A copy of this workflow with its own graph: editing one doesn't change the other.
    pub(crate) fn deep_clone(&self) -> Self {
        Self {
            graph: Rc::new(RefCell::new(self.graph.borrow().clone())),
        }
    }

    /// The id of the start node (`"start_node"` unless a loaded workflow uses another).
    pub fn start_id(&self) -> String {
        self.graph.borrow().start_id.clone()
    }

    /// Ids of every node, in the order they were added.
    pub fn node_ids(&self) -> Vec<String> {
        self.graph.borrow().node_order.clone()
    }

    /// Open a branch from the start node, followed when `condition` holds.
    pub fn when(&self, condition: ConditionInput, back: Option<ConditionInput>) -> WorkflowBranch {
        let source = self.graph.borrow().start_id.clone();
        branch(&self.graph, &source, condition, back)
    }

    /// A handle to an existing node, for editing it or going to it.
    pub fn node(&self, id: &str) -> WorkflowNode {
        if !self.graph.borrow().nodes.contains_key(id) {
            self.graph.borrow_mut().issue(
                super::errors::WorkflowIssueCode::UnknownNode,
                format!("There is no node \"{id}\""),
                Some(id),
            );
        }
        WorkflowNode::new(Rc::clone(&self.graph), id.to_string(), None)
    }

    /// Remove a node and every edge to or from it.
    pub fn remove(&self, id: &str) -> &Self {
        let mut g = self.graph.borrow_mut();
        if id == g.start_id {
            g.issue(
                super::errors::WorkflowIssueCode::CannotRemoveStart,
                "The start node can't be removed",
                Some(id),
            );
            return self;
        }
        if g.nodes.remove(id).is_some() {
            g.node_order.retain(|n| n != id);
        }
        g.edges.retain(|e| e.source != id && e.target != id);
        drop(g);
        self
    }

    /// Splice new nodes into the edge from `source_id` to `target_id`. `insert` receives a branch
    /// carrying that edge's conditions and returns the last new node, which is connected to
    /// `target_id`. Returns that node, or `None` (reported by `build()`).
    pub fn insert_between<F>(
        &self,
        source_id: &str,
        target_id: &str,
        insert: F,
    ) -> Option<WorkflowNode>
    where
        F: FnOnce(WorkflowBranch) -> Option<WorkflowNode>,
    {
        let index = self
            .graph
            .borrow()
            .edges
            .iter()
            .position(|e| e.source == source_id && e.target == target_id);
        let index = match index {
            Some(i) => i,
            None => {
                self.graph.borrow_mut().issue(
                    super::errors::WorkflowIssueCode::UnknownEdge,
                    format!("There is no edge from \"{source_id}\" to \"{target_id}\" to insert into"),
                    Some(source_id),
                );
                return None;
            }
        };

        // Splitting a bidirectional edge can't preserve the original target's reverse route, so reject
        // it rather than silently drop or misattach that route. Remove the backward condition first.
        if self.graph.borrow().edges[index].backward_condition.is_some() {
            self.graph.borrow_mut().issue(
                super::errors::WorkflowIssueCode::CannotInsertBidirectional,
                format!(
                    "Edge \"{source_id}\" → \"{target_id}\" has a backward condition; inserting into a bidirectional edge would drop its reverse route. Remove the backward condition first"
                ),
                Some(source_id),
            );
            return None;
        }

        let (original_uid, origin) = {
            let mut g = self.graph.borrow_mut();
            let original = g.edges.remove(index);
            let origin = EdgeOrigin {
                source: source_id.to_string(),
                forward_condition: original
                    .forward_condition
                    .clone()
                    .unwrap_or_else(|| Condition::always(None).to_wire()),
                backward_condition: original.backward_condition.clone(),
            };
            (original.uid, origin)
        };
        let before = self.graph.borrow().edges.len();
        let last = insert(WorkflowBranch::from_origin(Rc::clone(&self.graph), origin));

        // Put the first new edge back where the original was, so the source's edge order is kept.
        let first_hop = {
            let g = self.graph.borrow();
            (before..g.edges.len()).find(|&i| g.edges[i].source == source_id)
        };
        if let Some(fh) = first_hop {
            let mut g = self.graph.borrow_mut();
            let edge = g.edges.remove(fh);
            let new_uid = edge.uid;
            g.edges.insert(index, edge);
            if let Some(loaded) = g.loaded_order.get_mut(source_id) {
                for slot in loaded.iter_mut() {
                    if *slot == original_uid {
                        *slot = new_uid;
                        break;
                    }
                }
            }
        }

        let last = match last {
            Some(node) => node,
            None => {
                self.graph.borrow_mut().issue(
                    super::errors::WorkflowIssueCode::MissingInsertResult,
                    format!(
                        "insert_between(\"{source_id}\", \"{target_id}\") must return the last inserted node"
                    ),
                    Some(source_id),
                );
                return None;
            }
        };
        if last.node_type().as_deref() == Some("tool") {
            last.on_success(None).go_to(target_id, None);
        } else {
            last.go_to(target_id, None);
        }
        Some(last)
    }

    /// Validate and return the model `agents.create()` / `agents.update()` accept. A node that
    /// uses a [`Tool`] object is reported as `unresolved_tool`: use
    /// [`build_with_tool_ids`](Self::build_with_tool_ids) for those.
    pub fn build(&self) -> Result<AgentWorkflowRequestModel, WorkflowValidationError> {
        let wire = self.to_wire::<fn(&WorkflowIssue)>(None, &HashMap::new())?;
        to_request_model(wire)
    }

    /// Like [`build`](Self::build), but `on_warning` is called once per non-fatal problem.
    pub fn build_with_warnings<F: FnMut(&WorkflowIssue)>(
        &self,
        on_warning: F,
    ) -> Result<AgentWorkflowRequestModel, WorkflowValidationError> {
        let wire = self.to_wire(Some(on_warning), &HashMap::new())?;
        to_request_model(wire)
    }

    /// Like [`build`](Self::build), with the id of each [`Tool`] object the nodes use, by tool
    /// name. `agents_as_code().push()` passes these after creating the tools. Every `Tool` object
    /// needs one.
    pub fn build_with_tool_ids(
        &self,
        tool_ids: &HashMap<String, String>,
    ) -> Result<AgentWorkflowRequestModel, WorkflowValidationError> {
        let wire = self.to_wire::<fn(&WorkflowIssue)>(None, tool_ids)?;
        to_request_model(wire)
    }

    /// The workflow in API format (snake_case keys, sorted at every level). Validates like
    /// `build()`.
    pub fn to_json(&self) -> Result<Value, WorkflowValidationError> {
        self.to_json_with_tool_ids(&HashMap::new())
    }

    /// Like [`to_json`](Self::to_json), with each [`Tool`] object's id, by tool name.
    pub fn to_json_with_tool_ids(
        &self,
        tool_ids: &HashMap<String, String>,
    ) -> Result<Value, WorkflowValidationError> {
        Ok(canonical(
            &self.to_wire::<fn(&WorkflowIssue)>(None, tool_ids)?,
        ))
    }

    /// `to_json()` as a string with 2-space indentation and a trailing newline.
    pub fn to_json_string(&self) -> Result<String, WorkflowValidationError> {
        Ok(wire_to_json_string(&self.to_json()?))
    }

    /// Like [`to_json_string`](Self::to_json_string), with each [`Tool`] object's id, by tool
    /// name.
    pub fn to_json_string_with_tool_ids(
        &self,
        tool_ids: &HashMap<String, String>,
    ) -> Result<String, WorkflowValidationError> {
        Ok(wire_to_json_string(&self.to_json_with_tool_ids(tool_ids)?))
    }

    /// The API-format workflow after the checks `build_with_tool_ids()` runs, including that it
    /// fits the generated request model. Keeps fields the model doesn't declare.
    pub(crate) fn build_wire(
        &self,
        tool_ids: &HashMap<String, String>,
    ) -> Result<Value, WorkflowValidationError> {
        let wire = self.to_wire::<fn(&WorkflowIssue)>(None, tool_ids)?;
        to_request_model(wire.clone())?;
        Ok(canonical(&wire))
    }

    fn to_wire<F: FnMut(&WorkflowIssue)>(
        &self,
        mut on_warning: Option<F>,
        tool_ids: &HashMap<String, String>,
    ) -> Result<Value, WorkflowValidationError> {
        let g = self.graph.borrow();
        let mut issues = validate(&g);
        issues.extend(unresolved_tools(&g, tool_ids));
        let errors: Vec<WorkflowIssue> = issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .cloned()
            .collect();
        if !errors.is_empty() {
            return Err(WorkflowValidationError { issues: errors });
        }
        if let Some(cb) = on_warning.as_mut() {
            for issue in &issues {
                if issue.severity == Severity::Warning {
                    cb(issue);
                }
            }
        }

        let merges = reverse_edge_merges(&g);
        let kept: Vec<&GraphEdge> = g
            .edges
            .iter()
            .enumerate()
            .filter(|(i, _)| !merges.contains_key(i))
            .map(|(_, e)| e)
            .collect();
        let assigned = assign_edge_ids(&kept);
        let id_of: HashMap<u64, String> = kept
            .iter()
            .zip(assigned)
            .map(|(e, id)| (e.uid, id))
            .collect();

        let positions = layout(&g);

        let mut outgoing: HashMap<String, Vec<u64>> = HashMap::new();
        let mut edges_wire: Map<String, Value> = Map::new();
        for (i, edge) in g.edges.iter().enumerate() {
            if let Some(&into) = merges.get(&i) {
                let reverse = &g.edges[into];
                outgoing.entry(edge.source.clone()).or_default().push(reverse.uid);
                let rid = &id_of[&reverse.uid];
                if let Some(obj) = edges_wire.get_mut(rid).and_then(|v| v.as_object_mut()) {
                    if let Some(fc) = &edge.forward_condition {
                        obj.insert("backward_condition".to_string(), fc.clone());
                    }
                }
                continue;
            }
            // An edge belongs in a node's outgoing order only for a transition that leaves it: the
            // source has a forward transition when there is a forward condition, the target a reverse
            // transition when there is a backward condition. A loaded edge may carry only one.
            if edge.forward_condition.is_some() {
                outgoing.entry(edge.source.clone()).or_default().push(edge.uid);
            }
            if edge.backward_condition.is_some() {
                outgoing.entry(edge.target.clone()).or_default().push(edge.uid);
            }
            let mut wire = edge.extra.clone();
            wire.insert("source".to_string(), json!(edge.source));
            wire.insert("target".to_string(), json!(edge.target));
            if let Some(fc) = &edge.forward_condition {
                wire.insert("forward_condition".to_string(), fc.clone());
            }
            if let Some(bc) = &edge.backward_condition {
                wire.insert("backward_condition".to_string(), bc.clone());
            }
            edges_wire.insert(id_of[&edge.uid].clone(), Value::Object(wire));
        }

        let mut nodes_wire: Map<String, Value> = Map::new();
        let resolve = |id: &str| -> String {
            match g.tool_ref(id) {
                Some(tool) => tool_ids
                    .get(tool.name())
                    .cloned()
                    .unwrap_or_else(|| id.to_string()),
                None => id.to_string(),
            }
        };
        for node_id in &g.node_order {
            let mut obj = g.nodes[node_id].clone();
            map_node_tool_ids(&mut obj, &resolve);
            obj.insert("position".to_string(), positions[node_id].clone());
            let loaded = g.loaded_order.get(node_id).cloned().unwrap_or_default();
            let current = outgoing.get(node_id).cloned().unwrap_or_default();
            let order_ids: Vec<Value> = edge_order(&loaded, &current)
                .into_iter()
                .map(|uid| json!(id_of[&uid]))
                .collect();
            obj.insert("edge_order".to_string(), Value::Array(order_ids));
            nodes_wire.insert(node_id.clone(), Value::Object(obj));
        }

        let mut workflow = Map::new();
        workflow.insert("nodes".to_string(), Value::Object(nodes_wire));
        workflow.insert("edges".to_string(), Value::Object(edges_wire));
        if let Some(psl) = g.prevent_subagent_loops {
            workflow.insert("prevent_subagent_loops".to_string(), json!(psl));
        }
        Ok(Value::Object(workflow))
    }
}

impl Chain for Workflow {
    fn graph(&self) -> &Rc<RefCell<WorkflowGraph>> {
        &self.graph
    }
    fn origin(&self) -> EdgeOrigin {
        EdgeOrigin {
            source: self.graph.borrow().start_id.clone(),
            forward_condition: Condition::always(None).to_wire(),
            backward_condition: None,
        }
    }
}

/// Deserialize validated wire output into the generated request model. Validation covers topology
/// and option names, not every field's shape, so caller-provided JSON (a loaded `reroutes` object
/// instead of an array, say) can still fail here; report that as a validation error, never panic.
fn to_request_model(wire: Value) -> Result<AgentWorkflowRequestModel, WorkflowValidationError> {
    serde_json::from_value(wire).map_err(|err| WorkflowValidationError {
        issues: vec![WorkflowIssue::error(
            WorkflowIssueCode::InvalidModel,
            format!(
                "The workflow doesn't match the agent workflow request model: {err}. A loaded field likely has the wrong shape; fix it in the source JSON"
            ),
            None,
        )],
    })
}

/// A node that uses a `Tool` object with no id in `tool_ids`.
fn unresolved_tools(g: &WorkflowGraph, tool_ids: &HashMap<String, String>) -> Vec<WorkflowIssue> {
    let mut issues = Vec::new();
    for node_id in &g.node_order {
        let Some(model) = g.nodes.get(node_id) else {
            continue;
        };
        for id in node_tool_ids(model) {
            if let Some(tool) = g.tool_ref(&id) {
                if !tool_ids.contains_key(tool.name()) {
                    issues.push(WorkflowIssue::error(
                        WorkflowIssueCode::UnresolvedTool,
                        format!(
                            "Node \"{node_id}\" uses tool \"{}\", which has no id yet. Push the agent with agents_as_code().push(), or pass build_with_tool_ids()",
                            tool.name()
                        ),
                        Some(node_id.clone()),
                    ));
                }
            }
        }
    }
    issues
}

/// A loaded node's order for the edges it still has, then any others in edge-list order.
fn edge_order(loaded: &[u64], current: &[u64]) -> Vec<u64> {
    let kept: Vec<u64> = loaded.iter().copied().filter(|u| current.contains(u)).collect();
    let mut out = kept.clone();
    for u in current {
        if !kept.contains(u) {
            out.push(*u);
        }
    }
    out
}

/// Loaded edges keep their ids. New ones get `source__target`, with `__2`, `__3`... on collision.
fn assign_edge_ids(edges: &[&GraphEdge]) -> Vec<String> {
    let mut taken: HashSet<String> = edges.iter().filter_map(|e| e.id.clone()).collect();
    let mut ids = Vec::new();
    for edge in edges {
        if let Some(id) = &edge.id {
            ids.push(id.clone());
            continue;
        }
        let base = format!("{}__{}", edge.source, edge.target);
        let mut edge_id = base.clone();
        let mut n = 2;
        while taken.contains(&edge_id) {
            edge_id = format!("{base}__{n}");
            n += 1;
        }
        taken.insert(edge_id.clone());
        ids.push(edge_id);
    }
    ids
}
