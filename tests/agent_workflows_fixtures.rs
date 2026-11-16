//! Cross-SDK conformance: run every shared fixture through the Rust workflow builder and compare
//! `to_json_string()` byte-for-byte (or the sorted error codes from `build()`). Mirrors the JS
//! runner in `tests/custom/agents/workflow/fixtureRunner.ts`.

use std::fs;
use std::path::PathBuf;

use elevenlabs_rust_sdk::agent_workflows::{
    Chain, Condition, ConditionInput, Destination, Expr, Options, Workflow, WorkflowBranch,
    WorkflowNode,
};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
struct Fixture {
    name: String,
    workflow: Option<WorkflowOpts>,
    from: Option<Value>,
    ops: Vec<Op>,
    expected: Option<Value>,
    errors: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct WorkflowOpts {
    #[serde(rename = "preventSubagentLoops")]
    prevent_subagent_loops: Option<bool>,
}

#[derive(Deserialize)]
struct Op {
    op: String,
    at: Option<String>,
    when: Option<Value>,
    back: Option<Value>,
    #[serde(rename = "onSuccess")]
    on_success: Option<bool>,
    #[serde(rename = "onFailure")]
    on_failure: Option<bool>,
    id: Option<String>,
    prompt: Option<String>,
    #[serde(rename = "toolIds")]
    tool_ids: Option<Vec<String>>,
    destination: Option<Value>,
    target: Option<String>,
    node: Option<String>,
    source: Option<String>,
    options: Option<Map<String, Value>>,
    ops: Option<Vec<Op>>,
    success: Option<Vec<Op>>,
    failure: Option<Vec<Op>>,
}

// ---- fixture value helpers (mirror fixtureRunner.ts) ----------------------------------------

fn camel_to_snake(key: &str) -> String {
    let mut out = String::new();
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn parse_condition(raw: &Value) -> ConditionInput {
    if let Some(s) = raw.as_str() {
        return ConditionInput::Str(s.to_string());
    }
    let label = raw.get("label").and_then(|v| v.as_str());
    if let Some(llm) = raw.get("llm").and_then(|v| v.as_str()) {
        return ConditionInput::Condition(Condition::llm(llm, label));
    }
    let ast = raw.get("expr").cloned().unwrap_or(Value::Null);
    ConditionInput::Condition(Condition::expr(&Expr::from_model(ast), label))
}

fn parse_destination(raw: &Value) -> Destination {
    if let Some(p) = raw.get("phone").and_then(|v| v.as_str()) {
        return Destination::phone(p);
    }
    if let Some(p) = raw.get("phoneVariable").and_then(|v| v.as_str()) {
        return Destination::phone_variable(p);
    }
    if let Some(p) = raw.get("sip").and_then(|v| v.as_str()) {
        return Destination::sip(p);
    }
    if let Some(p) = raw.get("sipVariable").and_then(|v| v.as_str()) {
        return Destination::sip_variable(p);
    }
    let agent = raw.get("agent").and_then(|v| v.as_str()).unwrap_or_default();
    let node_id = raw.get("nodeId").and_then(|v| v.as_str());
    Destination::agent(agent, node_id)
}

fn build_options(op: &Op) -> Options {
    let mut options = Options::new();
    if let Some(raw) = &op.options {
        for (key, value) in raw {
            if key == "destination" {
                options.destination = Some(parse_destination(value));
            } else {
                options.raw.insert(camel_to_snake(key), value.clone());
            }
        }
    }
    options
}

// ---- a heterogeneous "current chain point" (Workflow | node | branch) -----------------------

enum Chainish<'a> {
    Wf(&'a Workflow),
    Node(WorkflowNode),
    Branch(WorkflowBranch),
}

impl Chainish<'_> {
    fn as_chain(&self) -> &dyn Chain {
        match self {
            Chainish::Wf(w) => *w,
            Chainish::Node(n) => n,
            Chainish::Branch(b) => b,
        }
    }
    fn when(&self, cond: ConditionInput, back: Option<ConditionInput>) -> WorkflowBranch {
        match self {
            Chainish::Wf(w) => w.when(cond, back),
            Chainish::Node(n) => n.when(cond, back),
            Chainish::Branch(_) => panic!("when() is not available on a branch"),
        }
    }
    fn on_success(&self) -> WorkflowBranch {
        match self {
            Chainish::Node(n) => n.on_success(None),
            _ => panic!("onSuccess() requires a node"),
        }
    }
    fn on_failure(&self) -> WorkflowBranch {
        match self {
            Chainish::Node(n) => n.on_failure(None),
            _ => panic!("onFailure() requires a node"),
        }
    }
}

fn run(wf: &Workflow, ops: &[Op], first: Chainish) -> Option<WorkflowNode> {
    let mut previous = first;
    let mut result: Option<WorkflowNode> = None;
    for op in ops {
        match op.op.as_str() {
            "update" => {
                wf.node(op.node.as_deref().unwrap()).update(build_options(op));
                continue;
            }
            "remove" => {
                wf.remove(op.node.as_deref().unwrap());
                continue;
            }
            "onResult" => {
                let node = match &op.at {
                    Some(at) => wf.node(at),
                    None => match &previous {
                        Chainish::Node(n) => n.clone(),
                        _ => panic!("onResult requires a preceding node"),
                    },
                };
                let success = op
                    .success
                    .as_ref()
                    .map(|s_ops| move |b: WorkflowBranch| {
                        run(wf, s_ops, Chainish::Branch(b));
                    });
                let failure = op
                    .failure
                    .as_ref()
                    .map(|f_ops| move |b: WorkflowBranch| {
                        run(wf, f_ops, Chainish::Branch(b));
                    });
                node.on_result(success, failure);
                continue;
            }
            "insertBetween" => {
                let empty: Vec<Op> = Vec::new();
                let nested = op.ops.as_ref().unwrap_or(&empty);
                wf.insert_between(
                    op.source.as_deref().unwrap(),
                    op.target.as_deref().unwrap(),
                    |b| run(wf, nested, Chainish::Branch(b)),
                );
                continue;
            }
            _ => {}
        }

        let base: Chainish = match &op.at {
            None => std::mem::replace(&mut previous, Chainish::Wf(wf)),
            Some(a) if a == "$start" => Chainish::Wf(wf),
            Some(a) => Chainish::Node(wf.node(a)),
        };

        let at: Chainish = if let Some(when) = &op.when {
            let back = op.back.as_ref().map(parse_condition);
            Chainish::Branch(base.when(parse_condition(when), back))
        } else if op.on_success == Some(true) {
            Chainish::Branch(base.on_success())
        } else if op.on_failure == Some(true) {
            Chainish::Branch(base.on_failure())
        } else {
            base
        };

        let node = match op.op.as_str() {
            "converse" => at.as_chain().converse(
                op.id.as_deref().unwrap(),
                op.prompt.as_deref().unwrap(),
                build_options(op),
            ),
            "tool" => at.as_chain().tool(
                op.id.as_deref().unwrap(),
                op.tool_ids.clone().unwrap_or_default(),
                build_options(op),
            ),
            "transfer" => at.as_chain().transfer(
                op.id.as_deref().unwrap(),
                parse_destination(op.destination.as_ref().unwrap()),
                build_options(op),
            ),
            "end" => at.as_chain().end(build_options(op)),
            "goTo" => at.as_chain().go_to(
                op.target.as_deref().unwrap(),
                op.back.as_ref().map(parse_condition),
            ),
            other => panic!("unknown op {other}"),
        };
        previous = Chainish::Node(node.clone());
        result = Some(node);
    }
    result
}

// ---- canonical comparison (same rules as the builder's wire module) -------------------------

fn canonical(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut out = Map::new();
            for k in keys {
                out.insert(k.clone(), canonical(&map[k]));
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonical).collect()),
        Value::Number(n) => {
            if n.as_i64().is_none() && n.as_u64().is_none() {
                if let Some(f) = n.as_f64() {
                    if f.fract() == 0.0 && f.abs() < i64::MAX as f64 {
                        return Value::Number((f as i64).into());
                    }
                }
            }
            value.clone()
        }
        other => other.clone(),
    }
}

