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
        "id": "value",
        "subject": "value",
        "description": "value",
        "status": "open",
        "priority": "low",
        "requester": {
            "id": "value",
            "first_name": "value",
            "last_name": "value",
            "full_name": "value",
            "email": "value"
        },
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value",
            "name": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value",
            "name": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value",
                "name": "value",
                "email": "value"
            }
        ],
        "sla": {
            "metric": "first_response",
            "duration_secs": 123,
            "started_at_unix_secs": 123,
            "due_at_unix_secs": 123
        },
        "deadline_at_unix_secs": 123,
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "events": [
            {
                "id": "value",
                "visibility": "public",
                "author": {
                    "id": "value",
                    "first_name": "value",
                    "last_name": "value",
                    "full_name": "value",
                    "email": "value",
                    "type": "requester"
                },
                "created_at_unix_secs": 123,
                "type": "message",
                "body": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "requester": {
            "type": "user_id",
            "value": "value"
        },
        "author": {
            "type": "requester"
        },
        "subject": "value",
        "description": "value",
        "priority": "low",
        "deadline_at_unix_secs": 123,
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/support/tickets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(expected_response.clone()))
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

    let result = client.tickets.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST" && entry.url.path() == "/v1/support/tickets"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/support/tickets request"
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
        "requester": {
            "type": "user_id",
            "value": "value"
        },
        "author": {
            "type": "requester"
        },
        "subject": "value",
        "description": "value",
        "priority": "low",
        "deadline_at_unix_secs": 123,
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/support/tickets"))
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

    let result = client.tickets.create(&body, None).await;

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
async fn create_message_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "ticket_id": "value",
        "event": {
            "id": "value",
            "visibility": "public",
            "author": {
                "id": "value",
                "first_name": "value",
                "last_name": "value",
                "full_name": "value",
                "email": "value",
                "type": "requester"
            },
            "created_at_unix_secs": 123,
            "type": "message",
            "body": "value"
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "author": {
            "type": "requester"
        },
        "body": "value",
        "visibility": "public",
        "status": "open"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/support/tickets/value/messages"))
        .respond_with(ResponseTemplate::new(201).set_body_json(expected_response.clone()))
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

    let result = client.tickets.create_message("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/support/tickets/value/messages"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/support/tickets/value/messages request"
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
async fn create_message_surfaces_the_declared_422() {
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
        "author": {
            "type": "requester"
        },
        "body": "value",
        "visibility": "public",
        "status": "open"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/support/tickets/value/messages"))
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

    let result = client.tickets.create_message("value", &body, None).await;

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
        "subject": "value",
        "description": "value",
        "status": "open",
        "priority": "low",
        "requester": {
            "id": "value",
            "first_name": "value",
            "last_name": "value",
            "full_name": "value",
            "email": "value"
        },
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value",
            "name": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value",
            "name": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value",
                "name": "value",
                "email": "value"
            }
        ],
        "sla": {
            "metric": "first_response",
            "duration_secs": 123,
            "started_at_unix_secs": 123,
            "due_at_unix_secs": 123
        },
        "deadline_at_unix_secs": 123,
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "events": [
            {
                "id": "value",
                "visibility": "public",
                "author": {
                    "id": "value",
                    "first_name": "value",
                    "last_name": "value",
                    "full_name": "value",
                    "email": "value",
                    "type": "requester"
                },
                "created_at_unix_secs": 123,
                "type": "message",
                "body": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/support/tickets/value"))
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

    let result = client.tickets.get("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET" && entry.url.path() == "/v1/support/tickets/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/support/tickets/value request"
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
        .and(path("/v1/support/tickets/value"))
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

    let result = client.tickets.get("value", None).await;

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
        "subject": "value",
        "description": "value",
        "status": "open",
        "priority": "low",
        "requester": {
            "id": "value",
            "first_name": "value",
            "last_name": "value",
            "full_name": "value",
            "email": "value"
        },
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value",
            "name": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value",
            "name": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value",
                "name": "value",
                "email": "value"
            }
        ],
        "sla": {
            "metric": "first_response",
            "duration_secs": 123,
            "started_at_unix_secs": 123,
            "due_at_unix_secs": 123
        },
        "deadline_at_unix_secs": 123,
        "created_at_unix_secs": 123,
        "updated_at_unix_secs": 123,
        "events": [
            {
                "id": "value",
                "visibility": "public",
                "author": {
                    "id": "value",
                    "first_name": "value",
                    "last_name": "value",
                    "full_name": "value",
                    "email": "value",
                    "type": "requester"
                },
                "created_at_unix_secs": 123,
                "type": "message",
                "body": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "subject": "value",
        "status": "open",
        "priority": "low",
        "deadline_at_unix_secs": 123,
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/support/tickets/value"))
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

    let result = client.tickets.update("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH" && entry.url.path() == "/v1/support/tickets/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/support/tickets/value request"
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
        "subject": "value",
        "status": "open",
        "priority": "low",
        "deadline_at_unix_secs": 123,
        "tag_ids": [
            "value"
        ],
        "assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "secondary_assignee": {
            "type": "ai_agent",
            "id": "value"
        },
        "subscribers": [
            {
                "type": "human_agent",
                "id": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/support/tickets/value"))
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

    let result = client.tickets.update("value", &body, None).await;

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
        .and(path("/v1/support/tickets/value"))
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

    let result = client.tickets.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE" && entry.url.path() == "/v1/support/tickets/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/support/tickets/value request"
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
        .and(path("/v1/support/tickets/value"))
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

    let result = client.tickets.delete("value", None).await;

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
