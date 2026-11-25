//! Unit tests for the tool builder. Mirrors the TypeScript SDK's `tools.test.ts`.

use serde_json::{json, Value};

use super::*;
use crate::api::types::{BuiltInToolsInput, ToolRequestModelToolConfig, ToolResponseModel};

fn get_order() -> WebhookTool {
    Tool::webhook("get_order", "Look up an order")
        .get("https://api.example.com/orders/{order_id}")
        .path_param("order_id", Schema::string().describe("The order number"))
}

fn issues_of<T: std::fmt::Debug>(result: Result<T, ToolValidationError>) -> Vec<ToolIssue> {
    result.expect_err("expected a ToolValidationError").issues
}

fn codes_of<T: std::fmt::Debug>(result: Result<T, ToolValidationError>) -> Vec<&'static str> {
    issues_of(result).iter().map(|i| i.code.as_str()).collect()
}

fn paths_of<T: std::fmt::Debug>(
    result: Result<T, ToolValidationError>,
) -> Vec<(&'static str, Option<String>)> {
    issues_of(result)
        .into_iter()
        .map(|i| (i.code.as_str(), i.path))
        .collect()
}

fn api_schema(tool: &WebhookTool) -> Value {
    tool.to_json().unwrap()["api_schema"].clone()
}

fn timeout(secs: i64) -> ToolOptions {
    ToolOptions::new().set("response_timeout_secs", secs)
}

// ---- Schema ------------------------------------------------------------------------------------

#[test]
fn schema_can_be_shared_and_editing_one_copy_leaves_the_others_alone() {
    let order_id = Schema::string().describe("The order number");
    let lookup = Tool::webhook("lookup", "Look up")
        .get("https://api.example.com/{id}")
        .path_param("id", order_id.clone());
    let cancel = Tool::webhook("cancel", "Cancel")
        .post("https://api.example.com/cancel")
        .body(Schema::object().property("id", order_id.clone()));

    let body = cancel
        .body_schema()
        .and_then(Schema::as_object)
        .unwrap()
        .clone();
    let cancel = cancel.body(body.property("id", order_id.optional()));

    assert_eq!(
        api_schema(&lookup)["path_params_schema"],
        json!({ "id": { "description": "The order number", "type": "string" } })
    );
    assert_eq!(
        api_schema(&cancel)["request_body_schema"],
        json!({
            "properties": { "id": { "description": "The order number", "type": "string" } },
            "type": "object"
        })
    );
}

#[test]
fn schema_loads_properties_not_in_required_as_optional() {
    let schema = Schema::from_json(&json!({
        "properties": { "a": { "type": "string" }, "b": { "type": "string" } },
        "required": ["a"],
        "type": "object"
    }));
    assert_eq!(schema.kind(), SchemaKind::Object);
    let object = schema.as_object().unwrap();
    assert!(!object.property_schema("a").unwrap().is_optional());
    assert!(object.property_schema("b").unwrap().is_optional());
}

#[test]
fn schema_kind_specific_constructors_and_reads() {
    let tags = Schema::array(Schema::string().describe("A tag"));
    assert_eq!(tags.kind(), SchemaKind::Array);
    assert_eq!(tags.items().unwrap().kind(), SchemaKind::String);
    assert_eq!(Schema::bool().to_json(), json!({ "type": "boolean" }));
    assert_eq!(
        Schema::object()
            .property("a", Schema::integer().constant(1))
            .property("b", Schema::number().constant(1.5))
            .property_names(),
        ["a", "b"]
    );
}

// ---- WebhookTool -------------------------------------------------------------------------------

#[test]
fn webhook_builds_the_sdk_request_model() {
    let request = get_order()
        .header("Authorization", Header::secret("sec_1"))
        .options(timeout(10))
        .build()
        .unwrap();
    assert!(request.response_mocks.is_none());
    let config = request
        .tool_config
        .as_webhook_tool_config_input()
        .expect("a webhook config");
    assert_eq!(config.name, "get_order");
    assert_eq!(config.description, "Look up an order");
    assert_eq!(config.response_timeout_secs, Some(10));
    assert_eq!(
        config.api_schema.url,
        "https://api.example.com/orders/{order_id}"
    );
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "tool_config": {
                "type": "webhook",
                "name": "get_order",
                "description": "Look up an order",
                "response_timeout_secs": 10,
                "api_schema": {
                    "method": "GET",
                    "url": "https://api.example.com/orders/{order_id}",
                    "path_params_schema": {
                        "order_id": { "type": "string", "description": "The order number" }
                    },
                    "request_headers": { "Authorization": { "secret_id": "sec_1" } }
                }
            }
        })
    );
}

#[test]
fn webhook_replaces_a_param_with_the_same_name_in_place() {
    let tool = Tool::webhook("search", "Search")
        .get("https://api.example.com/search")
        .query_param("q", Schema::string().describe("Query"))
        .query_param("limit", Schema::integer().constant(10))
        .query_param("q", Schema::string().describe("Search terms"));
    assert_eq!(
        api_schema(&tool)["query_params_schema"],
        json!({
            "properties": {
                "limit": { "constant_value": 10, "type": "integer" },
                "q": { "description": "Search terms", "type": "string" }
            },
            "required": ["q", "limit"]
        })
    );
}

#[test]
fn webhook_removes_params_headers_and_assignments() {
    let tool = get_order()
        .query_param("q", Schema::string().describe("Query"))
        .header("X-A", Header::value("1"))
        .assign("a", "a", Default::default())
        .assign("b", "b", Default::default())
        .remove_query_param("q")
        .remove_header("X-A")
        .remove_assignment("a");
    let json = tool.to_json().unwrap();
    assert!(json["api_schema"].get("query_params_schema").is_none());
    assert!(json["api_schema"].get("request_headers").is_none());
    assert_eq!(
        json["assignments"],
        json!([{ "dynamic_variable": "b", "source": "response", "value_path": "b" }])
    );
}

#[test]
fn webhook_removing_what_does_not_exist_does_nothing() {
    let tool = get_order()
        .remove_assignment("missing")
        .remove_header("missing")
        .remove_query_param("missing")
        .remove_path_param("missing");
    assert_eq!(tool.to_json().unwrap(), get_order().to_json().unwrap());
}

#[test]
fn webhook_exposes_the_body_schema_for_editing() {
    let tool = Tool::webhook("refund", "Refund")
        .post("https://api.example.com/refunds")
        .body(Schema::object().property("a", Schema::string().describe("A")));
    let body = tool
        .body_schema()
        .and_then(Schema::as_object)
        .unwrap()
        .clone();
    let tool = tool.body(
        body.without("a")
            .property("b", Schema::string().describe("B")),
    );
    assert_eq!(
        api_schema(&tool)["request_body_schema"],
        json!({
            "properties": { "b": { "description": "B", "type": "string" } },
            "required": ["b"],
            "type": "object"
        })
    );
}

