//! Which node types this SDK version's generated request model can represent.

/// Node `type` discriminants modelled by the generated [`crate::AgentWorkflowRequestModelNodes`]
/// enum. Mirrors its variants, so `build()`'s output serializes without hitting the `__Unknown`
/// catch-all. A loaded node whose type is not here gets an `unknown_node_type` warning.
const KNOWN_NODE_TYPES: &[&str] = &[
    "start",
    "end",
    "override_agent",
    "phone_number",
    "standalone_agent",
    "tool",
    "say",
    "update_state",
];

/// Whether this SDK version's generated request model has a member for `node_type`.
pub fn model_knows_node_type(node_type: Option<&str>) -> bool {
    matches!(node_type, Some(t) if KNOWN_NODE_TYPES.contains(&t))
}
