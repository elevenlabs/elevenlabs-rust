//! Node handles and the verbs that add a node and connect it to the current point.
//!
//! All graph state lives in the `Workflow`; handles share it through `Rc<RefCell<WorkflowGraph>>`
//! (the spec permits this for Rust). Each verb returns a [`WorkflowNode`], so linear flows chain.

use std::cell::RefCell;
use std::rc::Rc;

use serde_json::{json, Map, Value};

use super::condition::{to_condition, Condition, ConditionInput};
use super::destination::Destination;
use super::errors::WorkflowIssueCode;
use super::graph::{AppliedOptions, GraphEdge, WorkflowGraph};

/// Options passed to a verb. `destination` is kept apart (it is a [`Destination`], not wire JSON);
/// `raw` holds canonical snake_case keys mapped to already-wire values (including `tool_ids` and
/// `schema_overrides`).
#[derive(Default, Clone)]
pub struct Options {
    pub destination: Option<Destination>,
    pub raw: Map<String, Value>,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }
    /// Set a canonical snake_case option to a wire value.
    pub fn set(mut self, key: &str, value: Value) -> Self {
        self.raw.insert(key.to_string(), value);
        self
    }
    /// Set the transfer destination (for `update`).
    pub fn destination(mut self, destination: Destination) -> Self {
        self.destination = Some(destination);
        self
    }
}

fn into_applied(opts: Options) -> AppliedOptions {
    let Options {
        destination,
        mut raw,
    } = opts;
    let tool_ids = raw.remove("tool_ids").and_then(|v| {
        v.as_array().map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
    });
    let schema_overrides = raw.remove("schema_overrides");
    let rest: Vec<(String, Value)> = raw.into_iter().collect();
    AppliedOptions {
        destination,
        tool_ids,
        schema_overrides,
        rest,
    }
}

/// Where the next edge starts and which conditions it carries.
#[derive(Clone)]
pub struct EdgeOrigin {
    pub source: String,
    pub forward_condition: Value,
    pub backward_condition: Option<Value>,
}

fn connect(
    graph: &Rc<RefCell<WorkflowGraph>>,
    origin: &EdgeOrigin,
    target: &str,
    back: Option<ConditionInput>,
) -> WorkflowNode {
    let backward_condition = match back {
        Some(b) => Some(to_condition(b).to_wire()),
        None => origin.backward_condition.clone(),
    };
    {
        let mut g = graph.borrow_mut();
        let uid = g.next_uid();
        g.edges.push(GraphEdge {
            uid,
            source: origin.source.clone(),
            target: target.to_string(),
            forward_condition: Some(origin.forward_condition.clone()),
            backward_condition,
            id: None,
            extra: Map::new(),
        });
    }
    WorkflowNode::new(
        Rc::clone(graph),
        target.to_string(),
        Some(origin.source.clone()),
    )
}

/// The verbs that add a node and connect it to the current point.
pub trait Chain {
    #[doc(hidden)]
    fn graph(&self) -> &Rc<RefCell<WorkflowGraph>>;
    #[doc(hidden)]
    fn origin(&self) -> EdgeOrigin;

    /// Add a subagent (`override_agent`) node holding a conversation with `prompt` added to its
    /// instructions, until an outgoing condition holds. `label` defaults to `id`.
    fn converse(&self, id: &str, prompt: &str, opts: Options) -> WorkflowNode {
        let mut base = Map::new();
        base.insert("type".to_string(), json!("override_agent"));
        base.insert("label".to_string(), json!(id));
        base.insert("additional_prompt".to_string(), json!(prompt));
        let model = {
            let mut g = self.graph().borrow_mut();
            g.apply_options(id, base, into_applied(opts))
        };
        self.graph().borrow_mut().add_node(id, model);
        connect(self.graph(), &self.origin(), id, None)
    }

    /// Add a `tool` node that runs `tool_ids` in parallel. Branch on the result with `on_result()`.
    fn tool(&self, id: &str, tool_ids: Vec<String>, opts: Options) -> WorkflowNode {
        let mut base = Map::new();
        base.insert("type".to_string(), json!("tool"));
        let mut applied = into_applied(opts);
        applied.tool_ids = Some(tool_ids);
        let model = {
            let mut g = self.graph().borrow_mut();
            g.apply_options(id, base, applied)
        };
        self.graph().borrow_mut().add_node(id, model);
        connect(self.graph(), &self.origin(), id, None)
    }

    /// Add a `transfer` node (phone/SIP or agent). The destination is the base of the node model.
    fn transfer(&self, id: &str, destination: Destination, opts: Options) -> WorkflowNode {
        let base = destination
            .to_wire()
            .as_object()
            .cloned()
            .unwrap_or_default();
        let model = {
            let mut g = self.graph().borrow_mut();
            g.apply_options(id, base, into_applied(opts))
        };
        self.graph().borrow_mut().add_node(id, model);
        connect(self.graph(), &self.origin(), id, None)
    }

    /// Add an `end` node. `options.id` defaults to `"end"`; reusing an end id reuses that node.
    fn end(&self, mut opts: Options) -> WorkflowNode {
        let id = opts
            .raw
            .remove("id")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_else(|| "end".to_string());
        let mut base = Map::new();
        base.insert("type".to_string(), json!("end"));
        let model = {
            let mut g = self.graph().borrow_mut();
            g.apply_options(&id, base, into_applied(opts))
        };
        self.graph().borrow_mut().add_node(&id, model);
        connect(self.graph(), &self.origin(), &id, None)
    }

