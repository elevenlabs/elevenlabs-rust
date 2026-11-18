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
    let config = request.tool_config.as_webhook().expect("a webhook config");
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
fn accepts_a_null_constant_only_for_a_nullable_literal_type() {
    let nullable = Schema::from_json(&json!({ "type": ["integer", "null"] })).constant(Value::Null);
    assert!(Tool::client("t", "T").param("x", nullable).build().is_ok());
    let plain = Schema::integer().constant(Value::Null);
    assert_eq!(
        codes_of(Tool::client("t", "T").param("x", plain).build()),
        ["constant_type_mismatch"]
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
        json!({ "name": "end_call", "params": { "system_tool_type": "end_call" } })
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
    assert!(skip.build().unwrap().tool_config.is_system());
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
    tool.build_with_warnings(|issue| warnings.push(issue.code))
        .unwrap();
    assert_eq!(warnings, [ToolIssueCode::UnknownToolType]);
    assert_eq!(tool.to_json().unwrap(), raw);
    // Nothing can be set on it.
    assert_eq!(
        codes_of(tool.options(ToolOptions::new().set("name", "x")).build()),
        ["invalid_option"]
    );
}

#[test]
fn builds_a_tool_of_a_type_the_generated_types_do_not_model() {
    let raw = json!({ "name": "lookup", "type": "api_integration_webhook", "x": 1 });
    let request = Tool::from_json(&raw).build().unwrap();
    assert_eq!(serde_json::to_value(&request.tool_config).unwrap(), raw);
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
        ToolRequestModelToolConfig::Webhook { .. }
    ));
    let client: Tool = Tool::client("ping", "Ping").into();
    assert!(client.build().unwrap().tool_config.is_client());
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
