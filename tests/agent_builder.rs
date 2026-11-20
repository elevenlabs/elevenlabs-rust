//! Agents as code against a local fake of the agents and tools endpoints (after the JS SDK's
//! `fakeServer.ts`), plus the shared project fixture checks through the public API.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use elevenlabs_rust_sdk::agent_builder::{
    Agent, AgentIssueCode, AgentOptions, AgentProject, AgentPushAction, PullOptions, PushError,
    PushOptions, ToolPushAction,
};
use elevenlabs_rust_sdk::agent_tools::{Schema, SystemTool, Tool};
use elevenlabs_rust_sdk::agent_workflows::{Chain, Options, Workflow};
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use serde_json::{json, Map, Value};
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

// ---- the fake server -------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Call {
    method: String,
    path: String,
    query: BTreeMap<String, String>,
    body: Option<Value>,
}

#[derive(Default)]
struct State {
    /// `(id, config, used_by)`, in insertion order.
    tools: Vec<(String, Value, Vec<String>)>,
    agents: HashMap<String, Value>,
    /// Branch configs by `agent_id/branch_id`. Only the main branch counts for
    /// `used_by_agent_id`, as in the API.
    branches: HashMap<String, Value>,
    calls: Vec<Call>,
    /// Fails every request whose `METHOD path` equals this.
    fail_on: Option<String>,
    next: u64,
}

/// An in-memory stand-in for the agents and tools endpoints `push` and `pull` use.
struct FakeServer {
    server: MockServer,
    state: Arc<Mutex<State>>,
}

struct Responder(Arc<Mutex<State>>);

impl Respond for Responder {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let mut state = self.0.lock().unwrap();
        state.handle(request)
    }
}

fn json_response(body: Value, status: u16) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_json(body)
}

impl State {
    fn handle(&mut self, request: &Request) -> ResponseTemplate {
        let method = request.method.as_str().to_string();
        let path = request.url.path().to_string();
        let query: BTreeMap<String, String> = request.url.query_pairs().into_owned().collect();
        let body: Option<Value> = serde_json::from_slice(&request.body).ok();
        self.calls.push(Call {
            method: method.clone(),
            path: path.clone(),
            query: query.clone(),
            body: body.clone(),
        });
        if self.fail_on.as_deref() == Some(format!("{method} {path}").as_str()) {
            return json_response(json!({ "detail": "boom" }), 500);
        }
        let parts: Vec<&str> = path.split('/').collect(); // ["", "v1", "convai", kind, id]
        let kind = parts.get(3).copied();
        let id = parts.get(4).map(|s| s.to_string());
        match kind {
            Some("tools") => {
                if method == "GET" && id.is_none() {
                    let agent_id = query.get("used_by_agent_id");
                    let tools: Vec<Value> = self
                        .tools
                        .iter()
                        .filter(|(_, _, used)| agent_id.is_none_or(|a| used.contains(a)))
                        .map(|(id, config, _)| tool_response(id, config))
                        .collect();
                    return json_response(json!({ "tools": tools, "has_more": false }), 200);
                }
                if method == "POST" {
                    self.next += 1;
                    let tool_id = format!("tool_new_{}", self.next);
                    let config = body.unwrap()["tool_config"].clone();
                    self.tools
                        .push((tool_id.clone(), config.clone(), Vec::new()));
                    return json_response(tool_response(&tool_id, &config), 200);
                }
                let id = id.unwrap();
                let Some(tool) = self.tools.iter_mut().find(|(t, _, _)| *t == id) else {
                    return json_response(json!({ "detail": "not found" }), 404);
                };
                if method == "PATCH" {
                    tool.1 = body.unwrap()["tool_config"].clone();
                }
                json_response(tool_response(&id, &tool.1), 200)
            }
            Some("agents") => {
                let id = id.unwrap();
                if method == "POST" && id == "create" {
                    self.next += 1;
                    let agent_id = format!("agent_new_{}", self.next);
                    self.store_agent(&agent_id, body.unwrap());
                    return json_response(json!({ "agent_id": agent_id }), 200);
                }
                if let Some(branch_id) = query.get("branch_id") {
                    let key = format!("{id}/{branch_id}");
                    if method == "PATCH" {
                        let merged = shallow_merge(self.branches.get(&key), body.unwrap());
                        self.branches.insert(key.clone(), merged);
                    }
                    return match self.branches.get(&key) {
                        Some(branch) => json_response(agent_response(&id, branch), 200),
                        None => json_response(json!({ "detail": "not found" }), 404),
                    };
                }
                if method == "PATCH" {
                    let merged = shallow_merge(self.agents.get(&id), body.unwrap());
                    self.store_agent(&id, merged);
                }
                match self.agents.get(&id) {
                    Some(agent) => json_response(agent_response(&id, agent), 200),
                    None => json_response(json!({ "detail": "not found" }), 404),
                }
            }
            _ => panic!("unexpected request {method} {path}"),
        }
    }