#[test]
fn webhook_reports_every_problem_at_once_with_paths() {
    let result = Tool::webhook("broken", "Broken")
        .get("https://api.example.com/{a}")
        .path_param("b", Schema::string().describe("B").from_variable("b"))
        .build();
    assert_eq!(
        paths_of(result),
        [
            ("missing_path_param", Some("path_params.a".to_string())),
            ("unknown_path_param", Some("path_params.b".to_string())),
            (
                "conflicting_value_source",
                Some("path_params.b".to_string())
            ),
        ]
    );
}

#[test]
fn webhook_error_names_the_tool_and_lists_the_problems() {
    let err = Tool::webhook("broken", "Broken").build().unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid tool \"broken\":\n  - Set the method and URL with get(), post(), put(), patch() or delete()"
    );
}

#[test]
fn webhook_needs_a_value_source_on_every_literal() {
    let result = Tool::webhook("save", "Save")
        .post("https://api.example.com/save")
        .query_param("q", Schema::string())
        .body(
            Schema::object().property(
                "profile",
                Schema::object()
                    .from_variable("profile")
                    .property("name", Schema::string()),
            ),
        )
        .response(Schema::object().property("status", Schema::string()))
        .build();
    assert_eq!(
        paths_of(result),
        [
            ("missing_value_source", Some("query.q".to_string())),
            (
                "missing_value_source",
                Some("body.profile.name".to_string())
            ),
            ("missing_value_source", Some("response.status".to_string())),
        ]
    );
}

#[test]
fn webhook_body_rules_per_method() {
    let body = Schema::object().property("note", Schema::string().describe("A note"));
    let url = "https://api.example.com/notes";
    let t = || Tool::webhook("t", "T");
    for tool in [t().post(url), t().put(url), t().patch(url)] {
        assert_eq!(codes_of(tool.build()), ["missing_body"]);
        assert!(tool.body(body.clone()).build().is_ok());
    }
    assert!(t().delete(url).build().is_ok());
    assert!(t().delete(url).body(body.clone()).build().is_ok());
    assert_eq!(
        codes_of(t().get(url).body(body).build()),
        ["body_not_allowed"]
    );
    let json_content = ToolOptions::new().set("content_type", "application/json");
    assert!(t().get(url).options(json_content).build().is_ok());
}

#[test]
fn webhook_needs_param_names_unique_across_path_query_and_body() {
    let result = Tool::webhook("t", "T")
        .post("https://api.example.com/orders")
        .query_param("id", Schema::string().describe("Id"))
        .body(Schema::object().property("id", Schema::string().describe("Id")))
        .build();
    assert_eq!(
        paths_of(result),
        [("duplicate_param", Some("body.id".to_string()))]
    );
}

#[test]
fn webhook_accepts_response_timeouts_from_5_to_300_seconds() {
    assert!(get_order().options(timeout(5)).build().is_ok());
    assert!(get_order().options(timeout(300)).build().is_ok());
    assert_eq!(
        codes_of(get_order().options(timeout(4)).build()),
        ["invalid_option"]
    );
    assert_eq!(
        codes_of(get_order().options(timeout(301)).build()),
        ["invalid_option"]
    );
}

#[test]
fn webhook_follows_redirects_only_to_an_allowed_list_of_domains() {
    let redirects = ToolOptions::new().set("follow_redirects", true);
    let allowed = redirects
        .clone()
        .set("follow_redirects_allowed_domains", ["example.com"]);
    assert!(get_order().options(allowed).build().is_ok());
    assert_eq!(
        codes_of(get_order().options(redirects).build()),
        ["invalid_option"]
    );
}

#[test]
fn webhook_rejects_an_option_of_another_kind() {
    let result = get_order()
        .options(ToolOptions::new().set("expects_response", true))
        .build();
    assert_eq!(
        paths_of(result),
        [("invalid_option", Some("expects_response".to_string()))]
    );
}

#[test]
fn webhook_switches_a_loaded_params_value_source_in_place() {
    let original = Tool::webhook("refund", "Refund")
        .post("https://api.example.com/refunds")
        .body(
            Schema::object()
                .property("order_id", Schema::string().describe("The order number"))
                .property("reason", Schema::string().describe("Why")),
        );
    let loaded = Tool::from_json(&original.to_json().unwrap())
        .into_webhook()
        .unwrap();
    let body = loaded
        .body_schema()
        .and_then(Schema::as_object)
        .unwrap()
        .clone();
    let loaded = loaded.body(body.property("order_id", Schema::string().from_variable("order_id")));
    assert_eq!(
        api_schema(&loaded)["request_body_schema"],
        json!({
            "properties": {
                "order_id": { "dynamic_variable": "order_id", "type": "string" },
                "reason": { "description": "Why", "type": "string" }
            },
            "required": ["order_id", "reason"],
            "type": "object"
        })
    );
}

// ---- value sources -----------------------------------------------------------------------------

#[test]
fn an_empty_description_or_variable_name_is_no_source() {
    let param = |schema: LiteralSchema| Tool::client("t", "T").param("x", schema).build();
    assert_eq!(
        codes_of(param(Schema::string().describe(""))),
        ["missing_value_source"]
    );
    assert_eq!(
        codes_of(param(Schema::string().from_variable(""))),
        ["missing_value_source"]
    );
    assert_eq!(
        codes_of(param(Schema::string().describe("").allowed_from("fares"))),
        ["allowed_values_needs_description", "missing_value_source"]
    );
}

#[test]
fn accepts_a_constant_of_a_nullable_literal_type_and_writes_the_type_back() {
    for (ty, value) in [
        (json!(["integer", "null"]), json!(42)),
        (json!(["number", "null"]), json!(1.5)),
        (json!(["boolean", "null"]), json!(true)),
    ] {
        let raw = json!({
            "description": "T",
            "name": "t",
            "parameters": {
                "properties": { "x": { "constant_value": value, "type": ty } },
                "required": ["x"],
                "type": "object"
            },
            "type": "client"
        });
        let tool = Tool::from_json(&raw);
        assert!(tool.build().is_ok());
        assert_eq!(tool.to_json().unwrap(), raw);
    }
}

