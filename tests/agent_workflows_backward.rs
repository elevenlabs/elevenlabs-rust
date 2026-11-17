//! A backward condition is a second transition (target → source) the API stores on the same edge.
//! It must obey the same routing rules as any outgoing edge, so validate() checks it in the reverse
//! direction too. These tests cover that reverse-direction validation.

use elevenlabs_rust_sdk::agent_workflows::{Chain, Workflow};

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