fn canonical_string(value: &Value) -> String {
    serde_json::to_string_pretty(&canonical(value)).unwrap() + "\n"
}

fn run_fixture(fixture: &Fixture) -> Result<String, Vec<String>> {
    let wf = match &fixture.from {
        Some(from) => Workflow::from_json(from),
        None => Workflow::new(
            fixture
                .workflow
                .as_ref()
                .and_then(|w| w.prevent_subagent_loops),
        ),
    };
    run(&wf, &fixture.ops, Chainish::Wf(&wf));
    match wf.to_json_string() {
        Ok(json) => Ok(json),
        Err(err) => Err(err.codes().iter().map(|c| c.to_string()).collect()),
    }
}

#[test]
fn all_fixtures_pass() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/agent_workflows");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("fixtures dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();
    files.sort();
    assert_eq!(files.len(), 8, "expected 8 fixtures");

    let mut failures = Vec::new();
    for path in files {
        let text = fs::read_to_string(&path).unwrap();
        let fixture: Fixture = serde_json::from_str(&text).unwrap();
        let name = format!("{} ({})", fixture.name, path.file_name().unwrap().to_string_lossy());
        let outcome = run_fixture(&fixture);
        match (&fixture.expected, &fixture.errors) {
            (Some(expected), _) => {
                let want = canonical_string(expected);
                match outcome {
                    Ok(got) if got == want => {}
                    Ok(got) => failures.push(format!(
                        "{name}: JSON mismatch\n--- got ---\n{got}\n--- want ---\n{want}"
                    )),
                    Err(codes) => {
                        failures.push(format!("{name}: expected JSON but build failed: {codes:?}"))
                    }
                }
            }
            (None, Some(errors)) => {
                let mut want = errors.clone();
                want.sort();
                match outcome {
                    Err(got) if got == want => {}
                    Err(got) => failures.push(format!(
                        "{name}: error mismatch\n  got:  {got:?}\n  want: {want:?}"
                    )),
                    Ok(_) => failures.push(format!("{name}: expected errors {want:?} but build succeeded")),
                }
            }
            (None, None) => panic!("{name}: fixture has neither expected nor errors"),
        }
    }

    assert!(failures.is_empty(), "\n\n{}\n", failures.join("\n\n"));
}