#[test]
fn a_null_constant_of_a_nullable_literal_round_trips_and_build_does_not_drop_it() {
    // Loaded: `constant_value: null` survives from_json -> to_json, and reloading it.
    let raw = json!({
        "description": "T",
        "name": "t",
        "parameters": {
            "properties": { "x": { "constant_value": null, "type": ["integer", "null"] } },
            "required": ["x"],
            "type": "object"
        },
        "type": "client"
    });
    let tool = Tool::from_json(&raw);
    assert_eq!(tool.to_json().unwrap(), raw);
    let reloaded = Tool::from_json(&tool.to_json().unwrap());
    assert_eq!(reloaded.to_json().unwrap(), raw);
    // The generated request model reads null as unset, so build() reports it instead of
    // silently sending a param with no value source.
    let err = tool.build().unwrap_err();
    assert_eq!(err.codes(), ["constant_type_mismatch"]);
    assert!(err.issues[0].message.contains("to_json()"));
    assert_eq!(codes_of(tool.build_config()), ["constant_type_mismatch"]);

    // Built in code: the same.
    let nullable = Schema::from_json(&json!({ "type": ["integer", "null"] })).constant(Value::Null);
    let tool = Tool::client("t", "T").param("x", nullable);
    assert_eq!(
        tool.to_json().unwrap()["parameters"]["properties"]["x"],
        json!({ "constant_value": null, "type": ["integer", "null"] })
    );
    assert_eq!(codes_of(tool.build()), ["constant_type_mismatch"]);

    // A null constant is only valid on a nullable type.
    let plain = Schema::integer().constant(Value::Null);
    assert_eq!(
        codes_of(Tool::client("t", "T").param("x", plain).build()),
        ["constant_type_mismatch"]
    );
}

#[test]
fn a_loaded_null_constant_of_a_non_nullable_literal_is_absent() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "client",
        "parameters": {
            "type": "object",
            "properties": { "x": { "type": "string", "description": "X", "constant_value": null } },
            "required": ["x"]
        }
    }));
    assert!(tool.build().is_ok());
    assert_eq!(
        tool.to_json().unwrap()["parameters"]["properties"]["x"],
        json!({ "description": "X", "type": "string" })
    );
}

#[test]
fn still_rejects_a_constant_of_the_wrong_type_for_a_nullable_literal() {
    let raw = json!({
        "name": "t",
        "description": "T",
        "type": "client",
        "parameters": {
            "type": "object",
            "properties": { "x": { "type": ["integer", "null"], "constant_value": "a" } }
        }
    });
    assert_eq!(
        codes_of(Tool::from_json(&raw).build()),
        ["constant_type_mismatch"]
    );
}

#[test]
fn a_whole_float_is_an_integer_constant() {
    let tool = Tool::client("t", "T").param("x", Schema::integer().constant(3.0));
    assert!(tool.build().is_ok());
    assert_eq!(
        tool.to_json().unwrap()["parameters"]["properties"]["x"],
        json!({ "constant_value": 3, "type": "integer" })
    );
}

// ---- ClientTool --------------------------------------------------------------------------------

#[test]
fn client_loads_empty_params_with_no_type_as_an_object_and_can_add_to_them() {
    let tool = Tool::from_json(&json!({
        "name": "t", "description": "T", "type": "client", "parameters": {}
    }))
    .into_client()
    .unwrap();
    assert_eq!(tool.parameters_schema().unwrap().kind(), SchemaKind::Object);
    assert!(tool
        .param("x", Schema::string().describe("X"))
        .build()
        .is_ok());
}

#[test]
fn client_checks_require_any_of_against_the_params_it_still_has() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "client",
        "parameters": {
            "type": "object",
            "properties": {
                "a": { "type": "string", "description": "A" },
                "b": { "type": "string", "description": "B" }
            },
            "required_constraints": { "any_of": [{ "required": ["a", "b"] }] }
        }
    }))
    .into_client()
    .unwrap();
    assert!(tool.build().is_ok());
    let tool = tool.remove_param("b");
    assert_eq!(
        paths_of(tool.build()),
        [("unknown_required_property", Some("params".to_string()))]
    );
}

#[test]
fn client_removes_params() {
    let tool = Tool::client("show", "Show something")
        .param("a", Schema::string().describe("A"))
        .param("b", Schema::string().describe("B"))
        .remove_param("a");
    assert_eq!(tool.parameters_schema().unwrap().property_names(), ["b"]);
}

#[test]
fn client_leaves_parameters_out_when_there_are_none() {
    assert_eq!(
        Tool::client("ping", "Ping the client").to_json().unwrap(),
        json!({ "description": "Ping the client", "name": "ping", "type": "client" })
    );
}

#[test]
fn client_accepts_timeouts_from_1_to_120_or_minus_1_with_expects_response() {
    let build = |options: ToolOptions| Tool::client("wait", "Wait").options(options).build();
    assert!(build(timeout(1)).is_ok());
    assert!(build(timeout(120)).is_ok());
    assert!(build(timeout(-1).set("expects_response", true)).is_ok());
    for secs in [0, 121, -1] {
        assert_eq!(codes_of(build(timeout(secs))), ["invalid_option"]);
    }
}

// ---- SystemTool --------------------------------------------------------------------------------

#[test]
fn system_builds_an_entry_for_prompt_built_in_tools() {
    let entry = SystemTool::end_call().build_config().unwrap();
    let built_in = BuiltInToolsInput {
        end_call: Some(entry),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(built_in.end_call.unwrap()).unwrap(),
        json!({ "name": "end_call", "params": { "system_tool_type": "end_call" }, "type": "system" })
    );
}

#[test]
fn system_rejects_transfer_on_a_tool_that_does_not_transfer() {
    let result = SystemTool::end_call()
        .transfer(
            "Caller asks",
            Destination::phone("+15551234567"),
            Default::default(),
        )
        .build();
    assert_eq!(codes_of(result), ["invalid_option"]);
}

#[test]
fn system_transfers_to_a_sip_uri_only_with_a_sip_refer() {
    for sip in [
        Destination::sip("sip:sales@pbx.example.com"),
        Destination::sip_variable("partner_sip"),
    ] {
        let refer = ToolOptions::new().set("transfer_type", "sip_refer");
        assert!(SystemTool::transfer_to_number(Default::default())
            .transfer("Caller wants sales", sip.clone(), refer)
            .build()
            .is_ok());
        let result = SystemTool::transfer_to_number(Default::default())
            .transfer("Caller wants sales", sip, Default::default())
            .build();
        assert_eq!(codes_of(result), ["invalid_destination", "no_transfers"]);
    }
}

