use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_manual_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "title": "value",
        "qa_comment": "value",
        "priority": "low"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agents/value/triage-tickets"))
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
        .triage_tickets
        .create_manual("value", &body, None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/agents/value/triage-tickets"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/agents/value/triage-tickets request"
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
async fn create_manual_surfaces_the_declared_422() {
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
        "title": "value",
        "qa_comment": "value",
        "priority": "low"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agents/value/triage-tickets"))
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
        .triage_tickets
        .create_manual("value", &body, None)
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

#[tokio::test]
async fn create_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "conversation_id": "value",
        "title": "value",
        "qa_comment": "value",
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value"
            }
        ],
        "priority": "low"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets"))
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

    let result = client.triage_tickets.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST" && entry.url.path() == "/v1/convai/triage-tickets"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/triage-tickets request"
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
        "conversation_id": "value",
        "title": "value",
        "qa_comment": "value",
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value"
            }
        ],
        "priority": "low"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets"))
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

    let result = client.triage_tickets.create(&body, None).await;

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
async fn list_assignable_users_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"[
        {
            "user_id": "value",
            "email": "value",
            "first_name": "value",
            "is_service_account": true,
            "has_access": true
        }
    ]"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path(
            "/v1/convai/agents/value/triage-tickets/assignable-users",
        ))
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

    let result = client
        .triage_tickets
        .list_assignable_users("value", None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/convai/agents/value/triage-tickets/assignable-users"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/agents/value/triage-tickets/assignable-users request"
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
async fn list_assignable_users_surfaces_the_declared_422() {
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
        .and(path(
            "/v1/convai/agents/value/triage-tickets/assignable-users",
        ))
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

    let result = client
        .triage_tickets
        .list_assignable_users("value", None)
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

#[tokio::test]
async fn get_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/triage-tickets/value"))
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

    let result = client.triage_tickets.get("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET" && entry.url.path() == "/v1/convai/triage-tickets/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/triage-tickets/value request"
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
        .and(path("/v1/convai/triage-tickets/value"))
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

    let result = client.triage_tickets.get("value", None).await;

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
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "title": "value",
        "qa_comment": "value",
        "status": "open",
        "assignee_user_id": "value",
        "priority": "low"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/convai/triage-tickets/value"))
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

    let result = client.triage_tickets.update("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH"
                && entry.url.path() == "/v1/convai/triage-tickets/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/convai/triage-tickets/value request"
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
        "title": "value",
        "qa_comment": "value",
        "status": "open",
        "assignee_user_id": "value",
        "priority": "low"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/convai/triage-tickets/value"))
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

    let result = client.triage_tickets.update("value", &body, None).await;

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
async fn delete_sends_the_real_wire_request() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/convai/triage-tickets/value"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.triage_tickets.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE"
                && entry.url.path() == "/v1/convai/triage-tickets/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/convai/triage-tickets/value request"
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
        .and(path("/v1/convai/triage-tickets/value"))
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

    let result = client.triage_tickets.delete("value", None).await;

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
async fn add_comment_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "comment": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets/value/comments"))
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
        .triage_tickets
        .add_comment("value", &body, None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/triage-tickets/value/comments"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/triage-tickets/value/comments request"
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
async fn add_comment_surfaces_the_declared_422() {
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
        "comment": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets/value/comments"))
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
        .triage_tickets
        .add_comment("value", &body, None)
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

#[tokio::test]
async fn merge_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "source_ticket_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets/value/merge"))
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

    let result = client.triage_tickets.merge("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/triage-tickets/value/merge"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/triage-tickets/value/merge request"
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
async fn merge_surfaces_the_declared_422() {
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
        "source_ticket_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets/value/merge"))
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

    let result = client.triage_tickets.merge("value", &body, None).await;

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
async fn add_turn_comment_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agentqa_ticket_id": "value",
        "workspace_id": "value",
        "owner_user_id": "value",
        "agent_id": "value",
        "needs_clustering": true,
        "title": "value",
        "issue_type": "knowledge_gap",
        "labels": [
            "value"
        ],
        "conversation_ids": [
            "value"
        ],
        "first_seen_unix_secs": 123,
        "last_seen_unix_secs": 123,
        "qa_comment": "value",
        "ticket_comments": [
            {
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "turn_comments": [
            {
                "turn_index": 123,
                "comment": "value",
                "created_at_unix_secs": 123,
                "owner_user_id": "value"
            }
        ],
        "status": "open",
        "priority": "low",
        "priority_changes": [
            {
                "priority": "low",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "status_changes": [
            {
                "status": "open",
                "changed_by_user_id": "value",
                "changed_at_unix_secs": 123
            }
        ],
        "merged_into_ticket_id": "value",
        "source": "qa",
        "assignee_user_id": "value",
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "search_match": {
            "field": "title",
            "snippet": "value",
            "highlight_start": 123,
            "highlight_end": 123,
            "turn_index": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "turn_index": 123,
        "comment": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets/value/turn-comments"))
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
        .triage_tickets
        .add_turn_comment("value", &body, None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/triage-tickets/value/turn-comments"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/triage-tickets/value/turn-comments request"
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
async fn add_turn_comment_surfaces_the_declared_422() {
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
        "turn_index": 123,
        "comment": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/triage-tickets/value/turn-comments"))
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
        .triage_tickets
        .add_turn_comment("value", &body, None)
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