    fn store_agent(&mut self, agent_id: &str, config: Value) {
        let mut ids: Vec<String> = config["conversation_config"]["agent"]["prompt"]["tool_ids"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect();
        for node in config["workflow"]["nodes"]
            .as_object()
            .into_iter()
            .flat_map(|n| n.values())
        {
            for tool in node["tools"].as_array().into_iter().flatten() {
                ids.extend(tool["tool_id"].as_str().map(str::to_string));
            }
            for id in node["additional_tool_ids"].as_array().into_iter().flatten() {
                ids.extend(id.as_str().map(str::to_string));
            }
        }
        self.agents.insert(agent_id.to_string(), config);
        for (tool_id, _, used) in &mut self.tools {
            used.retain(|a| a != agent_id);
            if ids.contains(tool_id) {
                used.push(agent_id.to_string());
            }
        }
    }
}

fn shallow_merge(existing: Option<&Value>, body: Value) -> Value {
    let mut merged = existing
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (k, v) in body.as_object().cloned().unwrap_or_default() {
        merged.insert(k, v);
    }
    Value::Object(merged)
}

fn tool_response(id: &str, config: &Value) -> Value {
    json!({
        "id": id,
        "tool_config": config,
        "access_info": { "is_creator": true, "creator_name": "a", "creator_email": "a@example.com", "role": "admin" },
        "usage_stats": { "avg_latency_secs": 0.1 }
    })
}

/// The defaults the API fills into a stored workflow.
fn with_server_defaults(workflow: &Value) -> Value {
    let mut workflow = workflow.clone();
    if let Some(nodes) = workflow.get_mut("nodes").and_then(Value::as_object_mut) {
        for node in nodes.values_mut() {
            if node["type"] == "override_agent" {
                let mut full: Map<String, Value> = json!({
                    "conversation_config": {},
                    "additional_knowledge_base": [],
                    "additional_tool_ids": [],
                    "entry_behavior": "generate_immediately",
                    "allow_interruptions_on_entry": true
                })
                .as_object()
                .cloned()
                .unwrap();
                full.extend(node.as_object().cloned().unwrap());
                *node = Value::Object(full);
            }
        }
    }
    if let Some(edges) = workflow.get_mut("edges").and_then(Value::as_object_mut) {
        for edge in edges.values_mut() {
            let mut full = Map::new();
            full.insert("reroutes".into(), json!([]));
            full.extend(edge.as_object().cloned().unwrap());
            *edge = Value::Object(full);
        }
    }
    let obj = workflow.as_object_mut().unwrap();
    obj.entry("prevent_subagent_loops").or_insert(json!(false));
    workflow
}

fn agent_response(id: &str, config: &Value) -> Value {
    let mut response = json!({
        "agent_id": id,
        "metadata": { "created_at_unix_secs": 1, "updated_at_unix_secs": 1 },
        "version_id": "ver_1"
    });
    let obj = response.as_object_mut().unwrap();
    obj.extend(config.as_object().cloned().unwrap_or_default());
    if let Some(workflow) = config.get("workflow") {
        obj.insert("workflow".into(), with_server_defaults(workflow));
    }
    response
}

impl FakeServer {
    async fn start() -> Self {
        let server = MockServer::start().await;
        let state = Arc::new(Mutex::new(State::default()));
        Mock::given(any())
            .respond_with(Responder(Arc::clone(&state)))
            .mount(&server)
            .await;
        Self { server, state }
    }

    fn client(&self) -> ApiClient {
        let config = ClientConfig::builder()
            .base_url(self.server.uri())
            .api_key("test")
            .max_retries(0)
            .build()
            .unwrap();
        ApiClient::new(config).unwrap()
    }

    fn add_tool(&self, id: &str, config: Value, used_by: &[&str]) {
        self.state.lock().unwrap().tools.push((
            id.to_string(),
            config,
            used_by.iter().map(|s| s.to_string()).collect(),
        ));
    }

    fn add_agent(&self, id: &str, config: Value) {
        self.state
            .lock()
            .unwrap()
            .agents
            .insert(id.to_string(), config);
    }

    fn agent(&self, id: &str) -> Value {
        self.state.lock().unwrap().agents[id].clone()
    }

    fn tool_config(&self, id: &str) -> Option<Value> {
        let state = self.state.lock().unwrap();
        state
            .tools
            .iter()
            .find(|(t, _, _)| t == id)
            .map(|(_, c, _)| c.clone())
    }

    fn set_tool_config(&self, id: &str, config: Value) {
        let mut state = self.state.lock().unwrap();
        state.tools.iter_mut().find(|(t, _, _)| t == id).unwrap().1 = config;
    }

    fn set_branch(&self, key: &str, config: Value) {
        self.state
            .lock()
            .unwrap()
            .branches
            .insert(key.to_string(), config);
    }

    fn fail_on(&self, call: &str) {
        self.state.lock().unwrap().fail_on = Some(call.to_string());
    }

    fn calls(&self) -> Vec<Call> {
        self.state.lock().unwrap().calls.clone()
    }

    fn clear_calls(&self) {
        self.state.lock().unwrap().calls.clear();
    }

    /// The calls that change something, as `METHOD path`.
    fn writes(&self) -> Vec<String> {
        self.calls()
            .into_iter()
            .filter(|c| c.method != "GET")
            .map(|c| format!("{} {}", c.method, c.path))
            .collect()
    }
}

// ---- helpers ---------------------------------------------------------------------------------

fn get_order(description: &str) -> Tool {
    Tool::webhook("get_order", description)
        .get("https://api.example.com/orders/{order_id}")
        .path_param("order_id", Schema::string().describe("The order number"))
        .into()
}

fn default_get_order() -> Tool {
    get_order("Look up an order by its number")
}

fn show_order() -> Tool {
    Tool::client("show_order", "Show the order card")
        .param("order_id", Schema::string().from_variable("order_id"))
        .into()
}

fn support_agent(tools: Vec<Tool>) -> Agent {
    let wf = Workflow::new(None);
    wf.converse("triage", "Find out what the caller needs", Options::new())
        .when("Caller gave an order number".into(), None)
        .tool("lookup", vec![tools[0].clone().into()], Options::new())
        .on_success(None)
        .converse("resolve", "Explain the order", Options::new());
    let mut agent_tools: Vec<Tool> = vec![SystemTool::end_call().into()];
    agent_tools.extend(tools.into_iter().skip(1));
    Agent::new(
        "Support line",
        AgentOptions::new()
            .prompt("You help with orders.")
            .tools(agent_tools)
            .workflow(wf),
    )
}

fn default_support_agent() -> Agent {
    support_agent(vec![default_get_order(), show_order()])
}

fn push_opts<'a>() -> PushOptions<'a> {
    PushOptions::default()
}

fn with_project(project: &mut AgentProject) -> PushOptions<'_> {
    PushOptions {
        project: Some(project),
        ..Default::default()
    }
}

fn actions(
    result: &elevenlabs_rust_sdk::agent_builder::AgentPushResult,
) -> Vec<(String, String, &'static str)> {
    result
        .tools
        .iter()
        .map(|(name, t)| (name.clone(), t.id.clone(), t.action.as_str()))
        .collect()
}

fn entry(name: &str, id: &str, action: &'static str) -> (String, String, &'static str) {
    (name.to_string(), id.to_string(), action)
}

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/agent_builder/project/cli-graph")
}