#[test]
fn system_rejects_options_of_the_other_transfer_kind() {
    let result = SystemTool::transfer_to_agent()
        .transfer(
            "Caller wants sales",
            Destination::agent("agent_sales", None),
            ToolOptions::new().set("transfer_type", "conference"),
        )
        .build();
    assert_eq!(
        paths_of(result),
        [("invalid_option", Some("transfer_type".to_string()))]
    );
}

#[test]
fn system_constructor_options_are_typed() {
    let tool = SystemTool::knowledge_base(KnowledgeBaseToolOptions {
        enabled_strategies: Some(vec![
            crate::api::types::SearchStrategy::Keyword,
            crate::api::types::SearchStrategy::Semantic,
        ]),
    });
    assert_eq!(
        tool.to_json().unwrap(),
        json!({
            "name": "knowledge_base",
            "params": {
                "enabled_strategies": ["keyword", "semantic"],
                "system_tool_type": "knowledge_base"
            },
            "type": "system"
        })
    );
    let skip = SystemTool::skip_turn(SkipTurnOptions {
        wait_timeout_secs: Some(15.0),
    });
    assert_eq!(
        skip.to_json().unwrap()["params"]["wait_timeout_secs"],
        json!(15)
    );
    assert!(skip
        .build()
        .unwrap()
        .tool_config
        .is_system_tool_config_input());
}

// ---- loading -----------------------------------------------------------------------------------

#[test]
fn loads_empty_webhook_body_and_response_schemas_with_no_type_as_objects() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "webhook",
        "api_schema": {
            "url": "https://api.example.com/t",
            "method": "POST",
            "request_body_schema": {},
            "response_body_schema": {}
        }
    }));
    let tool = tool.as_webhook().unwrap();
    assert_eq!(tool.body_schema().unwrap().kind(), SchemaKind::Object);
    assert_eq!(tool.response_schema().unwrap().kind(), SchemaKind::Object);
}

#[test]
fn loads_a_request_or_response_wrapper_and_a_string() {
    let config = get_order().to_json().unwrap();
    assert_eq!(
        Tool::from_json(&json!({ "tool_config": config.clone() }))
            .to_json()
            .unwrap(),
        config
    );
    let text = get_order().to_json_string().unwrap();
    assert_eq!(
        Tool::from_json_str(&text).unwrap().to_json().unwrap(),
        config
    );
}

#[test]
fn loads_the_result_of_tools_get() {
    let response: ToolResponseModel = serde_json::from_value(json!({
        "id": "tool_1",
        "tool_config": get_order().to_json().unwrap(),
        "access_info": {
            "is_creator": true,
            "creator_name": "x",
            "creator_email": "x@example.com",
            "role": "admin"
        },
        "usage_stats": { "avg_latency_secs": 0.0 }
    }))
    .expect("a tools.get() response");
    let tool = Tool::from_response(&response);
    assert_eq!(tool.kind(), ToolKind::Webhook);
    assert_eq!(tool.to_json().unwrap(), get_order().to_json().unwrap());
}

#[test]
fn reads_null_as_absent() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "client",
        "response_timeout_secs": null,
        "parameters": {
            "type": "object",
            "properties": { "x": { "type": "string", "description": "X", "enum": null } },
            "required": ["x"]
        }
    }));
    assert_eq!(
        tool.to_json().unwrap(),
        json!({
            "description": "T",
            "name": "t",
            "parameters": {
                "properties": { "x": { "description": "X", "type": "string" } },
                "required": ["x"],
                "type": "object"
            },
            "type": "client"
        })
    );
}

#[test]
fn keeps_a_tool_of_an_unknown_type_and_warns_when_built() {
    let raw = json!({ "name": "search", "type": "mcp", "mcp_server_id": "mcp_1" });
    let tool = Tool::from_json(&raw);
    assert_eq!(tool.kind(), ToolKind::Other);
    assert!(tool.as_webhook().is_none());
    let mut warnings = Vec::new();
    let built = tool.build_with_warnings(|issue| warnings.push(issue.code));
    assert_eq!(warnings, [ToolIssueCode::UnknownToolType]);
    // The generated enum has no variant that holds it unchanged.
    assert_eq!(codes_of(built), ["invalid_model"]);
    assert_eq!(tool.to_json().unwrap(), raw);
    // Nothing can be set on it.
    assert_eq!(
        codes_of(tool.options(ToolOptions::new().set("name", "x")).build()),
        ["invalid_option"]
    );
}

#[test]
fn refuses_to_build_a_tool_the_generated_types_would_change() {
    let raw = json!({ "name": "lookup", "description": "Look up", "type": "api_integration_webhook", "x": 1 });
    let tool = Tool::from_json(&raw);
    assert_eq!(codes_of(tool.build()), ["invalid_model"]);
    assert_eq!(tool.to_json().unwrap(), raw);
}

#[test]
fn keeps_a_loaded_system_tool_type_the_builder_has_no_constructor_for() {
    let raw = json!({
        "name": "start_procedure",
        "params": {
            "procedures": { "refund": { "procedure_id": "proc_1" } },
            "system_tool_type": "start_procedure"
        },
        "type": "system"
    });
    let tool = Tool::from_json(&raw);
    assert_eq!(
        tool.as_system().unwrap().system_tool_type(),
        "start_procedure"
    );
    assert_eq!(tool.to_json().unwrap(), raw);
}

#[test]
fn tool_build_config_wraps_each_kind_in_its_variant() {
    let webhook: Tool = get_order().into();
    assert!(matches!(
        webhook.build_config().unwrap(),
        ToolRequestModelToolConfig::WebhookToolConfigInput(_)
    ));
    let client: Tool = Tool::client("ping", "Ping").into();
    assert!(client
        .build()
        .unwrap()
        .tool_config
        .is_client_tool_config_input());
}

#[test]
fn a_loaded_field_of_the_wrong_shape_is_a_validation_error_not_a_panic() {
    let raw = json!({
        "name": "t",
        "description": "T",
        "type": "client",
        "expects_response": "yes"
    });
    let tool = Tool::from_json(&raw);
    assert_eq!(codes_of(tool.build()), ["invalid_model"]);
    // The canonical JSON still round-trips.
    assert_eq!(tool.to_json().unwrap(), raw);
}

// ---- review fixes: rules shared by every SDK ----------------------------------------------

#[test]
fn removing_the_last_assignment_leaves_assignments_out() {
    let tool = get_order()
        .assign("a", "a", Default::default())
        .remove_assignment("a");
    assert!(tool.to_json().unwrap().get("assignments").is_none());
    assert_eq!(tool.to_json().unwrap(), get_order().to_json().unwrap());
}

