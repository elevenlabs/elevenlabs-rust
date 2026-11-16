use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "tool_name": "value",
        "force_pre_tool_speech": true,
        "pre_tool_speech": "auto",
        "disable_interruptions": true,
        "interruption_mode": "allow",
        "tool_call_sound": "typing",
        "tool_call_sound_behavior": "auto",
        "execution_mode": "immediate",
        "response_timeout_secs": 123,
        "assignments": [
            {
                "source": "response",
                "dynamic_variable": "value",
                "value_path": "value",
                "sanitize": true,
                "preserve_native_type": true
            }
        ],
        "input_overrides": {
            "key": {
                "source": "constant",
                "constant_value": "value"
            }
        },
        "response_mocks": [
            {
                "parameter_conditions": [
                    {
                        "eval": {
                            "type": "llm",
                            "description": "value"
                        },
                        "path": "value"
                    }
                ],
                "mock_result": "value",
                "is_error": true
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/mcp-servers/value/tool-configs/value"))
        .respond_with(ResponseTemplate::new(200).set_body_json(expected_response.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.tool_configs.get("value", "value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/convai/mcp-servers/value/tool-configs/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/mcp-servers/value/tool-configs/value request"
    );
    let request = matched[0];
    let auth_header = request
        .headers
        .get("xi-api-key")
        .map(|value| value.to_str().unwrap_or_default());
    assert!(
        auth_header.is_some_and(|value| value.starts_with("test_api_key")),
        "expected xi-api-key to start with \"test_api_key\", got {:?}",
        auth_header
    );
    let decoded = result.expect("call should succeed");
    assert_eq!(
        serde_json::to_value(&decoded).expect("response should serialize"),
        expected_response
    );
}

#[tokio::test]
async fn get_surfaces_the_declared_404() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/convai/mcp-servers/value/tool-configs/value"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.tool_configs.get("value", "value", None).await;

    match result {
        Ok(_) => panic!("expected the declared 404 to surface as an error"),
        Err(ApiError::Http { status, message }) => {
            assert_eq!(status, 404);
            assert!(
                message.is_empty(),
                "a failure with no declared body should carry none"
            );
        }
        Err(other) => panic!("expected ApiError::Http, got {:?}", other),
    }
}

#[tokio::test]
async fn delete_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "id": "value",
        "config": {
            "approval_policy": "auto_approve_all",
            "tool_approval_hashes": [
                {
                    "tool_name": "value",
                    "tool_hash": "value",
                    "approval_policy": "auto_approved"
                }
            ],
            "transport": "SSE",
            "url": "value",
            "secret_token": {
                "secret_id": "value"
            },
            "request_headers": {
                "key": "value"
            },
            "request_meta": {
                "key": "value"
            },
            "auth_connection": {
                "auth_connection_id": "value"
            },
            "name": "value",
            "description": "value",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "execution_mode": "immediate",
            "response_timeout_secs": 123,
            "tool_config_overrides": [
                {
                    "tool_name": "value",
                    "force_pre_tool_speech": true,
                    "pre_tool_speech": "auto",
                    "disable_interruptions": true,
                    "interruption_mode": "allow",
                    "tool_call_sound": "typing",
                    "tool_call_sound_behavior": "auto",
                    "execution_mode": "immediate",
                    "response_timeout_secs": 123,
                    "assignments": [
                        {
                            "source": "response",
                            "dynamic_variable": "value",
                            "value_path": "value",
                            "sanitize": true,
                            "preserve_native_type": true
                        }
                    ],
                    "input_overrides": {
                        "key": {
                            "source": "constant",
                            "constant_value": "value"
                        }
                    },
                    "response_mocks": [
                        {
                            "parameter_conditions": [
                                {
                                    "eval": {
                                        "type": "llm",
                                        "description": "value"
                                    },
                                    "path": "value"
                                }
                            ],
                            "mock_result": "value",
                            "is_error": true
                        }
                    ]
                }
            ],
            "disable_compression": true
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "dependent_agents": [
            {
                "referenced_resource_ids": [
                    "value"
                ],
                "id": "value",
                "name": "value",
                "type": "available",
                "created_at_unix_secs": 123,
                "access_level": "admin"
            }
        ],
        "metadata": {
            "created_at": 123,
            "owner_user_id": "value"
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/convai/mcp-servers/value/tool-configs/value"))
        .respond_with(ResponseTemplate::new(200).set_body_json(expected_response.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.tool_configs.delete("value", "value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE"
                && entry.url.path() == "/v1/convai/mcp-servers/value/tool-configs/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/convai/mcp-servers/value/tool-configs/value request"
    );
    let request = matched[0];
    let auth_header = request
        .headers
        .get("xi-api-key")
        .map(|value| value.to_str().unwrap_or_default());
    assert!(
        auth_header.is_some_and(|value| value.starts_with("test_api_key")),
        "expected xi-api-key to start with \"test_api_key\", got {:?}",
        auth_header
    );
    let decoded = result.expect("call should succeed");
    assert_eq!(
        serde_json::to_value(&decoded).expect("response should serialize"),
        expected_response
    );
}

#[tokio::test]
async fn delete_surfaces_the_declared_422() {
    let server = MockServer::start().await;
    let error_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "detail": [
            {
                "loc": [
                    "value"
                ],
                "msg": "value",
                "type": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/convai/mcp-servers/value/tool-configs/value"))
        .respond_with(ResponseTemplate::new(422).set_body_json(error_body.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.tool_configs.delete("value", "value", None).await;

    match result {
        Ok(_) => panic!("expected the declared 422 to surface as an error"),
        Err(ApiError::UnprocessableEntityError(body)) => {
            assert_eq!(
                serde_json::to_value(&body).expect("the typed error body should serialize"),
                error_body
            );
        }
        Err(other) => panic!(
            "expected ApiError::UnprocessableEntityError, got {:?}",
            other
        ),
    }
}
