use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn set_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "pricing_mode": "fixed",
        "intake_questions": [
            {
                "id": "value",
                "label": "value",
                "kind": "text",
                "options": [
                    "value"
                ],
                "required": true,
                "is_active": true
            }
        ],
        "groups": [
            {
                "label": "value",
                "rank": 123
            }
        ],
        "ungrouped_rank": 123,
        "name": "value",
        "description": "value",
        "price": 9.99,
        "is_active": true,
        "orderable": true,
        "allowed_fulfillment_types": [
            "pickup"
        ],
        "location_ids": [
            "value"
        ],
        "stock": {
            "key": 123
        },
        "id": "value",
        "created_at_unix": 123,
        "updated_at_unix": 123
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "location_id": "value",
        "on_hand": 123,
        "expected_on_hand": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/reception/products/value/stock"))
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
        .stock
        .set("value", &body, "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH"
                && entry.url.path() == "/v1/reception/products/value/stock"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/reception/products/value/stock request"
    );
    let request = matched[0];
    assert_eq!(
        request
            .headers
            .get("xi-workspace-id")
            .map(|value| value.to_str().unwrap_or_default()),
        Some("value"),
        "header xi-workspace-id"
    );
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
async fn set_surfaces_the_declared_422() {
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
        "location_id": "value",
        "on_hand": 123,
        "expected_on_hand": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/reception/products/value/stock"))
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
        .stock
        .set("value", &body, "value".to_string(), None)
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
async fn adjust_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "pricing_mode": "fixed",
        "intake_questions": [
            {
                "id": "value",
                "label": "value",
                "kind": "text",
                "options": [
                    "value"
                ],
                "required": true,
                "is_active": true
            }
        ],
        "groups": [
            {
                "label": "value",
                "rank": 123
            }
        ],
        "ungrouped_rank": 123,
        "name": "value",
        "description": "value",
        "price": 9.99,
        "is_active": true,
        "orderable": true,
        "allowed_fulfillment_types": [
            "pickup"
        ],
        "location_ids": [
            "value"
        ],
        "stock": {
            "key": 123
        },
        "id": "value",
        "created_at_unix": 123,
        "updated_at_unix": 123
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "location_id": "value",
        "delta": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/reception/products/value/stock/adjust"))
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
        .stock
        .adjust("value", &body, "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/reception/products/value/stock/adjust"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/reception/products/value/stock/adjust request"
    );
    let request = matched[0];
    assert_eq!(
        request
            .headers
            .get("xi-workspace-id")
            .map(|value| value.to_str().unwrap_or_default()),
        Some("value"),
        "header xi-workspace-id"
    );
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
async fn adjust_surfaces_the_declared_422() {
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
        "location_id": "value",
        "delta": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/reception/products/value/stock/adjust"))
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
        .stock
        .adjust("value", &body, "value".to_string(), None)
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