#[test]
fn a_loaded_empty_assignments_list_round_trips_unless_edited() {
    let mut raw = get_order().to_json().unwrap();
    raw["assignments"] = json!([]);
    let tool = Tool::from_json(&raw).into_webhook().unwrap();
    assert_eq!(tool.to_json().unwrap(), raw);
    // Removing a variable that isn't there changes nothing.
    assert_eq!(tool.remove_assignment("missing").to_json().unwrap(), raw);
}

#[test]
fn removing_the_last_client_param_leaves_parameters_out() {
    let tool = Tool::client("show", "Show")
        .param("a", Schema::string().describe("A"))
        .remove_param("a");
    assert!(tool.parameters_schema().is_none());
    assert_eq!(
        tool.to_json().unwrap(),
        json!({ "description": "Show", "name": "show", "type": "client" })
    );
    // Removing a param that isn't there changes nothing.
    let one = Tool::client("show", "Show").param("a", Schema::string().describe("A"));
    assert_eq!(
        one.clone().remove_param("missing").to_json().unwrap(),
        one.to_json().unwrap()
    );
}

#[test]
fn loaded_empty_parameters_round_trip_unless_edited() {
    let raw = json!({
        "description": "T",
        "name": "t",
        "parameters": { "type": "object" },
        "type": "client"
    });
    let tool = Tool::from_json(&raw).into_client().unwrap();
    assert_eq!(tool.to_json().unwrap(), raw);
    assert_eq!(tool.remove_param("missing").to_json().unwrap(), raw);
}

#[test]
fn an_empty_url_is_missing() {
    assert_eq!(
        codes_of(Tool::webhook("t", "T").get("").build()),
        ["missing_url"]
    );
    let loaded = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "webhook",
        "api_schema": { "method": "GET", "url": "" }
    }));
    assert_eq!(codes_of(loaded.build()), ["missing_url"]);
}

#[test]
fn a_loaded_body_or_response_of_another_type_is_kept_and_reported() {
    let not_object = json!({ "type": "string", "description": "X" });
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "webhook",
        "api_schema": {
            "method": "POST",
            "url": "https://api.example.com/t",
            "request_body_schema": not_object,
            "response_body_schema": not_object
        }
    }))
    .into_webhook()
    .unwrap();
    assert_eq!(tool.body_schema().unwrap().kind(), SchemaKind::String);
    assert_eq!(tool.body_schema().unwrap().to_json(), not_object);
    assert_eq!(tool.response_schema().unwrap().kind(), SchemaKind::String);
    assert_eq!(
        paths_of(tool.build()),
        [
            ("body_not_object", Some("body".to_string())),
            ("body_not_object", Some("response".to_string())),
        ]
    );
}

#[test]
fn loaded_client_params_of_another_type_are_kept_and_reported() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "client",
        "parameters": { "type": "string", "description": "X" }
    }))
    .into_client()
    .unwrap();
    assert!(tool.parameters_schema().is_none());
    // Adding a param doesn't rewrite them as an object either.
    let tool = tool.param("y", Schema::string().describe("Y"));
    assert_eq!(
        paths_of(tool.build()),
        [("body_not_object", Some("params".to_string()))]
    );
}

#[test]
fn a_loaded_query_schema_of_another_type_is_kept_and_reported() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "webhook",
        "api_schema": {
            "method": "GET",
            "url": "https://api.example.com/t",
            "query_params_schema": { "type": "string", "description": "X" }
        }
    }));
    assert_eq!(
        paths_of(tool.build()),
        [("body_not_object", Some("query".to_string()))]
    );
}

#[test]
fn a_loaded_empty_schema_is_a_valid_empty_object() {
    let tool = Tool::from_json(&json!({
        "name": "t",
        "description": "T",
        "type": "webhook",
        "api_schema": {
            "method": "POST",
            "url": "https://api.example.com/t",
            "request_body_schema": {},
            "response_body_schema": {},
            "query_params_schema": {}
        }
    }));
    assert!(tool.build().is_ok());
    // Body and response get `"type": "object"`, as in JS; the query schema's type stays
    // synthetic, so none is written.
    let api = &tool.to_json().unwrap()["api_schema"];
    assert_eq!(api["request_body_schema"], json!({ "type": "object" }));
    assert_eq!(api["response_body_schema"], json!({ "type": "object" }));
    assert_eq!(api["query_params_schema"], json!({}));
    let client = Tool::from_json(&json!({
        "name": "t", "description": "T", "type": "client", "parameters": {}
    }));
    assert!(client.build().is_ok());
    assert_eq!(
        client.to_json().unwrap()["parameters"],
        json!({ "type": "object" })
    );
    // And after an edit.
    let edited = client
        .into_client()
        .unwrap()
        .param("x", Schema::string().describe("X"));
    assert_eq!(
        edited.to_json().unwrap()["parameters"]["type"],
        json!("object")
    );
}

#[test]
fn a_loaded_query_schema_keeps_every_loaded_field() {
    let query = json!({
        "description": "",
        "properties": { "q": { "description": "Query", "type": "string" } },
        "required": ["q"],
        "x_custom": { "kept": true }
    });
    let raw = json!({
        "api_schema": {
            "method": "GET",
            "query_params_schema": query,
            "url": "https://api.example.com/search"
        },
        "description": "Search",
        "name": "search",
        "type": "webhook"
    });
    let tool = Tool::from_json(&raw).into_webhook().unwrap();
    assert_eq!(tool.to_json().unwrap(), raw);
    // Extra fields survive an edit, and no `type` is added.
    let edited = tool.query_param("limit", Schema::integer().constant(10));
    let written = &api_schema(&edited)["query_params_schema"];
    assert_eq!(written["x_custom"], json!({ "kept": true }));
    assert_eq!(written["description"], json!(""));
    assert!(written.get("type").is_none());
    assert_eq!(written["required"], json!(["q", "limit"]));
}

#[test]
fn a_loaded_query_schema_with_extra_fields_but_no_properties_round_trips() {
    for query in [
        json!({ "required": [], "x_custom": 1 }),
        json!({ "properties": {}, "type": "object" }),
    ] {
        let raw = json!({
            "api_schema": {
                "method": "GET",
                "query_params_schema": query,
                "url": "https://api.example.com/search"
            },
            "description": "Search",
            "name": "search",
            "type": "webhook"
        });
        assert_eq!(Tool::from_json(&raw).to_json().unwrap(), raw);
    }
}

// ---- review fixes, round 2 ---------------------------------------------------------------------

