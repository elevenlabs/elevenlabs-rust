//! Whole-graph validation, the reverse-edge merge rule, and BFS depths. Ported faithfully from the
//! Python/TS references so output is byte-identical.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use serde_json::Value;

use super::errors::{WorkflowIssue, WorkflowIssueCode};
use super::graph::WorkflowGraph;
use super::node_types::model_knows_node_type;

/// Checks that need the finished graph. Problems found while chaining are already in `graph.issues`.
pub fn validate(graph: &WorkflowGraph) -> Vec<WorkflowIssue> {
    let mut issues = graph.issues.clone();
    let merged = reverse_edge_merges(graph);

    let mut connected: HashSet<BTreeSet<String>> = HashSet::new();
    for (i, edge) in graph.edges.iter().enumerate() {
        if merged.contains_key(&i) {
            continue;
        }
        let pair: BTreeSet<String> = [edge.source.clone(), edge.target.clone()]
            .into_iter()
            .collect();
        if connected.contains(&pair) {
            issues.push(WorkflowIssue::error(
                WorkflowIssueCode::DuplicateEdge,
                format!(
                    "Edge \"{}\" → \"{}\" connects two nodes that already have an edge between them, and the API allows only one. Give the existing edge a different condition, or remove one",
                    edge.source, edge.target
                ),
                Some(edge.source.clone()),
            ));
        }
        connected.insert(pair);
    }

    let mut ends_with_edges: HashSet<String> = HashSet::new();
    for edge in &graph.edges {
        let name = format!("\"{}\" → \"{}\"", edge.source, edge.target);
        if edge.source == edge.target {
            issues.push(WorkflowIssue::error(
                WorkflowIssueCode::SelfLoop,
                format!("Edge {name} connects a node to itself, which the API doesn't allow"),
                Some(edge.source.clone()),
            ));
        }
        for id in [&edge.source, &edge.target] {
            if !graph.nodes.contains_key(id) {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::UnknownNode,
                    format!("Edge {name} refers to unknown node \"{id}\""),
                    Some(id.clone()),
                ));
            }
        }
        // Forward routing rules apply only when there is a forward transition (a forward condition).
        // A loaded edge can be backward-only, and then source → target does not exist.
        if let Some(forward) = &edge.forward_condition {
            let source_type = graph.node_type(&edge.source);
            let condition_type = forward.get("type").and_then(|v| v.as_str());
            if edge.target == graph.start_id {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::EdgeIntoStart,
                    format!(
                        "Edge {name} points into the start node; go to the first node after it instead"
                    ),
                    Some(edge.source.clone()),
                ));
            }
            if source_type.as_deref() == Some("end") && !ends_with_edges.contains(&edge.source) {
                ends_with_edges.insert(edge.source.clone());
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::EndHasOutgoingEdges,
                    format!("End node \"{}\" can't have outgoing edges", edge.source),
                    Some(edge.source.clone()),
                ));
            }
            if source_type.as_deref() == Some("tool") && condition_type != Some("result") {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::ToolEdgeNeedsResult,
                    format!(
                        "Edge {name} leaves a tool node, so it must use on_success() or on_failure()"
                    ),
                    Some(edge.source.clone()),
                ));
            }
            if source_type.is_some()
                && source_type.as_deref() != Some("tool")
                && condition_type == Some("result")
            {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::ResultEdgeNeedsTool,
                    format!(
                        "Edge {name} uses on_success()/on_failure(), which only applies to tool nodes"
                    ),
                    Some(edge.source.clone()),
                ));
            }
        }

        // A backward condition is a second transition the other way, target → source, that the API
        // stores on this same edge. It must satisfy the same routing rules as any edge leaving
        // edge.target, so check it in the reverse direction too.
        if let Some(backward) = &edge.backward_condition {
            let back_name = format!("\"{}\" → \"{}\"", edge.target, edge.source);
            let back_source_type = graph.node_type(&edge.target);
            let back_condition_type = backward.get("type").and_then(|v| v.as_str());
            if edge.source == graph.start_id {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::EdgeIntoStart,
                    format!("Edge {back_name} points into the start node; go to the first node after it instead"),
                    Some(edge.target.clone()),
                ));
            }
            if back_source_type.as_deref() == Some("end") && !ends_with_edges.contains(&edge.target)
            {
                ends_with_edges.insert(edge.target.clone());
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::EndHasOutgoingEdges,
                    format!("End node \"{}\" can't have outgoing edges", edge.target),
                    Some(edge.target.clone()),
                ));
            }
            if back_source_type.as_deref() == Some("tool") && back_condition_type != Some("result")
            {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::ToolEdgeNeedsResult,
                    format!("Edge {back_name} leaves a tool node, so it must use on_success() or on_failure()"),
                    Some(edge.target.clone()),
                ));
            }
            if back_source_type.is_some()
                && back_source_type.as_deref() != Some("tool")
                && back_condition_type == Some("result")
            {
                issues.push(WorkflowIssue::error(
                    WorkflowIssueCode::ResultEdgeNeedsTool,
                    format!("Edge {back_name} uses on_success()/on_failure(), which only applies to tool nodes"),
                    Some(edge.target.clone()),
                ));
            }
        }
    }

    for id in &graph.node_order {
        let node_type = graph.node_type(id);
        if !model_knows_node_type(node_type.as_deref()) {
            issues.push(WorkflowIssue::warning(
                WorkflowIssueCode::UnknownNodeType,
                format!(
                    "Node \"{id}\" has type \"{}\", which this SDK version doesn't model. build() keeps it as given but can't type it, and agents.create()/agents.update() would reject it. Upgrade the SDK, or send to_json() output directly.",
                    node_type.as_deref().unwrap_or("None")
                ),
                Some(id.clone()),
            ));
        }
    }

    let (reachable, _order) = depths_from(graph);
    for id in &graph.node_order {
        if !reachable.contains_key(id) {
            issues.push(WorkflowIssue::warning(
                WorkflowIssueCode::UnreachableNode,
                format!("Node \"{id}\" can't be reached from the start node"),
                Some(id.clone()),
            ));
        }
    }
    issues
}

