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
        "variants": [
            {
                "id": "value",
                "duration_minutes": 123,
                "price": 9.99,
                "is_active": true
            }
        ],
        "kind": "classic",
        "buffer_minutes": 123,
        "service_location": "business",
        "add_ons": [
            {
                "id": "value",
                "name": "value",
                "description": "value",
                "add_duration_minutes": 123,
                "add_price": 9.99
            }
        ],
        "max_participants": 123,
        "capacity": 123,
        "is_active": true,
        "any_staff_member": true,
        "staff_ids": [
            "value"
        ],
        "asset_ids": [
            "value"
        ],
        "advance_booking_minutes": 123,
        "location_ids": [
            "value"
        ],
        "display_on_booking_page": true,
        "travel_minutes": 123,
        "travel_before": true,
        "travel_after": true,
        "promotions": [
            {
                "id": "value",
                "name": "value",
                "target": "service",
                "variant_id": "value",
                "add_on_id": "value",
                "promotional_price": 9.99,
                "days_of_week": [
                    6
                ],
                "start_date": "2024-01-01",
                "end_date": "2024-01-01",
                "is_active": true
            }
        ],
        "id": "value",
        "created_at_unix": 123,
        "updated_at_unix": 123,
        "duration_minutes": 123,
        "price": 9.99
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
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
        "variants": [
            {
                "id": "value",
                "duration_minutes": 123,
                "price": 9.99,
                "is_active": true
            }
        ],
        "kind": "classic",
        "buffer_minutes": 123,
        "service_location": "business",
        "add_ons": [
            {
                "id": "value",
                "name": "value",
                "description": "value",
                "add_duration_minutes": 123,
                "add_price": 9.99
            }
        ],
        "max_participants": 123,
        "capacity": 123,
        "is_active": true,
        "any_staff_member": true,
        "staff_ids": [
            "value"
        ],
        "asset_ids": [
            "value"
        ],
        "advance_booking_minutes": 123,
        "location_ids": [
            "value"
        ],
        "display_on_booking_page": true,
        "travel_minutes": 123,
        "travel_before": true,
        "travel_after": true,
        "promotions": [
            {
                "id": "value",
                "name": "value",
                "target": "service",
                "variant_id": "value",
                "add_on_id": "value",
                "promotional_price": 9.99,
                "days_of_week": [
                    6
                ],
                "start_date": "2024-01-01",
                "end_date": "2024-01-01",
                "is_active": true
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/reception/services"))
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
        .services
        .create(&body, "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST" && entry.url.path() == "/v1/reception/services"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/reception/services request"
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
        "variants": [
            {
                "id": "value",
                "duration_minutes": 123,
                "price": 9.99,
                "is_active": true
            }
        ],
        "kind": "classic",
        "buffer_minutes": 123,
        "service_location": "business",
        "add_ons": [
            {
                "id": "value",
                "name": "value",
                "description": "value",
                "add_duration_minutes": 123,
                "add_price": 9.99
            }
        ],
        "max_participants": 123,
        "capacity": 123,
        "is_active": true,
        "any_staff_member": true,
        "staff_ids": [
            "value"
        ],
        "asset_ids": [
            "value"
        ],
        "advance_booking_minutes": 123,
        "location_ids": [
            "value"
        ],
        "display_on_booking_page": true,
        "travel_minutes": 123,
        "travel_before": true,
        "travel_after": true,
        "promotions": [
            {
                "id": "value",
                "name": "value",
                "target": "service",
                "variant_id": "value",
                "add_on_id": "value",
                "promotional_price": 9.99,
                "days_of_week": [
                    6
                ],
                "start_date": "2024-01-01",
                "end_date": "2024-01-01",
                "is_active": true
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/reception/services"))
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
        .services
        .create(&body, "value".to_string(), None)
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
        "variants": [
            {
                "id": "value",
                "duration_minutes": 123,
                "price": 9.99,
                "is_active": true
            }
        ],
        "kind": "classic",
        "buffer_minutes": 123,
        "service_location": "business",
        "add_ons": [
            {
                "id": "value",
                "name": "value",
                "description": "value",
                "add_duration_minutes": 123,
                "add_price": 9.99
            }
        ],
        "max_participants": 123,
        "capacity": 123,
        "is_active": true,
        "any_staff_member": true,
        "staff_ids": [
            "value"
        ],
        "asset_ids": [
            "value"
        ],
        "advance_booking_minutes": 123,
        "location_ids": [
            "value"
        ],
        "display_on_booking_page": true,
        "travel_minutes": 123,
        "travel_before": true,
        "travel_after": true,
        "promotions": [
            {
                "id": "value",
                "name": "value",
                "target": "service",
                "variant_id": "value",
                "add_on_id": "value",
                "promotional_price": 9.99,
                "days_of_week": [
                    6
                ],
                "start_date": "2024-01-01",
                "end_date": "2024-01-01",
                "is_active": true
            }
        ],
        "id": "value",
        "created_at_unix": 123,
        "updated_at_unix": 123,
        "duration_minutes": 123,
        "price": 9.99
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/reception/services/value"))
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
        .services
        .get("value", "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET" && entry.url.path() == "/v1/reception/services/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/reception/services/value request"
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
        .and(path("/v1/reception/services/value"))
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
        .services
        .get("value", "value".to_string(), None)
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
async fn update_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "service": {
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
            "variants": [
                {
                    "id": "value",
                    "duration_minutes": 123,
                    "price": 9.99,
                    "is_active": true
                }
            ],
            "kind": "classic",
            "buffer_minutes": 123,
            "service_location": "business",
            "add_ons": [
                {
                    "id": "value",
                    "name": "value",
                    "description": "value",
                    "add_duration_minutes": 123,
                    "add_price": 9.99
                }
            ],
            "max_participants": 123,
            "capacity": 123,
            "is_active": true,
            "any_staff_member": true,
            "staff_ids": [
                "value"
            ],
            "asset_ids": [
                "value"
            ],
            "advance_booking_minutes": 123,
            "location_ids": [
                "value"
            ],
            "display_on_booking_page": true,
            "travel_minutes": 123,
            "travel_before": true,
            "travel_after": true,
            "promotions": [
                {
                    "id": "value",
                    "name": "value",
                    "target": "service",
                    "variant_id": "value",
                    "add_on_id": "value",
                    "promotional_price": 9.99,
                    "days_of_week": [
                        6
                    ],
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "is_active": true
                }
            ],
            "id": "value",
            "created_at_unix": 123,
            "updated_at_unix": 123,
            "duration_minutes": 123,
            "price": 9.99
        },
        "has_affected_events": true
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
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
        "variants": [
            {
                "id": "value",
                "duration_minutes": 123,
                "price": 9.99,
                "is_active": true
            }
        ],
        "kind": "classic",
        "buffer_minutes": 123,
        "service_location": "business",
        "add_ons": [
            {
                "id": "value",
                "name": "value",
                "description": "value",
                "add_duration_minutes": 123,
                "add_price": 9.99
            }
        ],
        "max_participants": 123,
        "capacity": 123,
        "is_active": true,
        "any_staff_member": true,
        "staff_ids": [
            "value"
        ],
        "asset_ids": [
            "value"
        ],
        "advance_booking_minutes": 123,
        "location_ids": [
            "value"
        ],
        "display_on_booking_page": true,
        "travel_minutes": 123,
        "travel_before": true,
        "travel_after": true,
        "promotions": [
            {
                "id": "value",
                "name": "value",
                "target": "service",
                "variant_id": "value",
                "add_on_id": "value",
                "promotional_price": 9.99,
                "days_of_week": [
                    6
                ],
                "start_date": "2024-01-01",
                "end_date": "2024-01-01",
                "is_active": true
            }
        ],
        "duration_minutes": 123,
        "price": 9.99
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PUT"))
        .and(path("/v1/reception/services/value"))
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
        .services
        .update("value", &body, "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PUT" && entry.url.path() == "/v1/reception/services/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PUT /v1/reception/services/value request"
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
        "variants": [
            {
                "id": "value",
                "duration_minutes": 123,
                "price": 9.99,
                "is_active": true
            }
        ],
        "kind": "classic",
        "buffer_minutes": 123,
        "service_location": "business",
        "add_ons": [
            {
                "id": "value",
                "name": "value",
                "description": "value",
                "add_duration_minutes": 123,
                "add_price": 9.99
            }
        ],
        "max_participants": 123,
        "capacity": 123,
        "is_active": true,
        "any_staff_member": true,
        "staff_ids": [
            "value"
        ],
        "asset_ids": [
            "value"
        ],
        "advance_booking_minutes": 123,
        "location_ids": [
            "value"
        ],
        "display_on_booking_page": true,
        "travel_minutes": 123,
        "travel_before": true,
        "travel_after": true,
        "promotions": [
            {
                "id": "value",
                "name": "value",
                "target": "service",
                "variant_id": "value",
                "add_on_id": "value",
                "promotional_price": 9.99,
                "days_of_week": [
                    6
                ],
                "start_date": "2024-01-01",
                "end_date": "2024-01-01",
                "is_active": true
            }
        ],
        "duration_minutes": 123,
        "price": 9.99
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PUT"))
        .and(path("/v1/reception/services/value"))
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
        .services
        .update("value", &body, "value".to_string(), None)
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
async fn delete_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "key": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/reception/services/value"))
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
        .services
        .delete("value", "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE" && entry.url.path() == "/v1/reception/services/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/reception/services/value request"
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
        .and(path("/v1/reception/services/value"))
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
        .services
        .delete("value", "value".to_string(), None)
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