#[test]
fn rejects_an_integer_constant_the_request_model_would_round() {
    // Above i64::MAX, the generated constant union falls back to f64 and rounds it.
    let big = 9_223_372_036_854_775_809_u64;
    let tool = Tool::client("t", "T").param("id", Schema::integer().constant(big));
    assert_eq!(
        paths_of(tool.build()),
        [("constant_type_mismatch", Some("params.id".to_string()))]
    );
    assert_eq!(codes_of(tool.to_json()), ["constant_type_mismatch"]);
    let tool = Tool::client("t", "T").param("n", Schema::number().constant(u64::MAX));
    assert_eq!(codes_of(tool.build()), ["constant_type_mismatch"]);
    // Loaded the same.
    let loaded = Tool::from_json_str(
        r#"{"name":"t","description":"T","type":"client","parameters":{"type":"object",
            "properties":{"id":{"type":"integer","constant_value":9223372036854775809}},
            "required":["id"]}}"#,
    )
    .unwrap();
    assert_eq!(codes_of(loaded.build()), ["constant_type_mismatch"]);
    // i64::MAX, a negative integer and a whole float beyond i64 are all sent exactly.
    for constant in [json!(i64::MAX), json!(i64::MIN), json!(1e19)] {
        let schema = Schema::number().constant(constant.clone());
        let tool = Tool::client("t", "T").param("n", schema);
        let sent = serde_json::to_value(tool.build().unwrap()).unwrap();
        assert_eq!(
            sent["tool_config"]["parameters"]["properties"]["n"]["constant_value"],
            constant
        );
    }
    // Inside an array constant the value is free-form JSON, sent as is.
    let tags = Schema::array(Schema::integer().describe("Tag")).constant(json!([big]));
    let sent =
        serde_json::to_value(Tool::client("t", "T").param("tags", tags).build().unwrap()).unwrap();
    assert_eq!(
        sent["tool_config"]["parameters"]["properties"]["tags"]["constant_value"],
        json!([big])
    );
}

#[test]
fn a_null_constant_of_a_param_named_like_a_schema_field_is_still_reported() {
    let nullable =
        || Schema::from_json(&json!({ "type": ["string", "null"] })).constant(Value::Null);
    for name in [
        "constant_value",
        "dynamic_variable_placeholders",
        "properties",
    ] {
        let tool = Tool::client("t", "T").param(name, nullable());
        assert_eq!(
            paths_of(tool.build()),
            [("constant_type_mismatch", Some(format!("params.{name}")))],
            "{name}"
        );
        // to_json() keeps it, and it survives a reload.
        let json = tool.to_json().unwrap();
        assert_eq!(
            json["parameters"]["properties"][name],
            json!({ "constant_value": null, "type": ["string", "null"] })
        );
        assert_eq!(Tool::from_json(&json).to_json().unwrap(), json);

        let webhook = Tool::webhook("t", "T")
            .post(format!("https://api.example.com/{{{name}}}"))
            .path_param(name, nullable())
            .body(Schema::object().property("b", Schema::object().property(name, nullable())));
        assert_eq!(
            paths_of(webhook.build()),
            [
                (
                    "constant_type_mismatch",
                    Some(format!("path_params.{name}"))
                ),
                ("constant_type_mismatch", Some(format!("body.b.{name}"))),
            ],
            "{name}"
        );
        let json = webhook.to_json().unwrap();
        assert_eq!(
            json["api_schema"]["path_params_schema"][name]["constant_value"],
            Value::Null
        );
    }
    // Nested in the body, too.
    let body = Schema::object().property(
        "order",
        Schema::object().property("constant_value", nullable()),
    );
    let tool = Tool::webhook("t", "T")
        .post("https://api.example.com/t")
        .body(body);
    assert_eq!(
        paths_of(tool.build()),
        [(
            "constant_type_mismatch",
            Some("body.order.constant_value".to_string())
        )]
    );
}

#[test]
fn a_loaded_null_property_is_absent_whatever_its_name() {
    for name in ["x", "constant_value", "dynamic_variable_placeholders"] {
        let tool = Tool::from_json(&json!({
            "name": "t",
            "description": "T",
            "type": "client",
            "parameters": {
                "type": "object",
                "properties": { "a": { "type": "string", "description": "A" }, name: null }
            }
        }));
        assert!(tool.build().is_ok(), "{name}");
        assert_eq!(
            tool.to_json().unwrap()["parameters"]["properties"],
            json!({ "a": { "description": "A", "type": "string" } })
        );
    }
}

#[test]
fn a_modeled_schema_field_of_the_wrong_shape_is_kept_and_reported() {
    let client = |parameters: Value| {
        json!({
            "description": "T",
            "name": "t",
            "parameters": parameters,
            "type": "client"
        })
    };
    for parameters in [
        json!({ "properties": { "x": { "description": "X", "is_system_provided": "yes", "type": "string" } }, "type": "object" }),
        json!({ "properties": { "x": { "description": "X", "is_omitted": "yes", "type": "string" } }, "type": "object" }),
        json!({ "properties": { "x": { "description": "X", "type": "string" } }, "required": "x", "type": "object" }),
        json!({ "properties": { "x": { "description": "X", "type": "string" } }, "required": ["x", 1], "type": "object" }),
        json!({ "properties": "x", "type": "object" }),
        json!({ "properties": { "x": 5 }, "required": ["x"], "type": "object" }),
        json!({ "properties": { "x": { "description": "X", "type": "string" } }, "required_constraints": { "any_of": "x" }, "type": "object" }),
        json!({ "properties": { "x": { "items": "x", "type": "array" } }, "type": "object" }),
        json!({ "properties": { "x": { "allowed_values": "x", "description": "X", "type": "string" } }, "type": "object" }),
    ] {
        let raw = client(parameters);
        let tool = Tool::from_json(&raw);
        assert_eq!(codes_of(tool.build()), ["invalid_model"], "{raw}");
        // to_json() writes the input back as loaded.
        assert_eq!(tool.to_json().unwrap(), raw);
    }
}

#[test]
fn loaded_client_parameters_that_are_not_an_object_are_kept_and_reported() {
    for parameters in [json!(42), json!("x"), json!([])] {
        let raw = json!({
            "description": "T",
            "name": "t",
            "parameters": parameters,
            "type": "client"
        });
        let tool = Tool::from_json(&raw).into_client().unwrap();
        assert!(tool.parameters_schema().is_none());
        assert_eq!(codes_of(tool.build()), ["invalid_model"]);
        assert_eq!(tool.to_json().unwrap(), raw);
        // Adding a param doesn't replace them.
        let edited = tool.param("y", Schema::string().describe("Y"));
        assert_eq!(edited.to_json().unwrap(), raw);
        assert_eq!(codes_of(edited.build()), ["invalid_model"]);
    }
}

