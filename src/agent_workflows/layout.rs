//! Editor positions for nodes without one: column = BFS depth from start, row = discovery order in
//! that column, skipping grid cells a positioned node already occupies.

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use super::graph::WorkflowGraph;
use super::validate::{coord, depths_from};

const COLUMN_WIDTH: i64 = 300;
const ROW_HEIGHT: i64 = 200;

pub fn layout(graph: &WorkflowGraph) -> HashMap<String, Value> {
    let (depths, visit_order) = depths_from(graph);
    let last_column = depths.values().copied().max().unwrap_or(0).max(0) + 1;

    // Visiting order, then any remaining nodes in insertion order.
    let mut order = visit_order.clone();
    for id in &graph.node_order {
        if !depths.contains_key(id) {
            order.push(id.clone());
        }
    }

    let mut occupied: HashSet<(i64, i64)> = HashSet::new();
    for id in &graph.node_order {
        if let Some(pos) = graph.nodes.get(id).and_then(|m| m.get("position")) {
            if let (Some(x), Some(y)) = (
                pos.get("x").and_then(coord),
                pos.get("y").and_then(coord),
            ) {
                occupied.insert((x, y));
            }
        }
    }

    let mut rows: HashMap<i64, i64> = HashMap::new();
    let mut positions: HashMap<String, Value> = HashMap::new();
    for id in &order {
        if let Some(existing) = graph.nodes.get(id).and_then(|m| m.get("position")) {
            positions.insert(id.clone(), existing.clone());
            continue;
        }
        let column = depths.get(id).copied().unwrap_or(last_column);
        let mut row = rows.get(&column).copied().unwrap_or(0);
        while occupied.contains(&(column * COLUMN_WIDTH, row * ROW_HEIGHT)) {
            row += 1;
        }
        rows.insert(column, row + 1);
        occupied.insert((column * COLUMN_WIDTH, row * ROW_HEIGHT));
        positions.insert(
            id.clone(),
            json!({ "x": column * COLUMN_WIDTH, "y": row * ROW_HEIGHT }),
        );
    }
    positions
}