/// Edges the API stores as the backward condition of another edge: index of the later edge → index
/// of the earlier one.
pub fn reverse_edge_merges(graph: &WorkflowGraph) -> HashMap<usize, usize> {
    let mut merges: HashMap<usize, usize> = HashMap::new();
    let mut taken: HashSet<usize> = HashSet::new();
    for (j, later) in graph.edges.iter().enumerate() {
        if later.source == later.target
            || later.backward_condition.is_some()
            || later.forward_condition.is_none()
        {
            continue;
        }
        let found = graph.edges[..j].iter().enumerate().find(|(k, earlier)| {
            !taken.contains(k)
                && !merges.contains_key(k)
                && earlier.source == later.target
                && earlier.target == later.source
                && earlier.backward_condition.is_none()
        });
        if let Some((i, _)) = found {
            merges.insert(j, i);
            taken.insert(i);
        }
    }
    merges
}

/// Breadth-first depth of every node reachable from the start, paired with the visiting order.
pub fn depths_from(graph: &WorkflowGraph) -> (HashMap<String, i64>, Vec<String>) {
    let mut depths: HashMap<String, i64> = HashMap::new();
    let mut order: Vec<String> = Vec::new();

    fn visit(
        graph: &WorkflowGraph,
        depths: &mut HashMap<String, i64>,
        order: &mut Vec<String>,
        start: &str,
        depth: i64,
    ) {
        depths.insert(start.to_string(), depth);
        order.push(start.to_string());
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(start.to_string());
        while let Some(current) = queue.pop_front() {
            let current_depth = depths[&current];
            for edge in &graph.edges {
                if edge.source == current
                    && edge.forward_condition.is_some()
                    && !depths.contains_key(&edge.target)
                    && graph.nodes.contains_key(&edge.target)
                {
                    depths.insert(edge.target.clone(), current_depth + 1);
                    order.push(edge.target.clone());
                    queue.push_back(edge.target.clone());
                }
            }
        }
    }

    let start_id = graph.start_id.clone();
    visit(graph, &mut depths, &mut order, &start_id, 0);
    loop {
        let back = graph.edges.iter().find(|e| {
            e.backward_condition.is_some()
                && depths.contains_key(&e.target)
                && !depths.contains_key(&e.source)
                && graph.nodes.contains_key(&e.source)
        });
        match back {
            None => break,
            Some(edge) => {
                let source = edge.source.clone();
                let depth = depths[&edge.target] + 1;
                visit(graph, &mut depths, &mut order, &source, depth);
            }
        }
    }
    (depths, order)
}

/// A whole number carried as JSON `x`/`y`, as an integer grid coordinate.
pub fn coord(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_f64().map(|f| f as i64))
}