#[test]
fn loaded_system_params_without_a_system_tool_type_are_kept_and_reported() {
    for params in [
        Some(json!([])),
        Some(json!(42)),
        Some(json!({})),
        Some(json!({ "system_tool_type": 5 })),
        Some(json!({ "system_tool_type": "" })),
        None,
    ] {
        let mut raw = json!({ "name": "end_call", "type": "system" });
        if let Some(params) = &params {
            raw["params"] = params.clone();
        }
        let tool = Tool::from_json(&raw);
        assert_eq!(
            paths_of(tool.build()),
            [("invalid_model", Some("params".to_string()))],
            "{raw}"
        );
        assert_eq!(tool.to_json().unwrap(), raw);
    }
    // A system_tool_type the builder has no constructor for is still fine.
    let raw =
        json!({ "name": "x", "params": { "system_tool_type": "end_procedure" }, "type": "system" });
    assert!(Tool::from_json(&raw).build().is_ok());
}

#[test]
fn a_path_param_is_required_even_when_marked_optional() {
    let tool = Tool::webhook("t", "T")
        .get("https://api.example.com/{id}")
        .path_param("id", Schema::string().omit().optional());
    assert_eq!(
        paths_of(tool.build()),
        [("omitted_required", Some("path_params.id".to_string()))]
    );
    // optional() alone is a no-op: the reloaded JSON is the same and builds.
    let tool = Tool::webhook("t", "T")
        .get("https://api.example.com/{id}")
        .path_param("id", Schema::string().describe("Id").optional());
    let json = tool.to_json().unwrap();
    assert!(tool.build().is_ok());
    let reloaded = Tool::from_json(&json);
    assert!(reloaded.build().is_ok());
    assert_eq!(reloaded.to_json().unwrap(), json);
}

#[test]
fn to_json_string_formats_numbers_as_the_js_sdk_does() {
    let tool = Tool::client("t", "T")
        .param("a", Schema::number().constant(1e-7))
        .param("b", Schema::number().constant(1.5e-7))
        .param("c", Schema::number().constant(1e21))
        .param("d", Schema::number().constant(123456789012345680000.0))
        .param("e", Schema::number().constant(0.000001));
    let out = tool.clone().to_json_string().unwrap();
    for (name, js) in [
        ("a", "1e-7"),
        ("b", "1.5e-7"),
        ("c", "1e+21"),
        ("d", "123456789012345680000"),
        ("e", "0.000001"),
    ] {
        let line = format!(
            "\"{name}\": {{\n        \"constant_value\": {js},\n        \"type\": \"number\"\n      }}"
        );
        assert!(out.contains(&line), "{name}: {out}");
    }
    // And it loads back to the same values ("d" too: serde_json's `float_roundtrip` parser reads
    // it as the f64 it was written from).
    let reloaded = Tool::from_json_str(&out).unwrap();
    assert_eq!(reloaded.to_json_string().unwrap(), out);
    assert!(reloaded.build().is_ok());
    let schema =
        Schema::from_json_str("{\"constant_value\": 123456789012345680000, \"type\": \"number\"}");
    assert_eq!(
        schema.unwrap().to_json()["constant_value"],
        json!(123456789012345680000.0)
    );
}

// ---- review fixes, round 3 ---------------------------------------------------------------------