    /// Move the conversation to an existing (or later-created) node, for merges and loops. `back`
    /// is the condition for coming back along this edge.
    fn go_to(&self, target: &str, back: Option<ConditionInput>) -> WorkflowNode {
        connect(self.graph(), &self.origin(), target, back)
    }
}

pub(crate) fn branch(
    graph: &Rc<RefCell<WorkflowGraph>>,
    source: &str,
    condition: ConditionInput,
    back: Option<ConditionInput>,
) -> WorkflowBranch {
    WorkflowBranch {
        graph: Rc::clone(graph),
        from: EdgeOrigin {
            source: source.to_string(),
            forward_condition: to_condition(condition).to_wire(),
            backward_condition: back.map(|b| to_condition(b).to_wire()),
        },
    }
}

/// A pending edge out of a node, opened by `when()`, `on_success()` or `on_failure()`.
pub struct WorkflowBranch {
    graph: Rc<RefCell<WorkflowGraph>>,
    from: EdgeOrigin,
}

impl WorkflowBranch {
    pub(crate) fn from_origin(graph: Rc<RefCell<WorkflowGraph>>, from: EdgeOrigin) -> Self {
        Self { graph, from }
    }
}

impl Chain for WorkflowBranch {
    fn graph(&self) -> &Rc<RefCell<WorkflowGraph>> {
        &self.graph
    }
    fn origin(&self) -> EdgeOrigin {
        self.from.clone()
    }
}

/// A handle to one node in a workflow. Handles are cheap: all state lives in the `Workflow`.
#[derive(Clone)]
pub struct WorkflowNode {
    graph: Rc<RefCell<WorkflowGraph>>,
    pub id: String,
    parent_id: Option<String>,
}

impl WorkflowNode {
    pub(crate) fn new(
        graph: Rc<RefCell<WorkflowGraph>>,
        id: String,
        parent_id: Option<String>,
    ) -> Self {
        Self {
            graph,
            id,
            parent_id,
        }
    }

    /// The API node type, for example `"override_agent"` or `"tool"`.
    pub fn node_type(&self) -> Option<String> {
        self.graph.borrow().node_type(&self.id)
    }

    /// Open a branch that is followed when `condition` holds.
    pub fn when(&self, condition: ConditionInput, back: Option<ConditionInput>) -> WorkflowBranch {
        branch(&self.graph, &self.id, condition, back)
    }

    /// The node this handle was chained from (for writing a whole workflow as one chain).
    pub fn up(&self) -> WorkflowNode {
        let parent = self.parent_id.clone().unwrap_or_else(|| {
            let g = self.graph.borrow();
            g.edges
                .iter()
                .find(|e| e.target == self.id && e.source != self.id)
                .map(|e| e.source.clone())
                .unwrap_or_else(|| g.start_id.clone())
        });
        WorkflowNode::new(Rc::clone(&self.graph), parent, None)
    }

    /// Open the branch followed when every tool in this tool node succeeded.
    pub fn on_success(&self, label: Option<&str>) -> WorkflowBranch {
        branch(
            &self.graph,
            &self.id,
            ConditionInput::Condition(Condition::success(label)),
            None,
        )
    }

    /// Open the branch followed when any tool in this tool node failed.
    pub fn on_failure(&self, label: Option<&str>) -> WorkflowBranch {
        branch(
            &self.graph,
            &self.id,
            ConditionInput::Condition(Condition::failure(label)),
            None,
        )
    }

    /// Route both outcomes of this tool node. `success` runs on the success branch, then `failure`
    /// on the failure branch — always in that order. Returns this node.
    pub fn on_result<S, F>(&self, success: Option<S>, failure: Option<F>) -> &Self
    where
        S: FnOnce(WorkflowBranch),
        F: FnOnce(WorkflowBranch),
    {
        if let Some(success) = success {
            success(self.on_success(None));
        }
        if let Some(failure) = failure {
            failure(self.on_failure(None));
        }
        self
    }

    /// Change this node's options. Only options that apply to the node's kind may be set; `build()`
    /// reports the rest.
    pub fn update(&self, opts: Options) -> &Self {
        let model = self.graph.borrow().nodes.get(&self.id).cloned();
        match model {
            None => {
                self.graph.borrow_mut().issue(
                    WorkflowIssueCode::UnknownNode,
                    format!("Can't update unknown node \"{}\"", self.id),
                    Some(&self.id),
                );
            }
            Some(model) => {
                let updated = self
                    .graph
                    .borrow_mut()
                    .apply_options(&self.id, model, into_applied(opts));
                self.graph
                    .borrow_mut()
                    .nodes
                    .insert(self.id.clone(), updated);
            }
        }
        self
    }
}

impl Chain for WorkflowNode {
    fn graph(&self) -> &Rc<RefCell<WorkflowGraph>> {
        &self.graph
    }
    fn origin(&self) -> EdgeOrigin {
        EdgeOrigin {
            source: self.id.clone(),
            forward_condition: Condition::always(None).to_wire(),
            backward_condition: None,
        }
    }
}
