//! Unit tests for the agent builder and the project format, after the TS/Python suites.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;

use serde_json::{json, Value};

use super::wire::format_config;
use super::*;
use crate::agent_tools::json::js_canonical;
use crate::agent_tools::{Schema, SystemTool, Tool};
use crate::agent_workflows::{Chain, Options, Workflow};
use crate::api::types::{
    AgentConfig, AgentMessagesOverridesLlm as Llm, AgentPlatformSettingsRequestModel,
    ConversationalConfig, PrivacyConfigInput, PromptAgentApiModelInput, TtsConversationalModel,
    TurnConfig,
};

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/agent_builder/project/cli-graph")
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(fixture_dir().join(path)).unwrap()).unwrap()
}

fn get_order() -> Tool {
    Tool::webhook("get_order", "Look up an order by its number")
        .get("https://api.example.com/orders/{order_id}")
        .path_param("order_id", Schema::string().describe("The order number"))
        .into()
}

fn show_order() -> Tool {
    Tool::client("show_order", "Show the order card")
        .param("order_id", Schema::string().from_variable("order_id"))
        .into()
}

fn ids(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn issues_of(
    result: Result<impl std::fmt::Debug, AgentValidationError>,
) -> Vec<(String, Option<String>)> {
    result
        .unwrap_err()
        .issues
        .into_iter()
        .map(|i| (i.code.as_str().to_string(), i.path))
        .collect()
}

fn issue(code: &str, path: Option<&str>) -> (String, Option<String>) {
    (code.to_string(), path.map(str::to_string))
}

#[test]
fn maps_typed_options_tools_and_system_tools_into_the_api_config() {
    let agent = Agent::new(
        "Support line",
        AgentOptions::new()
            .prompt("You are a support agent.")
            .first_message("Hi!")
            .language("en")
            .llm(Llm::Gemini2_0Flash)
            .temperature(0.5)
            .voice_id("voice_1")
            .tts_model(TtsConversationalModel::ElevenFlashV2_5)
            .tags(["support"])
            .tools(vec![SystemTool::end_call().into(), get_order()])
            .tool_ids(["tool_existing"]),
    );
    let config = agent.to_json(&ids(&[("get_order", "tool_1")])).unwrap();
    assert_eq!(
        config,
        json!({
            "conversation_config": {
                "agent": {
                    "first_message": "Hi!",
                    "language": "en",
                    "prompt": {
                        "built_in_tools": {
                            "end_call": { "name": "end_call", "params": { "system_tool_type": "end_call" }, "type": "system" }
                        },
                        "llm": "gemini-2.0-flash",
                        "prompt": "You are a support agent.",
                        "temperature": 0.5,
                        "tool_ids": ["tool_existing", "tool_1"]
                    }
                },
                "tts": { "model_id": "eleven_flash_v2_5", "voice_id": "voice_1" }
            },
            "name": "Support line",
            "platform_settings": {},
            "tags": ["support"]
        })
    );
    // Sorted at every level.
    assert_eq!(config.to_string(), js_canonical(&config).to_string());

    let body = agent.build(&ids(&[("get_order", "tool_1")])).unwrap();
    assert_eq!(body.name.as_deref(), Some("Support line"));
    let prompt = body.conversation_config.agent.unwrap().prompt.unwrap();
    assert_eq!(
        prompt.tool_ids.unwrap(),
        vec!["tool_existing".to_string(), "tool_1".to_string()]
    );
}

#[test]
fn merges_the_raw_config_under_the_typed_options() {
    let mut agent = Agent::new(
        "A",
        AgentOptions::new()
            .prompt("p")
            .conversation_config(ConversationalConfig {
                turn: Some(TurnConfig {
                    turn_timeout: Some(7.0),
                    ..Default::default()
                }),
                agent: Some(AgentConfig {
                    disable_first_message_interruptions: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .platform_settings(AgentPlatformSettingsRequestModel {
                privacy: Some(PrivacyConfigInput {
                    record_voice: Some(false),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );
    agent.update(
        AgentOptions::new().conversation_config(ConversationalConfig {
            turn: Some(TurnConfig {
                silence_end_call_timeout: Some(20.0),
                ..Default::default()
            }),
            ..Default::default()
        }),
    );
    let config = agent.to_json(&HashMap::new()).unwrap();
    assert_eq!(
        config["conversation_config"],
        json!({
            "agent": { "disable_first_message_interruptions": true, "prompt": { "prompt": "p" } },
            "turn": { "silence_end_call_timeout": 20, "turn_timeout": 7 }
        })
    );
    assert_eq!(
        config["platform_settings"],
        json!({ "privacy": { "record_voice": false } })
    );
}

#[test]
fn fills_in_the_ids_of_tool_objects_used_by_workflow_nodes() {
    let lookup = get_order();
    let show = show_order();
    let wf = Workflow::new(None);
    let triage = wf.converse("triage", "Find out what the caller needs", Options::new());
    triage
        .when("Caller gave an order number".into(), None)
        .tool(
            "lookup",
            vec![lookup.clone().into()],
            Options::new().set(
                "schema_overrides",
                json!({ "get_order": { "path_params.order_id": { "source": "constant", "constant_value": "1" } } }),
            ),
        )
        .on_success(None)
        .converse(
            "resolve",
            "Explain the order",
            Options::new().tool_ids(vec![show.clone().into(), "tool_raw".into()]),
        );
    let agent = Agent::new("A", AgentOptions::new().prompt("p").workflow(wf));

    assert_eq!(agent.tools(), vec![lookup, show]);
    let config = agent
        .to_json(&ids(&[("get_order", "tool_g"), ("show_order", "tool_s")]))
        .unwrap();
    let nodes = &config["workflow"]["nodes"];
    assert_eq!(
        nodes["lookup"]["tools"],
        json!([{
            "schema_overrides": { "path_params.order_id": { "constant_value": "1", "source": "constant" } },
            "tool_id": "tool_g"
        }])
    );
    assert_eq!(
        nodes["resolve"]["additional_tool_ids"],
        json!(["tool_s", "tool_raw"])
    );
    assert_eq!(
        issues_of(agent.to_json(&ids(&[("get_order", "tool_g")]))),
        vec![issue("unresolved_tool", Some("tools.show_order"))]
    );
}

#[test]
fn validates_without_ids() {
    Agent::new(
        "A",
        AgentOptions::new().prompt("p").tools(vec![get_order()]),
    )
    .validate()
    .unwrap();
}

#[test]
fn reports_every_problem_at_once() {
    let wf = Workflow::new(None);
    wf.tool(
        "hang_up",
        vec![SystemTool::end_call().into()],
        Options::new(),
    );
    let other_get_order: Tool = Tool::webhook("get_order", "Another lookup")
        .get("https://api.example.com/orders/{order_id}")
        .path_param("order_id", Schema::string().describe("The order number"))
        .into();
    let agent = Agent::new(
        "",
        AgentOptions::new()
            .prompt("p")
            .tools(vec![
                get_order(),
                other_get_order,
                Tool::webhook("broken", "No URL").into(),
            ])
            .workflow(wf)
            .conversation_config(ConversationalConfig {
                agent: Some(AgentConfig {
                    prompt: Some(PromptAgentApiModelInput {
                        prompt: Some("raw prompt".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );
    assert_eq!(
        issues_of(agent.validate()),
        vec![
            issue("missing_name", None),
            issue(
                "option_conflict",
                Some("conversation_config.agent.prompt.prompt")
            ),
            issue("duplicate_tool_name", Some("tools.get_order")),
            issue("invalid_tool", Some("tools.broken")),
            issue("system_tool_in_workflow", Some("workflow.tools.end_call")),
        ]
    );
}

#[test]
fn the_same_tool_twice_is_not_a_duplicate() {
    let agent = Agent::new(
        "A",
        AgentOptions::new()
            .prompt("p")
            .tools(vec![get_order(), get_order()]),
    );
    agent.validate().unwrap();
    assert_eq!(agent.tools(), vec![get_order()]);
}

#[test]
fn warns_without_failing_when_there_is_no_prompt() {
    let mut warnings = Vec::new();
    Agent::new("A", AgentOptions::new())
        .validate_with_warnings(|issue| warnings.push(issue.code.as_str()))
        .unwrap();
    assert_eq!(warnings, vec!["missing_prompt"]);
}

#[test]
fn round_trips_a_cli_agent_config_keeping_unmodeled_fields_and_tool_order() {
    let raw = read_json("agent_configs/support.json");
    let lookup = Tool::from_json(&read_json("tool_configs/lookup_order.json"));
    let by_id = HashMap::from([("tool_lookup".to_string(), lookup.clone())]);
    let agent = Agent::from_json_with_tools(&raw, &by_id);

    assert_eq!(agent.name(), "Support");
    assert_eq!(agent.tools(), vec![lookup]);
    assert_eq!(agent.tool_ids(), ["tool_from_dashboard"]);
    let out = agent
        .to_json(&ids(&[("lookup_order", "tool_lookup")]))
        .unwrap();
    assert_eq!(
        format_config(&out),
        format_config(&raw),
        "round-trips to the sorted form"
    );
    assert_eq!(out.to_string(), js_canonical(&raw).to_string());
}

#[test]
fn turns_a_loaded_workflow_into_a_workflow_on_first_use() {
    let raw = read_json("agent_configs/support.beta.json");
    let refund = Tool::from_json(&read_json("tool_configs/issue_refund.json"));
    let by_id = HashMap::from([("tool_refund".to_string(), refund.clone())]);
    let agent = Agent::from_json_with_tools(&raw, &by_id);

    // Read from the raw workflow, before it's materialized.
    assert_eq!(agent.tools(), vec![refund]);
    let unedited = agent
        .to_json(&ids(&[("issue_refund", "tool_refund")]))
        .unwrap();
    assert_eq!(
        unedited["workflow"].to_string(),
        js_canonical(&raw["workflow"]).to_string()
    );

    agent
        .workflow()
        .unwrap()
        .node("refunds")
        .update(Options::new().set("prompt", json!("Handle refunds")));
    let nodes = agent
        .to_json(&ids(&[("issue_refund", "tool_new")]))
        .unwrap()["workflow"]["nodes"]
        .clone();
    assert_eq!(nodes["refunds"]["additional_tool_ids"], json!(["tool_new"]));
    assert_eq!(
        nodes["refunds"]["additional_prompt"],
        json!("Handle refunds")
    );
}

#[test]
fn replaces_a_loaded_agents_system_tools_along_with_its_other_tools() {
    let mut agent = Agent::from_json(&read_json("agent_configs/support.json"));
    assert!(agent.tools().is_empty()); // system tools aren't managed by push
    agent.update(AgentOptions::new().tools(vec![]));
    let config = agent.to_json(&HashMap::new()).unwrap();
    let prompt = &config["conversation_config"]["agent"]["prompt"];
    assert!(prompt.get("built_in_tools").is_none());
    // Ids it doesn't manage stay.
    assert_eq!(
        prompt["tool_ids"],
        json!(["tool_lookup", "tool_from_dashboard"])
    );
}

#[test]
fn treats_an_explicitly_empty_tool_list_as_set() {
    let agent = Agent::new(
        "A",
        AgentOptions::new()
            .prompt("p")
            .tool_ids(Vec::<String>::new())
            .conversation_config(ConversationalConfig {
                agent: Some(AgentConfig {
                    prompt: Some(PromptAgentApiModelInput {
                        tool_ids: Some(vec!["tool_existing".into()]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );
    assert_eq!(
        issues_of(agent.validate()),
        vec![issue(
            "option_conflict",
            Some("conversation_config.agent.prompt.tool_ids")
        )]
    );
}

#[test]
fn validates_a_loaded_workflow_without_materializing_it() {
    let mut raw = read_json("agent_configs/support.beta.json");
    raw["workflow"]["edges"]["broken"] = json!({ "source": "start_node", "target": "nowhere" });
    let agent = Agent::from_json(&raw);
    assert_eq!(
        issues_of(agent.validate()),
        vec![issue("invalid_workflow", Some("workflow"))]
    );
}

#[test]
fn writes_new_tool_ids_into_a_loaded_workflow_it_has_not_materialized() {
    let raw = read_json("agent_configs/support.beta.json");
    let refund = Tool::from_json(&read_json("tool_configs/issue_refund.json"));
    let agent =
        Agent::from_json_with_tools(&raw, &HashMap::from([("tool_refund".to_string(), refund)]));
    let config = agent
        .to_json(&ids(&[("issue_refund", "tool_clone")]))
        .unwrap();
    assert_eq!(
        config["workflow"]["nodes"]["refunds"]["additional_tool_ids"],
        json!(["tool_clone"])
    );
}

#[test]
fn attaches_a_tool_added_with_add_tool() {
    let mut agent = Agent::new("A", AgentOptions::new().prompt("p"));
    agent.add_tool(get_order());
    let config = agent.to_json(&ids(&[("get_order", "tool_1")])).unwrap();
    assert_eq!(
        config["conversation_config"]["agent"]["prompt"]["tool_ids"],
        json!(["tool_1"])
    );
}

#[test]
fn a_clone_has_its_own_workflow() {
    let wf = Workflow::new(None);
    wf.converse("a", "A", Options::new());
    let agent = Agent::new("A", AgentOptions::new().prompt("p").workflow(wf));
    let copy = agent.clone();
    copy.workflow().unwrap().remove("a");
    assert_eq!(agent.workflow().unwrap().node_ids(), ["start_node", "a"]);
    assert_eq!(copy.workflow().unwrap().node_ids(), ["start_node"]);
}

// ---- workflow tool refs ---------------------------------------------------------------------

#[test]
fn a_removed_nodes_tool_name_can_be_reused_by_a_new_tool() {
    let wf = Workflow::new(None);
    wf.tool("a", vec![get_order().into()], Options::new());
    wf.remove("a");
    let changed: Tool = Tool::webhook("get_order", "Changed")
        .get("https://api.example.com/orders/{order_id}")
        .path_param("order_id", Schema::string().describe("The order number"))
        .into();
    wf.tool("b", vec![changed.into()], Options::new());
    wf.build_with_tool_ids(&ids(&[("get_order", "tool_1")]))
        .unwrap();
}

#[test]
fn two_different_tools_with_one_name_in_a_node_are_reported() {
    let changed: Tool = Tool::webhook("get_order", "Changed")
        .get("https://api.example.com/orders/{order_id}")
        .path_param("order_id", Schema::string().describe("The order number"))
        .into();
    let wf = Workflow::new(None);
    wf.tool(
        "lookup",
        vec![get_order().into(), changed.into()],
        Options::new(),
    );
    let err = wf
        .build_with_tool_ids(&ids(&[("get_order", "tool_1")]))
        .unwrap_err();
    assert_eq!(err.codes(), vec!["duplicate_tool_name"]);

    let same = Workflow::new(None);
    same.tool(
        "lookup",
        vec![get_order().into(), get_order().into()],
        Options::new(),
    );
    same.build_with_tool_ids(&ids(&[("get_order", "tool_1")]))
        .unwrap();
}

#[test]
fn writes_tool_objects_as_ids_and_reports_missing_ones() {
    let wf = Workflow::new(None);
    wf.tool("lookup", vec![get_order().into()], Options::new());
    let tool_ids = ids(&[("get_order", "tool_1")]);
    let json = wf.to_json_with_tool_ids(&tool_ids).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&wf.to_json_string_with_tool_ids(&tool_ids).unwrap())
            .unwrap(),
        json
    );
    assert_eq!(
        json["nodes"]["lookup"]["tools"],
        json!([{ "tool_id": "tool_1" }])
    );
    assert_eq!(wf.tools(), vec![get_order()]);
    assert_eq!(wf.build().unwrap_err().codes(), vec!["unresolved_tool"]);
}

#[test]
fn schema_overrides_for_a_tool_it_does_not_run_name_the_tool() {
    let wf = Workflow::new(None);
    wf.tool(
        "lookup",
        vec![get_order().into()],
        Options::new().set("schema_overrides", json!({ "other": {} })),
    );
    let err = wf
        .build_with_tool_ids(&ids(&[("get_order", "tool_1")]))
        .unwrap_err();
    assert_eq!(err.codes(), vec!["invalid_option"]);
    assert!(err.issues[0].message.contains("tool \"other\""));
}

#[test]
fn loads_a_workflow_with_tool_objects_bound_by_id() {
    let raw = read_json("agent_configs/support.beta.json");
    let refund = Tool::from_json(&read_json("tool_configs/issue_refund.json"));
    let wf = Workflow::from_json_with_tools(
        &raw,
        &HashMap::from([("tool_refund".to_string(), refund.clone())]),
    );
    assert_eq!(wf.tools(), vec![refund]);
    let out = wf
        .to_json_with_tool_ids(&ids(&[("issue_refund", "tool_x")]))
        .unwrap();
    assert_eq!(
        out["nodes"]["refunds"]["additional_tool_ids"],
        json!(["tool_x"])
    );
}

// ---- project ------------------------------------------------------------------------------

fn read_tree() -> BTreeMap<String, String> {
    let root = fixture_dir();
    let mut files = BTreeMap::new();
    fn walk(root: &std::path::Path, dir: &std::path::Path, files: &mut BTreeMap<String, String>) {
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
    walk(&root, &root, &mut files);
    files
}

#[test]
fn loads_a_cli_project_tools_by_id_agents_with_tool_objects_unknown_ids_kept() {
    let project = AgentProject::from_files(&read_tree()).unwrap();
    let names: Vec<&str> = project.agents().iter().map(|a| a.name()).collect();
    assert_eq!(names, ["Support", "Billing"]);
    let tools: Vec<&str> = project.tools().iter().map(|t| t.name()).collect();
    assert_eq!(tools, ["lookup_order", "issue_refund"]);

    let support = project.agent("Support").unwrap();
    let support_tools: Vec<String> = support
        .tools()
        .iter()
        .map(|t| t.name().to_string())
        .collect();
    assert_eq!(support_tools, ["lookup_order"]);
    assert_eq!(support.tool_ids(), ["tool_from_dashboard"]);
    assert_eq!(project.agent_id(support).unwrap(), Some("agent_support"));
    assert_eq!(
        project.tool_id(&support.tools()[0]).unwrap(),
        Some("tool_lookup")
    );
    let billing = project.agent("Billing").unwrap();
    assert_eq!(project.agent_id(billing).unwrap(), Some("agent_billing"));
    assert_eq!(
        billing.tools().iter().map(|t| t.name()).collect::<Vec<_>>(),
        ["issue_refund"]
    );
    // A clone still matches its entry.
    assert_eq!(
        project.agent_id(&support.clone()).unwrap(),
        Some("agent_support")
    );
}

#[test]
fn gives_back_every_file_unchanged_when_nothing_was_pushed() {
    let files = read_tree();
    assert_eq!(AgentProject::from_files(&files).unwrap().to_files(), files);
}

#[test]
fn reports_missing_malformed_and_unsupported_files_clearly() {
    let index = |config: &str| json!({ "agents": [{ "config": config }] }).to_string();
    let err =
        AgentProject::from_files([("agents.json", index("agent_configs/a.yaml"))]).unwrap_err();
    assert!(
        err.message
            .contains("only JSON config files are supported yet"),
        "{err}"
    );
    let err =
        AgentProject::from_files([("agents.json", index("agent_configs/a.toml"))]).unwrap_err();
    assert!(
        err.message
            .contains("only JSON config files are supported yet"),
        "{err}"
    );
    let err =
        AgentProject::from_files([("agents.json", index("agent_configs/a.json"))]).unwrap_err();
    assert!(
        err.message
            .contains("points at agent_configs/a.json, which doesn't exist"),
        "{err}"
    );
    assert!(AgentProject::from_files([("agents.json", "{")]).is_err());
    let empty = AgentProject::from_files(Vec::<(String, String)>::new()).unwrap();
    assert!(empty
        .agent("Nope")
        .unwrap_err()
        .message
        .contains("No agent named \"Nope\""));
}

#[test]
fn names_new_files_like_the_cli() {
    assert_eq!(sanitize_filename("  Support line / EU "), "Support-line-EU");
    assert_eq!(sanitize_filename(".hidden"), "_hidden");
    assert_eq!(sanitize_filename("..."), "unnamed");
    assert_eq!(sanitize_filename(""), "unnamed");
    assert_eq!(sanitize_filename(&"x".repeat(150)).len(), 100);
    let taken: HashSet<String> = ["agent_configs/A.json", "agent_configs/A-1.json"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        unique_path("agent_configs", "A", &taken),
        "agent_configs/A-2.json"
    );
}

#[test]
fn keeps_the_main_agent_when_a_branch_is_recorded_and_the_tool_type_in_step() {
    let mut project = AgentProject::from_files(&read_tree()).unwrap();
    let main_prompt = project.agent("Support").unwrap().tool_ids().to_vec();
    let beta = Agent::from_json(&read_json("agent_configs/support.beta.json"));
    let config = beta.to_json(&HashMap::new()).unwrap();
    project
        .record_agent(
            &beta,
            "agent_support",
            &config,
            Some("agtbrch_beta"),
            None,
            project::MatchBy::Id,
        )
        .unwrap();
    assert_eq!(project.agent("Support").unwrap().tool_ids(), main_prompt);
    let changed = project.changed_files();
    assert!(changed.contains(&"agent_configs/support.beta.json".to_string()));
    assert!(!changed.contains(&"agent_configs/support.json".to_string()));

    let lookup_id = project
        .tool_id(project.tools()[0])
        .unwrap()
        .unwrap()
        .to_string();
    let client: Tool = Tool::client("lookup_order", "Now a client tool").into();
    project
        .record_tool(
            &client,
            &lookup_id,
            &client.to_json().unwrap(),
            project::MatchBy::Id,
        )
        .unwrap();
    let tools: Value = serde_json::from_str(&project.to_files()["tools.json"]).unwrap();
    assert_eq!(tools["tools"][0]["type"], json!("client"));
}

#[test]
fn drops_a_pending_write_once_a_file_is_back_to_what_was_loaded() {
    let loaded =
        json!({ "name": "A", "conversation_config": {}, "platform_settings": {}, "tags": [] });
    let files = BTreeMap::from([
        (
            "agents.json".to_string(),
            super::wire::format_index(
                &json!({ "agents": [{ "config": "agent_configs/a.json", "id": "agent_1" }] }),
            ),
        ),
        ("agent_configs/a.json".to_string(), format_config(&loaded)),
    ]);
    let mut project = AgentProject::from_files(&files).unwrap();
    let agent = project.agent("A").unwrap().clone();
    let mut edited = loaded.clone();
    edited["tags"] = json!(["x"]);
    project
        .record_agent(
            &agent,
            "agent_1",
            &edited,
            None,
            None,
            project::MatchBy::Definition,
        )
        .unwrap();
    assert_eq!(project.changed_files(), ["agent_configs/a.json"]);
    project
        .record_agent(
            &agent,
            "agent_1",
            &loaded,
            None,
            None,
            project::MatchBy::Definition,
        )
        .unwrap();
    assert!(project.changed_files().is_empty());
    let mut expected = files.clone();
    expected.insert("tools.json".into(), "{\n    \"tools\": []\n}".into());
    assert_eq!(project.to_files(), expected);
}

#[test]
fn two_entries_with_one_name_are_an_error_unless_the_agent_came_from_one() {
    let config = format_config(&json!({ "name": "A", "conversation_config": {} }));
    let files = BTreeMap::from([
        (
            "agents.json".to_string(),
            json!({ "agents": [
                { "config": "agent_configs/a.json", "id": "agent_1" },
                { "config": "agent_configs/b.json", "id": "agent_2" }
            ] })
            .to_string(),
        ),
        ("agent_configs/a.json".to_string(), config.clone()),
        ("agent_configs/b.json".to_string(), config),
    ]);
    let project = AgentProject::from_files(&files).unwrap();
    let fresh = Agent::new("A", AgentOptions::new());
    assert!(project.agent_id(&fresh).is_err());
    assert_eq!(
        project.agent_id(project.agents()[1]).unwrap(),
        Some("agent_2")
    );
}
