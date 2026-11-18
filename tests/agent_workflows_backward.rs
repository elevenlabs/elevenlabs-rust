//! A backward condition is a second transition (target → source) the API stores on the same edge.
//! It must obey the same routing rules as any outgoing edge, so validate() checks it in the reverse
//! direction too. These tests cover that reverse-direction validation.

use elevenlabs_rust_sdk::agent_workflows::{Chain, Condition, Options, Workflow};
use serde_json::json;

#[test]
fn backward_condition_into_start_is_rejected() {
    let wf = Workflow::new(None);
    // start --"Begin"--> greet, with a backward greet --"Restart"--> start.
    wf.when("Begin".into(), Some("Restart".into()))
        .converse("greet", "Hello", Default::default());
    let err = wf
        .build()
        .expect_err("a backward edge into the start node must fail");
    assert!(
        err.codes().contains(&"edge_into_start"),
        "expected edge_into_start, got {:?}",
        err.codes()
    );
}

#[test]
fn backward_condition_out_of_end_is_rejected() {
    let wf = Workflow::new(None);
    let triage = wf.converse("triage", "Triage", Default::default());
    // triage --"Done"--> end, with a backward end --"Resume"--> triage.
    triage
        .when("Done".into(), Some("Resume".into()))
        .end(Default::default());
    let err = wf
        .build()
        .expect_err("a backward edge out of an end node must fail");
    assert!(
        err.codes().contains(&"end_has_outgoing_edges"),
        "expected end_has_outgoing_edges, got {:?}",
        err.codes()
    );
}

#[test]
fn backward_loop_between_converse_nodes_is_allowed() {
    let wf = Workflow::new(None);
    let a = wf.converse("a", "A", Default::default());
    // a --"To B"--> b with a backward b --"Back to A"--> a: a normal loop, so no issue.
    let b = a
        .when("To B".into(), Some("Back to A".into()))
        .converse("b", "B", Default::default());
    b.end(Default::default());
    wf.build()
        .expect("a converse-to-converse backward loop is valid");
}

#[test]
fn loaded_edge_with_explicit_null_backward_condition_builds() {
    // A nullable wire field present as JSON `null` means "no condition", not a reverse transition. A
    // loaded start → converse edge with `backward_condition: null` must not trip edge_into_start.
    let wf = Workflow::new(None);
    wf.converse("greet", "Hi", Default::default())
        .end(Default::default());
    let json = wf.to_json_string().expect("valid workflow");
    let mut wire: serde_json::Value = serde_json::from_str(&json).unwrap();
    for (_id, edge) in wire["edges"].as_object_mut().unwrap().iter_mut() {
        edge.as_object_mut()
            .unwrap()
            .insert("backward_condition".into(), serde_json::Value::Null);
    }
    Workflow::from_json(&wire)
        .build()
        .expect("explicit null backward_condition must not invent a reverse transition");
}

#[test]
fn insert_between_bidirectional_edge_is_rejected() {
    let wf = Workflow::new(None);
    let a = wf.converse("a", "A", Default::default());
    // a → b with a backward b → a: a bidirectional edge.
    a.when("to b".into(), Some("back to a".into()))
        .converse("b", "B", Default::default());
    let result = wf.insert_between("a", "b", |br| {
        Some(br.converse("mid", "M", Default::default()))
    });
    assert!(
        result.is_none(),
        "inserting into a bidirectional edge must be rejected"
    );
    let err = wf
        .build()
        .expect_err("a rejected bidirectional insert records an error");
    assert!(
        err.codes().contains(&"cannot_insert_bidirectional"),
        "got {:?}",
        err.codes()
    );
}

#[test]
fn position_option_is_accepted_for_unmapped_node_types() {
    // `say`/`update_state` are valid generated node types with no dedicated option map; `position` is
    // shared by every node kind, so updating it must not record invalid_option.
    let fc = Condition::llm("go", None).to_wire();
    let wire = json!({
        "nodes": {
            "start_node": {"type": "start"},
            "announce": {"type": "say", "text": "hello"}
        },
        "edges": {
            "start_node__announce": {"source": "start_node", "target": "announce", "forward_condition": fc}
        }
    });
    let wf = Workflow::from_json(&wire);
    let mut opts = Options::new();
    opts.raw
        .insert("position".to_string(), json!({"x": 10.0, "y": 20.0}));
    wf.node("announce").update(opts);
    wf.build()
        .expect("position must be accepted on a node kind with no dedicated field map");
}

#[test]
fn loaded_backward_only_edge_has_no_forward_transition() {
    // An edge stored as x → start_node carrying ONLY a backward condition is the reverse transition
    // start_node → x; there is no forward x → start_node, so it must not trip edge_into_start, and x's
    // outgoing edge_order must not list it.
    let fc = Condition::llm("go", None).to_wire();
    let bc = Condition::llm("restart", None).to_wire();
    let wire = json!({
        "nodes": {
            "start_node": {"type": "start"},
            "hub": {"type": "override_agent", "label": "hub", "additional_prompt": "H"},
            "x": {"type": "override_agent", "label": "x", "additional_prompt": "X"},
            "end": {"type": "end"}
        },
        "edges": {
            "e1": {"source": "start_node", "target": "hub", "forward_condition": fc.clone()},
            "e2": {"source": "hub", "target": "x", "forward_condition": fc.clone()},
            "e3": {"source": "hub", "target": "end", "forward_condition": fc.clone()},
            "e4": {"source": "x", "target": "start_node", "backward_condition": bc}
        }
    });
    let wf = Workflow::from_json(&wire);
    let out = wf
        .to_json()
        .expect("a backward-only edge must not trip the forward routing checks");
    let x_order = out["nodes"]["x"]["edge_order"].as_array().unwrap();
    assert!(
        !x_order.iter().any(|v| v == "e4"),
        "x has no forward transition via e4, so it must not be in x's edge_order: {x_order:?}"
    );
}

#[test]
fn malformed_loaded_field_is_a_validation_error_not_a_panic() {
    // Validation covers topology and option names, not every field's shape. A loaded edge whose
    // `reroutes` is an object (the model wants an array) must make build() return an error.
    let wf = Workflow::new(None);
    wf.converse("greet", "Hi", Default::default())
        .end(Default::default());
    let mut wire: serde_json::Value = serde_json::from_str(&wf.to_json_string().unwrap()).unwrap();
    for (_id, edge) in wire["edges"].as_object_mut().unwrap().iter_mut() {
        edge.as_object_mut()
            .unwrap()
            .insert("reroutes".into(), json!({"x": 1, "y": 2}));
    }
    let loaded = Workflow::from_json(&wire);
    let err = loaded
        .build()
        .expect_err("a malformed field must be reported, not panic");
    assert_eq!(err.codes(), vec!["invalid_model"]);
    let err = loaded
        .build_with_warnings(|_| {})
        .expect_err("build_with_warnings reports it too");
    assert_eq!(err.codes(), vec!["invalid_model"]);
}