fn read_tree(root: &Path) -> BTreeMap<String, String> {
    fn walk(root: &Path, dir: &Path, files: &mut BTreeMap<String, String>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(root, &path, files);
            } else {
                let rel = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(rel, std::fs::read_to_string(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    walk(root, root, &mut files);
    files
}

fn temp_dir(prefix: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "{prefix}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

// ---- shared fixture checks -------------------------------------------------------------------

#[test]
fn fixture_loads_with_the_same_agents_tools_ids_and_attached_ids() {
    let project = AgentProject::from_files(&read_tree(&fixture_dir())).unwrap();
    let agents: Vec<&str> = project.agents().iter().map(|a| a.name()).collect();
    assert_eq!(agents, ["Support", "Billing"]);
    let tools: Vec<&str> = project.tools().iter().map(|t| t.name()).collect();
    assert_eq!(tools, ["lookup_order", "issue_refund"]);
    let tool_ids: Vec<Option<&str>> = project
        .tools()
        .iter()
        .map(|t| project.tool_id(t).unwrap())
        .collect();
    assert_eq!(tool_ids, [Some("tool_lookup"), Some("tool_refund")]);

    let support = project.agent("Support").unwrap();
    assert_eq!(project.agent_id(support).unwrap(), Some("agent_support"));
    let names: Vec<String> = support
        .tools()
        .iter()
        .map(|t| t.name().to_string())
        .collect();
    assert_eq!(names, ["lookup_order"]);
    assert_eq!(support.tool_ids(), ["tool_from_dashboard"]);

    let billing = project.agent("Billing").unwrap();
    assert_eq!(project.agent_id(billing).unwrap(), Some("agent_billing"));
    let names: Vec<String> = billing
        .tools()
        .iter()
        .map(|t| t.name().to_string())
        .collect();
    assert_eq!(names, ["issue_refund"]);
    assert!(billing.tool_ids().is_empty());
}

#[test]
fn fixture_gives_back_every_file_byte_identical_when_nothing_was_pushed() {
    let files = read_tree(&fixture_dir());
    let project = AgentProject::from_files(&files).unwrap();
    assert_eq!(project.to_files(), files);
    assert!(project.changed_files().is_empty());

    let opened = AgentProject::open(fixture_dir()).unwrap();
    assert_eq!(opened.to_files(), files);
}

#[test]
fn fixture_support_config_round_trips_to_its_sorted_form() {
    let raw =
        parse(&std::fs::read_to_string(fixture_dir().join("agent_configs/support.json")).unwrap());
    let lookup = Tool::from_json(&parse(
        &std::fs::read_to_string(fixture_dir().join("tool_configs/lookup_order.json")).unwrap(),
    ));
    let agent =
        Agent::from_json_with_tools(&raw, &HashMap::from([("tool_lookup".to_string(), lookup)]));
    let out = agent
        .to_json(&HashMap::from([(
            "lookup_order".to_string(),
            "tool_lookup".to_string(),
        )]))
        .unwrap();
    assert_eq!(out, raw);
    // Sorted at every level: the serialized text is the sorted text.
    fn sorted(value: &Value) -> Value {
        match value {
            Value::Object(map) => {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                Value::Object(
                    keys.into_iter()
                        .map(|k| (k.clone(), sorted(&map[k])))
                        .collect(),
                )
            }
            Value::Array(items) => Value::Array(items.iter().map(sorted).collect()),
            other => other.clone(),
        }
    }
    assert_eq!(
        serde_json::to_string(&out).unwrap(),
        serde_json::to_string(&sorted(&raw)).unwrap()
    );
}

// ---- push ------------------------------------------------------------------------------------

#[tokio::test]
async fn creates_the_tools_then_the_agent_with_their_ids() {
    let server = FakeServer::start().await;
    let result = server
        .client()
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap();

    assert_eq!(
        server.writes(),
        [
            "POST /v1/convai/tools",
            "POST /v1/convai/tools",
            "POST /v1/convai/agents/create"
        ]
    );
    assert_eq!(result.agent_id, "agent_new_3");
    assert_eq!(result.action, AgentPushAction::Created);
    assert_eq!(
        actions(&result),
        [
            entry("get_order", "tool_new_2", "created"),
            entry("show_order", "tool_new_1", "created"),
        ]
    );
    let sent = server.agent("agent_new_3");
    assert_eq!(
        sent["conversation_config"]["agent"]["prompt"]["tool_ids"],
        json!(["tool_new_1"])
    );
    assert_eq!(
        sent["conversation_config"]["agent"]["prompt"]["built_in_tools"]["end_call"]["name"],
        "end_call"
    );
    assert_eq!(
        sent["workflow"]["nodes"]["lookup"]["tools"],
        json!([{ "tool_id": "tool_new_2" }])
    );
}

#[tokio::test]
async fn updates_only_what_changed_and_leaves_another_agents_same_named_tool_alone() {
    let server = FakeServer::start().await;
    let client = server.client();
    let created = client
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap();
    server.add_tool(
        "tool_other",
        get_order("Someone else's").to_json().unwrap(),
        &["agent_other"],
    );
    server.clear_calls();

    let again = client
        .agents_as_code()
        .push(
            &support_agent(vec![get_order("Find an order"), show_order()]),
            PushOptions {
                agent_id: Some(created.agent_id.clone()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(
        server.writes(),
        [
            "PATCH /v1/convai/tools/tool_new_2".to_string(),
            format!("PATCH /v1/convai/agents/{}", created.agent_id)
        ]
    );
    let first = &server.calls()[0];
    assert_eq!(
        first.query,
        BTreeMap::from([
            ("used_by_agent_id".to_string(), created.agent_id.clone()),
            ("page_size".to_string(), "100".to_string()),
        ])
    );
    assert_eq!(
        actions(&again),
        [
            entry("get_order", "tool_new_2", "updated"),
            entry("show_order", "tool_new_1", "unchanged"),
        ]
    );
    assert_eq!(again.action, AgentPushAction::Updated);
}

#[tokio::test]
async fn reports_tools_the_agent_stopped_using_as_detached_without_deleting_them() {
    let server = FakeServer::start().await;
    let client = server.client();
    let created = client
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap();
    let mut agent = default_support_agent();
    agent.update(AgentOptions::new().tools(vec![]));
    let result = client
        .agents_as_code()
        .push(
            &agent,
            PushOptions {
                agent_id: Some(created.agent_id),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(result.tools["show_order"].id, "tool_new_1");
    assert_eq!(result.tools["show_order"].action, ToolPushAction::Detached);
    assert!(server.tool_config("tool_new_1").is_some());
    assert!(!server.writes().iter().any(|w| w.starts_with("DELETE")));
}

#[tokio::test]
async fn matches_a_branch_push_against_the_branchs_tools_not_the_main_branchs() {
    let server = FakeServer::start().await;
    let client = server.client();
    let created = client
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap();
    let agent_id = created.agent_id;
    // The branch uses its own copy of get_order, and no show_order.
    server.add_tool(
        "tool_branch_order",
        get_order("Branch lookup").to_json().unwrap(),
        &[],
    );
    let mut branch = server.agent(&agent_id);
    branch["workflow"]["nodes"]["lookup"]["tools"] = json!([{ "tool_id": "tool_branch_order" }]);
    branch["conversation_config"]["agent"]["prompt"]["tool_ids"] = json!([]);
    server.set_branch(&format!("{agent_id}/agtbrch_1"), branch);
    server.clear_calls();

    let result = client
        .agents_as_code()
        .push(
            &support_agent(vec![get_order("Changed lookup")]),
            PushOptions {
                agent_id: Some(agent_id.clone()),
                branch_id: Some("agtbrch_1".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(
        actions(&result),
        [entry("get_order", "tool_branch_order", "updated")]
    );
    assert_eq!(
        server.writes(),
        [
            "PATCH /v1/convai/tools/tool_branch_order".to_string(),
            format!("PATCH /v1/convai/agents/{agent_id}")
        ]
    );
    let calls = server.calls();
    assert!(!calls
        .iter()
        .any(|c| c.query.contains_key("used_by_agent_id")));
    let patch = calls
        .iter()
        .find(|c| c.method == "PATCH" && c.path.contains("/agents/"))
        .unwrap();
    assert_eq!(
        patch.query.get("branch_id").map(String::as_str),
        Some("agtbrch_1")
    );
}

#[tokio::test]
async fn validates_before_any_api_call() {
    let server = FakeServer::start().await;
    let agent = Agent::new(
        "A",
        AgentOptions::new()
            .prompt("p")
            .tools(vec![Tool::webhook("broken", "No URL").into()]),
    );
    let err = server
        .client()
        .agents_as_code()
        .push(&agent, push_opts())
        .await
        .unwrap_err();
    assert!(matches!(err, PushError::Validation(_)), "{err}");
    assert!(server.calls().is_empty());
}

#[tokio::test]
async fn reports_what_it_wrote_before_a_failure() {
    let server = FakeServer::start().await;
    server.fail_on("POST /v1/convai/agents/create");
    let err = server
        .client()
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap_err();
    let PushError::Push(err) = err else {
        panic!("expected a push error, got {err}");
    };
    assert!(
        err.message
            .starts_with("Couldn't create agent \"Support line\""),
        "{}",
        err.message
    );
    assert!(err
        .to_string()
        .starts_with("Couldn't create agent \"Support line\": "));
    assert_eq!(err.result.agent_id, None);
    assert_eq!(err.result.tools["show_order"].id, "tool_new_1");
    assert_eq!(
        err.result.tools["show_order"].action,
        ToolPushAction::Created
    );
    assert_eq!(err.result.tools["get_order"].id, "tool_new_2");
    assert_eq!(
        err.result.tools["get_order"].action,
        ToolPushAction::Created
    );
}

#[tokio::test]
async fn saves_the_ids_written_before_a_failure_to_a_project_on_disk() {
    let server = FakeServer::start().await;
    server.fail_on("POST /v1/convai/agents/create");
    let dir = temp_dir("agents-partial");
    let mut project = AgentProject::open(&dir).unwrap();
    let err = server
        .client()
        .agents_as_code()
        .push(&default_support_agent(), with_project(&mut project))
        .await
        .unwrap_err();
    assert!(matches!(err, PushError::Push(_)));
    let tools = parse(&std::fs::read_to_string(dir.join("tools.json")).unwrap());
    assert_eq!(
        tools["tools"],
        json!([
            { "type": "client", "config": "tool_configs/show_order.json", "id": "tool_new_1" },
            { "type": "webhook", "config": "tool_configs/get_order.json", "id": "tool_new_2" }
        ])
    );
    assert!(!dir.join("agents.json").exists());

    // Pushing again with the saved project reuses the tools and creates only the agent.
    server.state.lock().unwrap().fail_on = None;
    server.clear_calls();
    let mut reopened = AgentProject::open(&dir).unwrap();
    let result = server
        .client()
        .agents_as_code()
        .push(&default_support_agent(), with_project(&mut reopened))
        .await
        .unwrap();
    assert_eq!(server.writes(), ["POST /v1/convai/agents/create"]);
    assert_eq!(result.tools["get_order"].action, ToolPushAction::Unchanged);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn refuses_agent_id_together_with_a_project() {
    let server = FakeServer::start().await;
    let mut project = AgentProject::from_files(Vec::<(String, String)>::new()).unwrap();
    let err = server
        .client()
        .agents_as_code()
        .push(
            &default_support_agent(),
            PushOptions {
                agent_id: Some("agent_1".into()),
                project: Some(&mut project),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(matches!(err, PushError::InvalidOptions(_)));
    assert!(err.to_string().contains("not both"));
    assert!(server.calls().is_empty());
}

// ---- push with a project ---------------------------------------------------------------------

#[tokio::test]
async fn records_ids_and_writes_the_cli_layout_then_updates_by_recorded_id() {
    let server = FakeServer::start().await;
    let client = server.client();
    let mut project = AgentProject::from_files(Vec::<(String, String)>::new()).unwrap();

    client
        .agents_as_code()
        .push(&default_support_agent(), with_project(&mut project))
        .await
        .unwrap();
    let files = project.to_files();
    assert_eq!(
        files.keys().collect::<Vec<_>>(),
        [
            "agent_configs/Support-line.json",
            "agents.json",
            "tool_configs/get_order.json",
            "tool_configs/show_order.json",
            "tools.json"
        ]
    );
    assert_eq!(
        files["agents.json"],
        "{\n    \"agents\": [\n        {\n            \"config\": \"agent_configs/Support-line.json\",\n            \"id\": \"agent_new_3\"\n        }\n    ]\n}"
    );
    assert_eq!(
        parse(&files["tools.json"]),
        json!({ "tools": [
            { "type": "client", "config": "tool_configs/show_order.json", "id": "tool_new_1" },
            { "type": "webhook", "config": "tool_configs/get_order.json", "id": "tool_new_2" }
        ] })
    );
    assert_eq!(
        parse(&files["agent_configs/Support-line.json"])["conversation_config"]["agent"]["prompt"]
            ["tool_ids"],
        json!(["tool_new_1"])
    );

    // A fresh script run: new objects, matched to the project's entries by name.
    server.clear_calls();
    let mut reloaded = AgentProject::from_files(&files).unwrap();
    let result = client
        .agents_as_code()
        .push(&default_support_agent(), with_project(&mut reloaded))
        .await
        .unwrap();
    assert_eq!(server.writes(), ["PATCH /v1/convai/agents/agent_new_3"]);
    assert_eq!(result.tools["get_order"].action, ToolPushAction::Unchanged);
    // Only the version the update reported is new, recorded like the CLI does.
    assert_eq!(reloaded.changed_files(), ["agents.json"]);
    assert_eq!(
        parse(&reloaded.to_files()["agents.json"])["agents"][0]["version_id"],
        "ver_1"
    );
}

#[tokio::test]
async fn pushes_a_loaded_agents_settings_the_sdk_does_not_model() {
    let server = FakeServer::start().await;
    let client = server.client();
    server.add_tool(
        "tool_lookup",
        json!({ "type": "client", "name": "lookup", "description": "old" }),
        &[],
    );
    let files = BTreeMap::from([
        (
            "agents.json",
            json!({ "agents": [{ "config": "agent_configs/a.json", "id": "agent_1" }] }).to_string(),
        ),
        (
            "tools.json",
            json!({ "tools": [{ "type": "client", "config": "tool_configs/t.json", "id": "tool_lookup" }] })
                .to_string(),
        ),
        (
            "agent_configs/a.json",
            json!({
                "name": "A",
                "conversation_config": {
                    "agent": { "prompt": { "prompt": "p", "tool_ids": ["tool_lookup"] } },
                    "future_setting": { "on": true }
                },
                "platform_settings": {},
                "tags": []
            })
            .to_string(),
        ),
        (
            "tool_configs/t.json",
            json!({ "type": "client", "name": "lookup", "description": "d", "future_field": 1 })
                .to_string(),
        ),
    ]);
    let mut project = AgentProject::from_files(&files).unwrap();
    server.add_agent("agent_1", json!({}));

    let mut agent = project.agent("A").unwrap().clone();
    agent.update(AgentOptions::new().first_message("Hello"));
    client
        .agents_as_code()
        .push(&agent, with_project(&mut project))
        .await
        .unwrap();

    let calls = server.calls();
    let tool_patch = calls
        .iter()
        .find(|c| c.method == "PATCH" && c.path.contains("/tools/"))
        .unwrap();
    assert_eq!(
        tool_patch.body.as_ref().unwrap()["tool_config"]["future_field"],
        1
    );
    let agent_patch = calls
        .iter()
        .find(|c| c.method == "PATCH" && c.path.contains("/agents/"))
        .unwrap();
    let body = agent_patch.body.as_ref().unwrap();
    assert_eq!(
        body["conversation_config"]["future_setting"],
        json!({ "on": true })
    );
    assert_eq!(
        body["conversation_config"]["agent"]["first_message"],
        "Hello"
    );
    let mut changed = project.changed_files();
    changed.sort();
    assert_eq!(
        changed,
        [
            "agent_configs/a.json",
            "agents.json",
            "tool_configs/t.json",
            "tools.json"
        ]
    );
    // The edited clone is now the project's agent.
    assert_eq!(
        project
            .agent("A")
            .unwrap()
            .to_json(&HashMap::from([(
                "lookup".to_string(),
                "tool_lookup".to_string()
            )]))
            .unwrap()["conversation_config"]["agent"]["first_message"],
        "Hello"
    );
}

#[tokio::test]
async fn updates_a_project_tool_whose_only_change_is_a_field_the_sdk_does_not_model() {
    let server = FakeServer::start().await;
    server.add_tool(
        "tool_1",
        json!({ "type": "client", "name": "ping", "description": "d", "future_field": 1 }),
        &["agent_1"],
    );
    server.add_agent("agent_1", json!({}));
    let files = BTreeMap::from([
        (
            "agents.json",
            json!({ "agents": [{ "config": "agent_configs/a.json", "id": "agent_1" }] }).to_string(),
        ),
        (
            "tools.json",
            json!({ "tools": [{ "type": "client", "config": "tool_configs/t.json", "id": "tool_1" }] })
                .to_string(),
        ),
        (
            "agent_configs/a.json",
            json!({ "name": "A", "conversation_config": { "agent": { "prompt": { "prompt": "p", "tool_ids": ["tool_1"] } } } })
                .to_string(),
        ),
        (
            "tool_configs/t.json",
            json!({ "type": "client", "name": "ping", "description": "d", "future_field": 2 })
                .to_string(),
        ),
    ]);
    let mut project = AgentProject::from_files(&files).unwrap();
    let agent = project.agent("A").unwrap().clone();
    let result = server
        .client()
        .agents_as_code()
        .push(&agent, with_project(&mut project))
        .await
        .unwrap();
    assert_eq!(result.tools["ping"].action, ToolPushAction::Updated);
    assert_eq!(server.tool_config("tool_1").unwrap()["future_field"], 2);
}

#[tokio::test]
async fn brings_the_project_up_to_date_when_the_server_already_matches_a_new_definition() {
    let server = FakeServer::start().await;
    let client = server.client();
    let mut project = AgentProject::from_files(Vec::<(String, String)>::new()).unwrap();
    client
        .agents_as_code()
        .push(
            &support_agent(vec![get_order("v2"), show_order()]),
            with_project(&mut project),
        )
        .await
        .unwrap();
    let mut files = project.to_files();
    files.insert(
        "tool_configs/get_order.json".into(),
        serde_json::to_string_pretty(&get_order("v1").to_json().unwrap()).unwrap(),
    );
    let mut stale = AgentProject::from_files(&files).unwrap();
    let result = client
        .agents_as_code()
        .push(
            &support_agent(vec![get_order("v2"), show_order()]),
            with_project(&mut stale),
        )
        .await
        .unwrap();
    assert_eq!(result.tools["get_order"].action, ToolPushAction::Unchanged);
    assert_eq!(
        parse(&stale.to_files()["tool_configs/get_order.json"])["description"],
        "v2"
    );
}

#[tokio::test]
async fn does_not_report_a_tool_as_detached_while_the_workflow_still_uses_its_id() {
    let server = FakeServer::start().await;
    let client = server.client();
    server.add_tool("tool_raw", show_order().to_json().unwrap(), &[]);
    let wf = Workflow::new(None);
    wf.tool("run", vec!["tool_raw".into()], Options::new());
    let agent = Agent::new("A", AgentOptions::new().prompt("p").workflow(wf));
    let created = client
        .agents_as_code()
        .push(&agent, push_opts())
        .await
        .unwrap();
    let again = client
        .agents_as_code()
        .push(
            &agent,
            PushOptions {
                agent_id: Some(created.agent_id),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(again.tools.is_empty());
}

#[tokio::test]
async fn keeps_a_project_tools_unmodeled_fields_when_the_same_tool_is_rebuilt_in_code() {
    let server = FakeServer::start().await;
    let client = server.client();
    let mut project = AgentProject::from_files(Vec::<(String, String)>::new()).unwrap();
    client
        .agents_as_code()
        .push(&default_support_agent(), with_project(&mut project))
        .await
        .unwrap();
    let mut files = project.to_files();
    let mut with_extra = parse(&files["tool_configs/get_order.json"]);
    with_extra["future_field"] = json!(1);
    files.insert(
        "tool_configs/get_order.json".into(),
        serde_json::to_string_pretty(&with_extra).unwrap(),
    );
    server.set_tool_config("tool_new_2", with_extra);

    let mut reloaded = AgentProject::from_files(&files).unwrap();
    let result = client
        .agents_as_code()
        .push(&default_support_agent(), with_project(&mut reloaded))
        .await
        .unwrap();
    assert_eq!(result.tools["get_order"].action, ToolPushAction::Unchanged);
    assert_eq!(
        parse(&reloaded.to_files()["tool_configs/get_order.json"])["future_field"],
        1
    );
}

#[tokio::test]
async fn sends_the_version_description_and_request_options() {
    let server = FakeServer::start().await;
    let client = server.client();
    let created = client
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap();
    server.clear_calls();
    client
        .agents_as_code()
        .push(
            &default_support_agent(),
            PushOptions {
                agent_id: Some(created.agent_id),
                version_description: Some("Tweak".into()),
                request_options: Some(
                    elevenlabs_rust_sdk::RequestOptions::new().additional_query_param("trace", "1"),
                ),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let calls = server.calls();
    assert!(calls
        .iter()
        .all(|c| c.query.get("trace").map(String::as_str) == Some("1")));
    let patch = calls.iter().find(|c| c.method == "PATCH").unwrap();
    assert_eq!(patch.body.as_ref().unwrap()["version_description"], "Tweak");
}

// ---- pull ------------------------------------------------------------------------------------

#[tokio::test]
async fn pull_loads_an_agent_with_its_tools_and_writes_them_into_a_project() {
    let server = FakeServer::start().await;
    let client = server.client();
    let created = client
        .agents_as_code()
        .push(&default_support_agent(), push_opts())
        .await
        .unwrap();
    let agent_id = created.agent_id;

    let mut project = AgentProject::from_files(Vec::<(String, String)>::new()).unwrap();
    let agent = client
        .agents_as_code()
        .pull(
            &agent_id,
            PullOptions {
                project: Some(&mut project),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(agent.name(), "Support line");
    let names: Vec<String> = agent.tools().iter().map(|t| t.name().to_string()).collect();
    assert_eq!(names, ["show_order", "get_order"]);
    assert_eq!(project.agent_id(&agent).unwrap(), Some(agent_id.as_str()));

    let files = project.to_files();
    assert_eq!(
        files.keys().collect::<Vec<_>>(),
        [
            "agent_configs/Support-line.json",
            "agents.json",
            "tool_configs/get_order.json",
            "tool_configs/show_order.json",
            "tools.json"
        ]
    );
    let config = parse(&files["agent_configs/Support-line.json"]);
    assert_eq!(
        config.as_object().unwrap().keys().collect::<Vec<_>>(),
        [
            "conversation_config",
            "name",
            "platform_settings",
            "tags",
            "workflow"
        ]
    );

    // Pushing the pulled agent back changes no tools.
    server.clear_calls();
    let result = client
        .agents_as_code()
        .push(&agent, with_project(&mut project))
        .await
        .unwrap();
    assert_eq!(
        server.writes(),
        [format!("PATCH /v1/convai/agents/{agent_id}")]
    );
    assert!(result
        .tools
        .values()
        .all(|t| t.action == ToolPushAction::Unchanged));
    assert_eq!(result.tools.len(), 2);
}

#[tokio::test]
async fn pull_without_a_project_keeps_other_tool_kinds_attached_by_id() {
    let server = FakeServer::start().await;
    server.add_tool(
        "tool_mcp",
        json!({ "type": "mcp", "name": "search", "description": "MCP search" }),
        &[],
    );
    server.add_tool("tool_show", show_order().to_json().unwrap(), &[]);
    server.add_agent(
        "agent_1",
        json!({
            "name": "A",
            "conversation_config": { "agent": { "prompt": { "prompt": "p", "tool_ids": ["tool_mcp", "tool_show"] } } },
            "phone_numbers": []
        }),
    );
    let agent = server
        .client()
        .agents_as_code()
        .pull("agent_1", PullOptions::default())
        .await
        .unwrap();
    let names: Vec<String> = agent.tools().iter().map(|t| t.name().to_string()).collect();
    assert_eq!(names, ["show_order"]);
    assert_eq!(agent.tool_ids(), ["tool_mcp"]);
    let config = agent
        .to_json(&HashMap::from([(
            "show_order".to_string(),
            "tool_show".to_string(),
        )]))
        .unwrap();
    assert!(config.get("phone_numbers").is_none());
    assert_eq!(
        config["conversation_config"]["agent"]["prompt"]["tool_ids"],
        json!(["tool_mcp", "tool_show"])
    );
}

fn mcp_tool_config() -> Value {
    json!({ "type": "mcp", "name": "docs_search", "description": "Search the docs" })
}

#[tokio::test]
async fn push_leaves_a_loaded_mcp_tool_object_attached_by_its_id() {
    let server = FakeServer::start().await;
    server.add_tool("tool_mcp", mcp_tool_config(), &["agent_1"]);
    let config = json!({
        "name": "A",
        "conversation_config": { "agent": { "prompt": { "prompt": "p", "tool_ids": ["tool_mcp"] } } }
    });
    server.add_agent("agent_1", config.clone());
    let agent = Agent::from_json_with_tools(
        &config,
        &HashMap::from([("tool_mcp".to_string(), Tool::from_json(&mcp_tool_config()))]),
    );
    assert!(agent.tools().is_empty());
    let result = server
        .client()
        .agents_as_code()
        .push(
            &agent,
            PushOptions {
                agent_id: Some("agent_1".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(result.tools.is_empty(), "{:?}", result.tools);
    assert_eq!(server.writes(), ["PATCH /v1/convai/agents/agent_1"]);
    assert_eq!(
        server.agent("agent_1")["conversation_config"]["agent"]["prompt"]["tool_ids"],
        json!(["tool_mcp"])
    );
}

#[test]
fn an_mcp_tool_object_without_an_id_is_an_invalid_tool() {
    let mcp = Tool::from_json(&mcp_tool_config());
    let agent = Agent::new(
        "A",
        AgentOptions::new().prompt("p").tools(vec![mcp.clone()]),
    );
    let err = agent.validate().unwrap_err();
    let issue = err
        .issues
        .iter()
        .find(|i| i.code == AgentIssueCode::InvalidTool)
        .unwrap_or_else(|| panic!("{err}"));
    assert_eq!(issue.path.as_deref(), Some("tools.docs_search"));
    assert_eq!(
        issue.message,
        "Tool \"docs_search\" has type \"mcp\", which push doesn't create or update: load it with its id or attach it with tool_ids"
    );

    let wf = Workflow::new(None);
    wf.converse("triage", "Find out what the caller needs", Options::new())
        .when("Caller asked about the docs".into(), None)
        .tool("search", vec![mcp.into()], Options::new());
    let agent = Agent::new("A", AgentOptions::new().prompt("p").workflow(wf));
    let err = agent.validate().unwrap_err();
    assert!(
        err.issues
            .iter()
            .any(|i| i.code == AgentIssueCode::InvalidTool
                && i.path.as_deref() == Some("tools.docs_search")),
        "{err}"
    );
}

#[tokio::test]
async fn rejects_a_branch_entry_without_a_config_before_any_write() {
    let server = FakeServer::start().await;
    let client = server.client();
    let mut files = read_tree(&fixture_dir());
    let mut index = parse(&files["agents.json"]);
    index["agents"][0]["branches"]["beta"] = json!({ "branch_id": "agtbrch_beta" });
    files.insert(
        "agents.json".to_string(),
        serde_json::to_string_pretty(&index).unwrap(),
    );
    let mut project = AgentProject::from_files(&files).unwrap();
    let main_before = project.to_files()["agent_configs/support.json"].clone();
    let support = project.agent("Support").unwrap().clone();
    let err = client
        .agents_as_code()
        .push(
            &support,
            PushOptions {
                project: Some(&mut project),
                branch_id: Some("agtbrch_beta".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(matches!(err, PushError::Project(_)));
    assert!(err.to_string().contains("has no config file"), "{err}");
    let err = client
        .agents_as_code()
        .pull(
            "agent_support",
            PullOptions {
                project: Some(&mut project),
                branch_id: Some("agtbrch_beta".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(err.to_string().contains("has no config file"), "{err}");
    assert!(server.calls().is_empty());
    assert!(project.changed_files().is_empty());
    assert_eq!(
        project.to_files()["agent_configs/support.json"],
        main_before
    );
    assert_eq!(main_before, files["agent_configs/support.json"]);
}

#[tokio::test]
async fn rejects_a_branch_the_project_has_no_file_for_before_any_request() {
    let server = FakeServer::start().await;
    let client = server.client();
    let mut project = AgentProject::from_files(&read_tree(&fixture_dir())).unwrap();
    let billing = project.agent("Billing").unwrap().clone();
    let err = client
        .agents_as_code()
        .push(
            &billing,
            PushOptions {
                project: Some(&mut project),
                branch_id: Some("agtbrch_x".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(matches!(err, PushError::Project(_)));
    assert!(err.to_string().contains("has no config file"), "{err}");
    let err = client
        .agents_as_code()
        .pull(
            "agent_support",
            PullOptions {
                project: Some(&mut project),
                branch_id: Some("agtbrch_x".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(err.to_string().contains("has no config file"), "{err}");
    assert!(server.calls().is_empty());
}

#[tokio::test]
async fn a_branch_push_writes_the_branch_file_and_leaves_the_main_one_alone() {
    let server = FakeServer::start().await;
    let mut project = AgentProject::from_files(&read_tree(&fixture_dir())).unwrap();
    for (id, file) in [
        ("tool_lookup", "lookup_order.json"),
        ("tool_refund", "issue_refund.json"),
    ] {
        let config =
            parse(&std::fs::read_to_string(fixture_dir().join("tool_configs").join(file)).unwrap());
        server.add_tool(id, config, &[]);
    }
    let beta_config = parse(
        &std::fs::read_to_string(fixture_dir().join("agent_configs/support.beta.json")).unwrap(),
    );
    server.set_branch("agent_support/agtbrch_beta", beta_config.clone());
    let refund = project.tools()[1].clone();
    let beta = Agent::from_json_with_tools(
        &beta_config,
        &HashMap::from([("tool_refund".to_string(), refund)]),
    );
    let result = server
        .client()
        .agents_as_code()
        .push(
            &beta,
            PushOptions {
                project: Some(&mut project),
                branch_id: Some("agtbrch_beta".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(result.action, AgentPushAction::Updated);
    assert_eq!(result.agent_id, "agent_support");
    assert_eq!(
        result.tools["issue_refund"].action,
        ToolPushAction::Unchanged
    );
    let changed = project.changed_files();
    assert!(changed.contains(&"agents.json".to_string()), "{changed:?}");
    assert!(!changed.contains(&"agent_configs/support.json".to_string()));
    let index = parse(&project.to_files()["agents.json"]);
    assert_eq!(
        index["agents"][0]["branches"]["beta"]["version_id"],
        "ver_1"
    );
    assert!(index["agents"][0].get("version_id").is_none());
    assert_eq!(
        project.agent("Support").unwrap().tool_ids(),
        ["tool_from_dashboard"]
    );
}

// ---- projects on disk ------------------------------------------------------------------------

#[tokio::test]
async fn opens_an_empty_directory_and_push_writes_the_cli_layout_owner_only() {
    let dir = temp_dir("agents");
    std::fs::create_dir_all(dir.join("agent_configs")).unwrap();
    // A file the project doesn't list.
    std::fs::write(dir.join("agent_configs/A.json"), "not ours").unwrap();
    let mut project = AgentProject::open(&dir).unwrap();
    let server = FakeServer::start().await;
    let agent = Agent::new(
        "A",
        AgentOptions::new()
            .prompt("p")
            .tools(vec![Tool::client("ping", "Ping the app").into()]),
    );

    server
        .client()
        .agents_as_code()
        .push(&agent, with_project(&mut project))
        .await
        .unwrap();

    let files = read_tree(&dir);
    assert_eq!(
        files.keys().collect::<Vec<_>>(),
        [
            "agent_configs/A-1.json",
            "agent_configs/A.json",
            "agents.json",
            "tool_configs/ping.json",
            "tools.json"
        ]
    );
    assert_eq!(files["agent_configs/A.json"], "not ours");
    assert_eq!(
        files["tool_configs/ping.json"],
        "{\n    \"description\": \"Ping the app\",\n    \"name\": \"ping\",\n    \"type\": \"client\"\n}"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for path in ["agents.json", "tools.json", "agent_configs/A-1.json"] {
            let mode = std::fs::metadata(dir.join(path))
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600, "{path}");
        }
    }
    assert!(project.changed_files().is_empty());

    let reopened = AgentProject::open(&dir).unwrap();
    assert_eq!(
        reopened.agent_id(reopened.agent("A").unwrap()).unwrap(),
        project.agent_id(&agent).unwrap()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn refuses_config_paths_outside_the_project() {
    let escaping = temp_dir("agents-escaping");
    std::fs::write(
        escaping.join("agents.json"),
        json!({ "agents": [{ "config": "../a.json" }] }).to_string(),
    )
    .unwrap();
    let err = AgentProject::open(&escaping).unwrap_err();
    assert!(err.message.contains("outside the project"), "{err}");
    let _ = std::fs::remove_dir_all(&escaping);
}

#[cfg(unix)]
#[test]
fn refuses_config_paths_that_lead_outside_through_a_symlink() {
    let outside = temp_dir("agents-outside");
    std::fs::write(outside.join("a.json"), "{}").unwrap();
    let linked = temp_dir("agents-linked");
    std::os::unix::fs::symlink(&outside, linked.join("agent_configs")).unwrap();
    std::fs::write(
        linked.join("agents.json"),
        json!({ "agents": [{ "config": "agent_configs/a.json" }] }).to_string(),
    )
    .unwrap();
    let err = AgentProject::open(&linked).unwrap_err();
    assert!(err.message.contains("through a symlink"), "{err}");
    let _ = std::fs::remove_dir_all(&outside);
    let _ = std::fs::remove_dir_all(&linked);
}

#[cfg(unix)]
#[tokio::test]
async fn never_writes_through_a_symlinked_file() {
    let dir = temp_dir("agents-symlink-file");
    let outside = temp_dir("agents-symlink-target");
    std::fs::create_dir_all(dir.join("tool_configs")).unwrap();
    std::fs::write(outside.join("victim.json"), "untouched").unwrap();
    let files = BTreeMap::from([
        (
            "tools.json",
            json!({ "tools": [{ "type": "client", "config": "tool_configs/ping.json", "id": "tool_1" }] })
                .to_string(),
        ),
        (
            "tool_configs/ping.json",
            json!({ "type": "client", "name": "ping", "description": "old" }).to_string(),
        ),
    ]);
    for (path, text) in &files {
        std::fs::write(dir.join(path), text).unwrap();
    }
    let mut project = AgentProject::open(&dir).unwrap();
    // Swap the config for a symlink to a file in the project after opening it.
    std::fs::remove_file(dir.join("tool_configs/ping.json")).unwrap();
    std::fs::create_dir_all(dir.join("inside")).unwrap();
    std::fs::write(dir.join("inside/target.json"), "inside").unwrap();
    std::os::unix::fs::symlink(
        dir.join("inside/target.json"),
        dir.join("tool_configs/ping.json"),
    )
    .unwrap();
    let server = FakeServer::start().await;
    server.add_tool(
        "tool_1",
        json!({ "type": "client", "name": "ping", "description": "old" }),
        &[],
    );
    let agent = Agent::new(
        "A",
        AgentOptions::new()
            .prompt("p")
            .tools(vec![Tool::client("ping", "new").into()]),
    );
    let err = server
        .client()
        .agents_as_code()
        .push(&agent, with_project(&mut project))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("it's a symlink"), "{err}");
    assert_eq!(
        std::fs::read_to_string(dir.join("inside/target.json")).unwrap(),
        "inside"
    );
    assert_eq!(
        std::fs::read_to_string(outside.join("victim.json")).unwrap(),
        "untouched"
    );
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&outside);
}
