use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "created_branch_id": "value",
        "created_version_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "parent_version_id": "value",
        "name": "value",
        "description": "value",
        "conversation_config": {},
        "platform_settings": {},
        "workflow": {
            "edges": {
                "key": {
                    "source": "value",
                    "target": "value",
                    "forward_condition": {
                        "label": "value",
                        "type": "unconditional"
                    },
                    "backward_condition": {
                        "label": "value",
                        "type": "unconditional"
                    },
                    "reroutes": [
                        {
                            "x": 9.99,
                            "y": 9.99
                        }
                    ]
                }
            },
            "nodes": {
                "key": {
                    "type": "start",
                    "position": {
                        "x": 9.99,
                        "y": 9.99
                    },
                    "edge_order": [
                        "value"
                    ]
                }
            },
            "prevent_subagent_loops": true
        },
        "include_draft": true
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agents/value/branches"))
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
    let body = serde_json::from_value(request_body.clone()).unwrap();

    let result = client.branches.create("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/agents/value/branches"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/agents/value/branches request"
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
    let sent_body: serde_json::Value =
        serde_json::from_slice(&request.body).expect("request body should be JSON");
    assert_eq!(sent_body, request_body);
    let decoded = result.expect("call should succeed");
    assert_eq!(
        serde_json::to_value(&decoded).expect("response should serialize"),
        expected_response
    );
}

#[tokio::test]
async fn create_surfaces_the_declared_422() {
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
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "parent_version_id": "value",
        "name": "value",
        "description": "value",
        "conversation_config": {},
        "platform_settings": {},
        "workflow": {
            "edges": {
                "key": {
                    "source": "value",
                    "target": "value",
                    "forward_condition": {
                        "label": "value",
                        "type": "unconditional"
                    },
                    "backward_condition": {
                        "label": "value",
                        "type": "unconditional"
                    },
                    "reroutes": [
                        {
                            "x": 9.99,
                            "y": 9.99
                        }
                    ]
                }
            },
            "nodes": {
                "key": {
                    "type": "start",
                    "position": {
                        "x": 9.99,
                        "y": 9.99
                    },
                    "edge_order": [
                        "value"
                    ]
                }
            },
            "prevent_subagent_loops": true
        },
        "include_draft": true
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agents/value/branches"))
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
    let body = serde_json::from_value(request_body.clone()).unwrap();

    let result = client.branches.create("value", &body, None).await;

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

#[tokio::test]
async fn get_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "id": "value",
        "name": "value",
        "agent_id": "value",
        "description": "value",
        "created_at": 123,
        "last_committed_at": 123,
        "is_archived": true,
        "protection_status": "writer_perms_required",
        "auto_rebase": true,
        "sync_status": {
            "branch_id": "value",
            "last_attempt": {
                "outcome": "succeeded",
                "attempted_at": 123
            }
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "current_live_percentage": 9.99,
        "parent_branch": {
            "id": "value",
            "name": "value"
        },
        "most_recent_versions": [
            {
                "id": "value",
                "agent_id": "value",
                "branch_id": "value",
                "version_description": "value",
                "seq_no_in_branch": 123,
                "time_committed_secs": 123,
                "parents": {
                    "in_branch_parent_id": "value",
                    "out_of_branch_parent_id": "value",
                    "merged_into_branch_id": "value",
                    "merged_from_branch_id": "value",
                    "merged_from_version_id": "value",
                    "rebased_from_version_id": "value"
                },
                "access_info": {
                    "is_creator": true,
                    "creator_name": "value",
                    "creator_email": "value",
                    "role": "admin",
                    "anonymous_access_level_override": "admin",
                    "access_source": "creator"
                },
                "merged_authors": [
                    {
                        "is_creator": true,
                        "creator_name": "value",
                        "creator_email": "value",
                        "role": "admin",
                        "anonymous_access_level_override": "admin",
                        "access_source": "creator"
                    }
                ]
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/agents/value/branches/value"))
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

    let result = client.branches.get("value", "value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/convai/agents/value/branches/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/agents/value/branches/value request"
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
async fn get_surfaces_the_declared_422() {
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

    Mock::given(method("GET"))
        .and(path("/v1/convai/agents/value/branches/value"))
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

    let result = client.branches.get("value", "value", None).await;

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

#[tokio::test]
async fn update_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "id": "value",
        "name": "value",
        "agent_id": "value",
        "description": "value",
        "created_at": 123,
        "last_committed_at": 123,
        "is_archived": true,
        "protection_status": "writer_perms_required",
        "auto_rebase": true,
        "sync_status": {
            "branch_id": "value",
            "last_attempt": {
                "outcome": "succeeded",
                "attempted_at": 123
            }
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "current_live_percentage": 9.99,
        "parent_branch": {
            "id": "value",
            "name": "value"
        },
        "most_recent_versions": [
            {
                "id": "value",
                "agent_id": "value",
                "branch_id": "value",
                "version_description": "value",
                "seq_no_in_branch": 123,
                "time_committed_secs": 123,
                "parents": {
                    "in_branch_parent_id": "value",
                    "out_of_branch_parent_id": "value",
                    "merged_into_branch_id": "value",
                    "merged_from_branch_id": "value",
                    "merged_from_version_id": "value",
                    "rebased_from_version_id": "value"
                },
                "access_info": {
                    "is_creator": true,
                    "creator_name": "value",
                    "creator_email": "value",
                    "role": "admin",
                    "anonymous_access_level_override": "admin",
                    "access_source": "creator"
                },
                "merged_authors": [
                    {
                        "is_creator": true,
                        "creator_name": "value",
                        "creator_email": "value",
                        "role": "admin",
                        "anonymous_access_level_override": "admin",
                        "access_source": "creator"
                    }
                ]
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "name": "value",
        "description": "value",
        "is_archived": true,
        "protection_status": "writer_perms_required",
        "auto_rebase": true,
        "sync_branch_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/value/branches/value"))
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
    let body = serde_json::from_value(request_body.clone()).unwrap();

    let result = client
        .branches
        .update("value", "value", Some(&body), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH"
                && entry.url.path() == "/v1/convai/agents/value/branches/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/convai/agents/value/branches/value request"
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
    let sent_body: serde_json::Value =
        serde_json::from_slice(&request.body).expect("request body should be JSON");
    assert_eq!(sent_body, request_body);
    let decoded = result.expect("call should succeed");
    assert_eq!(
        serde_json::to_value(&decoded).expect("response should serialize"),
        expected_response
    );
}

#[tokio::test]
async fn update_surfaces_the_declared_422() {
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
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "name": "value",
        "description": "value",
        "is_archived": true,
        "protection_status": "writer_perms_required",
        "auto_rebase": true,
        "sync_branch_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/value/branches/value"))
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
    let body = serde_json::from_value(request_body.clone()).unwrap();

    let result = client
        .branches
        .update("value", "value", Some(&body), None)
        .await;

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
