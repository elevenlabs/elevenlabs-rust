//! Cross-SDK conformance: run every shared tool fixture through the Rust tool builder and compare
//! `to_json_string()` byte-for-byte (or the sorted error codes from `build()`). Mirrors the JS
//! runner in `tests/custom/agents/tools/fixtureRunner.ts`; see the tool spec, "Fixtures".

use std::fs;
use std::path::PathBuf;

use elevenlabs_rust_sdk::agent_tools::{
    AssignOptions, AuthConnection, Destination, Header, Schema, SystemTool, Tool, ToolOptions,
};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
struct Fixture {
    name: String,
    tool: Option<ToolSpec>,
    from: Option<Value>,
    schema: Option<Vec<Op>>,
    #[serde(default)]
    ops: Vec<Op>,
    expected: Option<Value>,
    errors: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ToolSpec {
    webhook: Option<(String, String)>,
    client: Option<(String, String)>,
    system: Option<String>,
    #[serde(rename = "systemOptions")]
    system_options: Option<Map<String, Value>>,
}

#[derive(Deserialize)]
struct Op {
    op: String,
    #[serde(default)]
    args: Vec<Value>,
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

/// A fixture options object (camelCase keys) with the Rust option names.
fn snake_keys(raw: Option<&Value>) -> Map<String, Value> {
    raw.and_then(Value::as_object)
        .map(|map| {
            map.iter()
                .map(|(key, value)| (camel_to_snake(key), value.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn tool_options(raw: Option<&Value>) -> ToolOptions {
    snake_keys(raw)
        .into_iter()
        .fold(ToolOptions::new(), |options, (key, value)| {
            options.set(&key, value)
        })
}

fn str_arg(op: &Op, index: usize) -> &str {
    op.args[index]
        .as_str()
        .unwrap_or_else(|| panic!("{}: argument {index} must be a string", op.op))
}

fn strings_arg(op: &Op, index: usize) -> Vec<String> {
    op.args[index]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

fn destination(raw: &Value) -> Destination {
    let field = |key: &str| raw.get(key).and_then(Value::as_str);
    if let Some(p) = field("phone") {
        return Destination::phone(p);
    }
    if let Some(p) = field("phoneVariable") {
        return Destination::phone_variable(p);
    }
    if let Some(p) = field("sip") {
        return Destination::sip(p);
    }
    if let Some(p) = field("sipVariable") {
        return Destination::sip_variable(p);
    }
    Destination::agent(field("agent").unwrap_or_default(), field("nodeId"))
}

fn header(raw: &Value) -> Header {
    let field = |key: &str| raw.get(key).and_then(Value::as_str);
    if let Some(v) = field("value") {
        return Header::value(v);
    }
    if let Some(v) = field("secret") {
        return Header::secret(v);
    }
    if let Some(v) = field("variable") {
        return Header::variable(v);
    }
    Header::env_var(field("envVar").unwrap_or_default())
}

fn auth(raw: &Value) -> AuthConnection {
    match raw.get("id").and_then(Value::as_str) {
        Some(id) => AuthConnection::id(id),
        None => AuthConnection::env_var(raw["envVar"].as_str().unwrap_or_default()),
    }
}

/// A schema argument is wire-format JSON, or `{"build": [ops]}` for one built with `Schema`
/// methods (e.g. `optional()`, which has no wire form of its own).
fn schema_arg(raw: &Value) -> Schema {
    match raw.get("build").and_then(Value::as_array) {
        Some(ops) => {
            let ops: Vec<Op> = ops
                .iter()
                .map(|op| serde_json::from_value(op.clone()).unwrap())
                .collect();
            build_schema(&ops)
        }
        None => Schema::from_json(raw),
    }
}

fn build_schema(ops: &[Op]) -> Schema {
    let (first, rest) = ops.split_first().expect("a schema needs a constructor op");
    let mut schema: Schema = match first.op.as_str() {
        "string" => Schema::string().into(),
        "integer" => Schema::integer().into(),
        "number" => Schema::number().into(),
        "bool" => Schema::bool().into(),
        "object" => Schema::object().into(),
        "array" => Schema::array(schema_arg(&first.args[0])).into(),
        "fromJSON" => Schema::from_json(&first.args[0]),
        other => panic!("unknown schema constructor {other}"),
    };
    for op in rest {
        schema = match op.op.as_str() {
            "describe" => schema.describe(str_arg(op, 0)),
            "fromVariable" => schema.from_variable(str_arg(op, 0)),
            "constant" => schema.constant(op.args[0].clone()),
            "omit" => schema.omit(),
            "optional" => schema.optional(),
            "systemProvided" => literal(schema, op).system_provided().into(),
            "choices" => literal(schema, op).choices(strings_arg(op, 0)).into(),
            "allowedFrom" => literal(schema, op).allowed_from(str_arg(op, 0)).into(),
            "property" => object(schema, op)
                .property(str_arg(op, 0), schema_arg(&op.args[1]))
                .into(),
            "without" => object(schema, op).without(str_arg(op, 0)).into(),
            "requireAnyOf" => object(schema, op).require_any_of(strings_arg(op, 0)).into(),
            "requireAllOf" => object(schema, op).require_all_of(strings_arg(op, 0)).into(),
            other => panic!("unknown schema op {other}"),
        };
    }
    schema
}

fn literal(schema: Schema, op: &Op) -> elevenlabs_rust_sdk::agent_tools::LiteralSchema {
    schema
        .into_literal()
        .unwrap_or_else(|| panic!("{} needs a literal schema", op.op))
}

fn object(schema: Schema, op: &Op) -> elevenlabs_rust_sdk::agent_tools::ObjectSchema {
    schema
        .into_object()
        .unwrap_or_else(|| panic!("{} needs an object schema", op.op))
}

fn system_tool(system_tool_type: &str, options: Option<&Map<String, Value>>) -> SystemTool {
    let options = Value::Object(snake_keys(
        options.map(|m| Value::Object(m.clone())).as_ref(),
    ));
    fn typed<T: serde::de::DeserializeOwned>(options: &Value) -> T {
        serde_json::from_value(options.clone()).unwrap()
    }
    match system_tool_type {
        "end_call" => SystemTool::end_call(),
        "skip_turn" => SystemTool::skip_turn(typed(&options)),
        "language_detection" => SystemTool::language_detection(typed(&options)),
        "play_keypad_touch_tone" => SystemTool::play_keypad_touch_tone(typed(&options)),
        "voicemail_detection" => SystemTool::voicemail_detection(typed(&options)),
        "knowledge_base" => SystemTool::knowledge_base(typed(&options)),
        "transfer_to_number" => SystemTool::transfer_to_number(typed(&options)),
        "transfer_to_agent" => SystemTool::transfer_to_agent(),
        other => panic!("unknown system tool {other}"),
    }
}

// ---- applying ops ---------------------------------------------------------------------------

fn apply(tool: Tool, op: &Op) -> Tool {
    if op.op == "options" {
        return tool.options(tool_options(op.args.first()));
    }
    match tool {
        Tool::Webhook(t) => Tool::Webhook(match op.op.as_str() {
            "get" => t.get(str_arg(op, 0)),
            "post" => t.post(str_arg(op, 0)),
            "put" => t.put(str_arg(op, 0)),
            "patch" => t.patch(str_arg(op, 0)),
            "delete" => t.delete(str_arg(op, 0)),
            "pathParam" => t.path_param(str_arg(op, 0), schema_arg(&op.args[1])),
            "queryParam" => t.query_param(str_arg(op, 0), schema_arg(&op.args[1])),
            "body" => t.body(schema_arg(&op.args[0])),
            "response" => t.response(schema_arg(&op.args[0])),
            "header" => t.header(str_arg(op, 0), header(&op.args[1])),
            "auth" => t.auth(auth(&op.args[0])),
            "assign" => {
                let raw = snake_keys(op.args.get(2));
                let flag = |key: &str| raw.get(key).and_then(Value::as_bool);
                let options = AssignOptions {
                    sanitize: flag("sanitize"),
                    preserve_native_type: flag("preserve_native_type"),
                };
                t.assign(str_arg(op, 0), str_arg(op, 1), options)
            }
            "removePathParam" => t.remove_path_param(str_arg(op, 0)),
            "removeQueryParam" => t.remove_query_param(str_arg(op, 0)),
            "removeHeader" => t.remove_header(str_arg(op, 0)),
            "removeAssignment" => t.remove_assignment(str_arg(op, 0)),
            other => panic!("unknown webhook op {other}"),
        }),
        Tool::Client(t) => Tool::Client(match op.op.as_str() {
            "param" => t.param(str_arg(op, 0), schema_arg(&op.args[1])),
            "removeParam" => t.remove_param(str_arg(op, 0)),
            other => panic!("unknown client op {other}"),
        }),
        Tool::System(t) => Tool::System(match op.op.as_str() {
            "transfer" => t.transfer(
                str_arg(op, 0),
                destination(&op.args[1]),
                tool_options(op.args.get(2)),
            ),
            other => panic!("unknown system op {other}"),
        }),
        Tool::Other(_) => panic!("op {} on a tool of an unknown type", op.op),
    }
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
    if let Some(ops) = &fixture.schema {
        return Ok(serde_json::to_string_pretty(&build_schema(ops).to_json()).unwrap() + "\n");
    }
    let mut tool: Tool = match (&fixture.from, &fixture.tool) {
        (Some(from), _) => Tool::from_json(from),
        (None, Some(spec)) => {
            if let Some((name, description)) = &spec.webhook {
                Tool::webhook(name, description).into()
            } else if let Some((name, description)) = &spec.client {
                Tool::client(name, description).into()
            } else {
                let system = spec.system.as_deref().expect("a tool kind");
                system_tool(system, spec.system_options.as_ref()).into()
            }
        }
        (None, None) => panic!("{}: fixture has no tool", fixture.name),
    };
    for op in &fixture.ops {
        tool = apply(tool, op);
    }
    tool.to_json_string()
        .map_err(|err| err.codes().iter().map(|c| c.to_string()).collect())
}

#[test]
fn all_fixtures_pass() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/agent_tools");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("fixtures dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();
    files.sort();
    assert_eq!(files.len(), 21, "expected 21 fixtures");

    let mut failures = Vec::new();
    for path in files {
        let text = fs::read_to_string(&path).unwrap();
        let fixture: Fixture = serde_json::from_str(&text).unwrap();
        let name = format!(
            "{} ({})",
            fixture.name,
            path.file_name().unwrap().to_string_lossy()
        );
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
                    Ok(_) => failures.push(format!(
                        "{name}: expected errors {want:?} but build succeeded"
                    )),
                }
            }
            (None, None) => panic!("{name}: fixture has neither expected nor errors"),
        }
    }

    assert!(failures.is_empty(), "\n\n{}\n", failures.join("\n\n"));
}

/// Every fixture that expects JSON also loads back and writes byte-identically (the round-trip
/// guarantee of the deterministic output rules).
#[test]
fn every_expected_output_round_trips() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/agent_tools");
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let fixture: Fixture = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let Some(expected) = &fixture.expected else {
            continue;
        };
        if fixture.schema.is_some() {
            let schema = Schema::from_json(expected);
            assert_eq!(schema.to_json(), canonical(expected), "{}", fixture.name);
            continue;
        }
        let tool = Tool::from_json(expected);
        assert_eq!(
            tool.to_json_string().unwrap(),
            canonical_string(expected),
            "{}",
            fixture.name
        );
    }
}
