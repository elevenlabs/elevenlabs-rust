use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"[
        {
            "agent_id": "value",
            "config": {
                "additional_instructions": "value",
                "tone_traits": [
                    "value"
                ],
                "details_to_collect": [
                    "value"
                ],
                "llm": "gpt-4o-mini",
                "reasoning_effort": "none",
                "disable_rag": true,
                "background_voice_detection": true,
                "turn_eagerness": "patient",
                "voice_id": "value",
                "language": "value",
                "additional_languages": [
                    "value"
                ],
                "first_message": "value",
                "first_message_mode": "interruptible",
                "enable_end_call_tool": true,
                "enable_skip_turn_tool": true,
                "enable_play_keypad_touch_tone_tool": true,
                "agent_rules": [
                    {
                        "title": "value",
                        "description": "value",
                        "priority": "strict",
                        "id": "value",
                        "created_at_unix": 123
                    }
                ],
                "transfer_rules": [
                    {
                        "label": "value",
                        "phone_number": "value",
                        "condition": "value",
                        "post_dial_digits": "0",
                        "id": "value",
                        "created_at_unix": 123
                    }
                ],
                "hide_caller_number_on_transfers": true,
                "record_after_transfers": true,
                "procedures": [
                    {
                        "name": "value",
                        "trigger": "value",
                        "body": "value",
                        "id": "value",
                        "created_at_unix": 123,
                        "source_key": "value"
                    }
                ],
                "enable_secure_mode": true,
                "enable_spam_protection": true
            },
            "deactivated_at_unix": 123,
            "deactivation_reason": "tier_downgrade",
            "name": "value",
            "agent_exists": true,
            "is_transfer_allowed": true,
            "is_active": true
        }
    ]"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/reception/customer-facing-agents"))
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

    let result = client.receptionists.list("value".to_string(), None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/reception/customer-facing-agents"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/reception/customer-facing-agents request"
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
async fn list_surfaces_the_declared_422() {
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
        .and(path("/v1/reception/customer-facing-agents"))
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

    let result = client.receptionists.list("value".to_string(), None).await;

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
        "agent_id": "value",
        "config": {
            "additional_instructions": "value",
            "tone_traits": [
                "value"
            ],
            "details_to_collect": [
                "value"
            ],
            "llm": "gpt-4o-mini",
            "reasoning_effort": "none",
            "disable_rag": true,
            "background_voice_detection": true,
            "turn_eagerness": "patient",
            "voice_id": "value",
            "language": "value",
            "additional_languages": [
                "value"
            ],
            "first_message": "value",
            "first_message_mode": "interruptible",
            "enable_end_call_tool": true,
            "enable_skip_turn_tool": true,
            "enable_play_keypad_touch_tone_tool": true,
            "agent_rules": [
                {
                    "title": "value",
                    "description": "value",
                    "priority": "strict",
                    "id": "value",
                    "created_at_unix": 123
                }
            ],
            "transfer_rules": [
                {
                    "label": "value",
                    "phone_number": "value",
                    "condition": "value",
                    "post_dial_digits": "0",
                    "id": "value",
                    "created_at_unix": 123
                }
            ],
            "hide_caller_number_on_transfers": true,
            "record_after_transfers": true,
            "procedures": [
                {
                    "name": "value",
                    "trigger": "value",
                    "body": "value",
                    "id": "value",
                    "created_at_unix": 123,
                    "source_key": "value"
                }
            ],
            "enable_secure_mode": true,
            "enable_spam_protection": true
        },
        "deactivated_at_unix": 123,
        "deactivation_reason": "tier_downgrade",
        "name": "value",
        "agent_exists": true,
        "is_transfer_allowed": true,
        "is_active": true
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "config": {
            "additional_instructions": "value",
            "tone_traits": [
                "value"
            ],
            "details_to_collect": [
                "value"
            ],
            "llm": "gpt-4o-mini",
            "reasoning_effort": "none",
            "disable_rag": true,
            "background_voice_detection": true,
            "turn_eagerness": "patient",
            "voice_id": "value",
            "language": "value",
            "additional_languages": [
                "value"
            ],
            "first_message": "value",
            "first_message_mode": "interruptible",
            "enable_end_call_tool": true,
            "enable_skip_turn_tool": true,
            "enable_play_keypad_touch_tone_tool": true,
            "hide_caller_number_on_transfers": true,
            "record_after_transfers": true,
            "enable_secure_mode": true,
            "enable_spam_protection": true
        },
        "name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/reception/customer-facing-agents/value"))
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
        .receptionists
        .update("value", &body, "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH"
                && entry.url.path() == "/v1/reception/customer-facing-agents/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/reception/customer-facing-agents/value request"
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
        "config": {
            "additional_instructions": "value",
            "tone_traits": [
                "value"
            ],
            "details_to_collect": [
                "value"
            ],
            "llm": "gpt-4o-mini",
            "reasoning_effort": "none",
            "disable_rag": true,
            "background_voice_detection": true,
            "turn_eagerness": "patient",
            "voice_id": "value",
            "language": "value",
            "additional_languages": [
                "value"
            ],
            "first_message": "value",
            "first_message_mode": "interruptible",
            "enable_end_call_tool": true,
            "enable_skip_turn_tool": true,
            "enable_play_keypad_touch_tone_tool": true,
            "hide_caller_number_on_transfers": true,
            "record_after_transfers": true,
            "enable_secure_mode": true,
            "enable_spam_protection": true
        },
        "name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/reception/customer-facing-agents/value"))
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
        .receptionists
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
async fn activate_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agent_id": "value",
        "config": {
            "additional_instructions": "value",
            "tone_traits": [
                "value"
            ],
            "details_to_collect": [
                "value"
            ],
            "llm": "gpt-4o-mini",
            "reasoning_effort": "none",
            "disable_rag": true,
            "background_voice_detection": true,
            "turn_eagerness": "patient",
            "voice_id": "value",
            "language": "value",
            "additional_languages": [
                "value"
            ],
            "first_message": "value",
            "first_message_mode": "interruptible",
            "enable_end_call_tool": true,
            "enable_skip_turn_tool": true,
            "enable_play_keypad_touch_tone_tool": true,
            "agent_rules": [
                {
                    "title": "value",
                    "description": "value",
                    "priority": "strict",
                    "id": "value",
                    "created_at_unix": 123
                }
            ],
            "transfer_rules": [
                {
                    "label": "value",
                    "phone_number": "value",
                    "condition": "value",
                    "post_dial_digits": "0",
                    "id": "value",
                    "created_at_unix": 123
                }
            ],
            "hide_caller_number_on_transfers": true,
            "record_after_transfers": true,
            "procedures": [
                {
                    "name": "value",
                    "trigger": "value",
                    "body": "value",
                    "id": "value",
                    "created_at_unix": 123,
                    "source_key": "value"
                }
            ],
            "enable_secure_mode": true,
            "enable_spam_protection": true
        },
        "deactivated_at_unix": 123,
        "deactivation_reason": "tier_downgrade",
        "name": "value",
        "agent_exists": true,
        "is_transfer_allowed": true,
        "is_active": true
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/reception/customer-facing-agents/value/activate"))
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
        .receptionists
        .activate("value", "value".to_string(), None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/reception/customer-facing-agents/value/activate"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/reception/customer-facing-agents/value/activate request"
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
async fn activate_surfaces_the_declared_422() {
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

    Mock::given(method("POST"))
        .and(path("/v1/reception/customer-facing-agents/value/activate"))
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
        .receptionists
        .activate("value", "value".to_string(), None)
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