#[test]
fn accepts_a_constant_above_i64_max_that_is_exact_as_an_f64() {
    // `1e19` is written as the integer 10000000000000000000, which loads as a u64 above i64::MAX.
    // The request model sends it as the f64 it equals, so it reloads and builds.
    let tool = Tool::client("t", "T").param("n", Schema::number().constant(1e19));
    let out = tool.to_json_string().unwrap();
    assert!(
        out.contains("\"constant_value\": 10000000000000000000,"),
        "{out}"
    );
    let reloaded = Tool::from_json_str(&out).unwrap();
    assert_eq!(reloaded.to_json_string().unwrap(), out);
    let sent = serde_json::to_value(reloaded.build().unwrap()).unwrap();
    assert_eq!(
        sent["tool_config"]["parameters"]["properties"]["n"]["constant_value"],
        json!(1e19)
    );
    // An integer schema too, and 2^63 (the first value above i64::MAX), which is exact.
    for json in [
        r#"{"type":"integer","constant_value":10000000000000000000}"#,
        r#"{"type":"number","constant_value":9223372036854775808}"#,
    ] {
        let tool = Tool::client("t", "T").param("n", Schema::from_json_str(json).unwrap());
        assert!(tool.build().is_ok(), "{json}");
    }
    // One the f64 would round is still rejected.
    let tool = Tool::client("t", "T").param(
        "n",
        Schema::from_json_str(r#"{"type":"number","constant_value":10000000000000000001}"#)
            .unwrap(),
    );
    assert_eq!(codes_of(tool.build()), ["constant_type_mismatch"]);
}

#[test]
fn loaded_empty_required_constraint_groups_round_trip() {
    let client = |constraints: Value| {
        json!({
            "description": "T",
            "name": "t",
            "parameters": {
                "properties": { "a": { "description": "A", "type": "string" } },
                "required": ["a"],
                "required_constraints": constraints,
                "type": "object"
            },
            "type": "client"
        })
    };
    for constraints in [
        json!({}),
        json!({ "any_of": [] }),
        json!({ "all_of": [], "any_of": [] }),
        json!({ "all_of": [], "any_of": [{ "required": ["a"] }] }),
    ] {
        let raw = client(constraints);
        let tool = Tool::from_json(&raw);
        assert_eq!(tool.to_json().unwrap(), raw);
        assert!(tool.build().is_ok(), "{raw}");
    }
    // A group appended to a loaded schema merges with its groups, keeping the empty one.
    let body = Schema::from_json(&json!({
        "properties": { "a": { "description": "A", "type": "string" } },
        "required_constraints": { "all_of": [] },
        "type": "object"
    }))
    .into_object()
    .unwrap()
    .require_any_of(["a"]);
    assert_eq!(
        body.to_json()["required_constraints"],
        json!({ "all_of": [], "any_of": [{ "required": ["a"] }] })
    );
}

#[test]
fn a_loaded_enum_that_is_not_a_list_of_strings_is_kept_and_reported() {
    for choices in [json!(["a", 1]), json!("a"), json!({ "a": 1 })] {
        let raw = json!({
            "description": "T",
            "name": "t",
            "parameters": {
                "properties": { "x": { "description": "X", "enum": choices, "type": "string" } },
                "required": ["x"],
                "type": "object"
            },
            "type": "client"
        });
        let tool = Tool::from_json(&raw);
        assert_eq!(
            paths_of(tool.build()),
            [("invalid_model", Some("params.x".to_string()))],
            "{raw}"
        );
        assert_eq!(tool.to_json().unwrap(), raw);
    }
}

#[test]
fn rejects_params_a_modeled_system_tool_type_does_not_match() {
    let tool = SystemTool::transfer_to_agent().transfer(
        "Caller asks for sales",
        Destination::agent("agent_sales", None),
        ToolOptions::new().set("delay_ms", "soon"),
    );
    assert_eq!(
        paths_of(tool.build()),
        [("invalid_model", Some("params".to_string()))]
    );
    // to_json() writes it as given.
    assert_eq!(
        tool.to_json().unwrap()["params"]["transfers"][0]["delay_ms"],
        json!("soon")
    );
    // Loaded the same.
    let raw = json!({
        "name": "skip_turn",
        "params": { "system_tool_type": "skip_turn", "wait_timeout_secs": "later" },
        "type": "system"
    });
    let loaded = Tool::from_json(&raw);
    assert_eq!(
        paths_of(loaded.build()),
        [("invalid_model", Some("params".to_string()))]
    );
    // A type the generated enum doesn't model still goes to its catch-all as loaded.
    let raw = json!({ "name": "x", "params": { "system_tool_type": "future_tool", "a": 1 }, "type": "system" });
    let sent = serde_json::to_value(Tool::from_json(&raw).build().unwrap()).unwrap();
    assert_eq!(sent["tool_config"]["params"], raw["params"]);
}

#[test]
fn modeled_system_tool_types_match_the_generated_params_enum() {
    // The variants' wire names, read from the generated source, so a regeneration that adds or
    // renames one fails here until `MODELED_SYSTEM_TOOL_TYPES` is updated.
    let source = include_str!("../api/types/system_tool_config_input_params.rs");
    let generated: Vec<&str> = source
        .lines()
        .filter_map(|line| line.trim().strip_prefix("#[serde(rename = \""))
        .filter_map(|rest| rest.split('"').next())
        .collect();
    assert!(!generated.is_empty());
    assert_eq!(generated, super::system::MODELED_SYSTEM_TOOL_TYPES);
}

#[test]
fn to_json_string_writes_a_whole_float_as_javascript_does() {
    // The nearest i64 to this f64 is 1000000000000000128; JavaScript writes 1000000000000000100.
    let tool = Tool::client("t", "T").param("n", Schema::number().constant(1000000000000000100.0));
    let out = tool.to_json_string().unwrap();
    assert!(
        out.contains("\"constant_value\": 1000000000000000100,"),
        "{out}"
    );
    let schema = Schema::number().constant(1000000000000000100.0);
    assert!(super::json::to_json_string(&schema.to_json()).contains("1000000000000000100,"));
    // It reloads to the same bytes, and builds.
    let reloaded = Tool::from_json_str(&out).unwrap();
    assert_eq!(reloaded.to_json_string().unwrap(), out);
    assert!(tool.build().is_ok());
    // A whole float JavaScript writes as its integer is still an integer in to_json().
    let tool = Tool::client("t", "T").param("n", Schema::number().constant(3.0));
    assert_eq!(
        tool.to_json().unwrap()["parameters"]["properties"]["n"]["constant_value"],
        json!(3)
    );
}

#[test]
fn loaded_empty_path_params_and_headers_round_trip_until_emptied() {
    let raw = json!({
        "api_schema": {
            "method": "GET",
            "path_params_schema": {},
            "request_headers": {},
            "url": "https://api.example.com/x"
        },
        "description": "T",
        "name": "t",
        "type": "webhook"
    });
    let tool = Tool::from_json(&raw);
    assert_eq!(tool.to_json().unwrap(), raw);
    assert!(tool.build().is_ok());
    // Removing what isn't there leaves them as loaded.
    let tool = tool
        .into_webhook()
        .unwrap()
        .remove_header("X")
        .remove_path_param("x");
    assert_eq!(tool.to_json().unwrap(), raw);
    // Removing the last entry of a loaded map leaves the field out.
    let raw = json!({
        "api_schema": {
            "method": "GET",
            "path_params_schema": { "id": { "description": "Id", "type": "string" } },
            "request_headers": { "X": "1" },
            "url": "https://api.example.com/{id}"
        },
        "description": "T",
        "name": "t",
        "type": "webhook"
    });
    let tool = Tool::from_json(&raw)
        .into_webhook()
        .unwrap()
        .get("https://api.example.com/x")
        .remove_path_param("id")
        .remove_header("X");
    let api = api_schema(&tool);
    assert!(api.get("path_params_schema").is_none(), "{api}");
    assert!(api.get("request_headers").is_none(), "{api}");
}

fn transfer_rule() -> Value {
    json!({
        "condition": "Caller asks for the front desk",
        "provider_configs": [{
            "type": "twilio",
            "config": {
                "type": "twilio_cold_blind_transfer",
                "transfer_destination": { "type": "phone", "destination": "+15551234567" }
            }
        }]
    })
}

fn loaded_transfer(system_tool_type: &str) -> Tool {
    Tool::from_json(&json!({
        "type": "system",
        "name": system_tool_type,
        "params": {
            "system_tool_type": system_tool_type,
            "transfers": [],
            "transfer_rules": [transfer_rule()]
        }
    }))
}

#[test]
fn transfer_rules_do_not_satisfy_an_agent_transfer() {
    assert_eq!(
        codes_of(loaded_transfer("transfer_to_agent").to_json()),
        ["no_transfers"]
    );
}

#[test]
fn build_never_drops_loaded_transfer_rules() {
    let tool = loaded_transfer("transfer_to_number");
    assert_eq!(
        tool.to_json().unwrap()["params"]["transfer_rules"],
        json!([transfer_rule()])
    );
    match tool.build() {
        // The generated model has transfer_rules: the request carries them.
        Ok(request) => {
            let sent = serde_json::to_value(&request).unwrap();
            assert_eq!(
                sent["tool_config"]["params"]["transfer_rules"],
                json!([transfer_rule()]),
                "{sent}"
            );
        }
        // It doesn't yet: build() refuses rather than send a tool with no destinations.
        Err(err) => assert_eq!(
            err.issues
                .iter()
                .map(|i| (i.code.as_str(), i.path.clone()))
                .collect::<Vec<_>>(),
            [("invalid_model", Some("params.transfer_rules".to_string()))]
        ),
    }
}
