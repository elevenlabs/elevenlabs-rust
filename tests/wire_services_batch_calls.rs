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
        "phone_number_id": "value",
        "phone_provider": "twilio",
        "whatsapp_params": {
            "whatsapp_phone_number_id": "value",
            "whatsapp_call_permission_request_template_name": "value",
            "whatsapp_call_permission_request_template_language_code": "value"
        },
        "name": "value",
        "agent_id": "value",
        "branch_id": "value",
        "environment": "value",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "timezone": "value",
        "total_calls_dispatched": 123,
        "total_calls_scheduled": 123,
        "total_calls_finished": 123,
        "last_updated_at_unix": 123,
        "status": "pending",
        "retry_count": 123,
        "telephony_call_config": {
            "ringing_timeout_secs": 123,
            "twilio_call_recording_enabled": true,
            "twilio_machine_detection": {
                "mode": "enable",
                "actions": [
                    {
                        "action": "hang_up",
                        "answered_by": [
                            "human"
                        ]
                    }
                ]
            }
        },
        "target_concurrency_limit": 123,
        "agent_name": "value",
        "branch_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "call_name": "value",
        "agent_id": "value",
        "recipients": [
            {
                "id": "value",
                "phone_number": "value",
                "whatsapp_user_id": "value",
                "conversation_initiation_client_data": {
                    "conversation_config_override": {
                        "asr": {
                            "keywords": [
                                "value"
                            ]
                        },
                        "turn": {
                            "soft_timeout_config": {
                                "message": "value",
                                "additional_soft_timeout_messages": [
                                    "value"
                                ]
                            },
                            "noisy_environment_message": "value"
                        },
                        "tts": {
                            "model_id": "eleven_turbo_v2",
                            "voice_id": "value",
                            "supported_voices": [
                                {
                                    "label": "value",
                                    "voice_id": "value",
                                    "description": "value",
                                    "language": "value",
                                    "model_family": "turbo",
                                    "optimize_streaming_latency": 0,
                                    "stability": 0.5,
                                    "speed": 1.2,
                                    "similarity_boost": 0.5,
                                    "clone_strength": 0.5
                                }
                            ],
                            "stability": 9.99,
                            "speed": 9.99,
                            "similarity_boost": 9.99,
                            "clone_strength": 9.99,
                            "pronunciation_dictionary_locators": [
                                {
                                    "pronunciation_dictionary_id": "value",
                                    "version_id": "value"
                                }
                            ]
                        },
                        "conversation": {
                            "text_only": true,
                            "max_duration_seconds": 123
                        },
                        "agent": {
                            "first_message": "value",
                            "language": "value",
                            "max_conversation_duration_message": "value",
                            "prompt": {
                                "prompt": "value",
                                "llm": "gpt-4o-mini",
                                "reasoning_effort": "none",
                                "tool_ids": [
                                    "value"
                                ],
                                "tool_ids_strict": true,
                                "native_mcp_server_ids": [
                                    "value"
                                ],
                                "knowledge_base": [
                                    {
                                        "type": "file",
                                        "name": "value",
                                        "id": "value",
                                        "usage_mode": "prompt"
                                    }
                                ]
                            }
                        }
                    },
                    "custom_llm_extra_body": {},
                    "user_id": "value",
                    "source_info": {
                        "source": "unknown",
                        "version": "value"
                    },
                    "branch_id": "value",
                    "environment": "value",
                    "starting_workflow_node_id": "value",
                    "procedure_ids": [
                        "value"
                    ],
                    "start_procedure_ids": [
                        "value"
                    ],
                    "dynamic_variables": {}
                }
            }
        ],
        "scheduled_time_unix": 123,
        "agent_phone_number_id": "value",
        "whatsapp_params": {
            "whatsapp_phone_number_id": "value",
            "whatsapp_call_permission_request_template_name": "value",
            "whatsapp_call_permission_request_template_language_code": "value"
        },
        "timezone": "value",
        "branch_id": "value",
        "environment": "value",
        "telephony_call_config": {
            "ringing_timeout_secs": 123,
            "twilio_call_recording_enabled": true,
            "twilio_machine_detection": {
                "mode": "enable",
                "actions": [
                    {
                        "action": "hang_up",
                        "answered_by": [
                            "human"
                        ]
                    }
                ]
            }
        },
        "target_concurrency_limit": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/batch-calling/submit"))
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

    let result = client.batch_calls.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST" && entry.url.path() == "/v1/convai/batch-calling/submit"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/batch-calling/submit request"
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
        "call_name": "value",
        "agent_id": "value",
        "recipients": [
            {
                "id": "value",
                "phone_number": "value",
                "whatsapp_user_id": "value",
                "conversation_initiation_client_data": {
                    "conversation_config_override": {
                        "asr": {
                            "keywords": [
                                "value"
                            ]
                        },
                        "turn": {
                            "soft_timeout_config": {
                                "message": "value",
                                "additional_soft_timeout_messages": [
                                    "value"
                                ]
                            },
                            "noisy_environment_message": "value"
                        },
                        "tts": {
                            "model_id": "eleven_turbo_v2",
                            "voice_id": "value",
                            "supported_voices": [
                                {
                                    "label": "value",
                                    "voice_id": "value",
                                    "description": "value",
                                    "language": "value",
                                    "model_family": "turbo",
                                    "optimize_streaming_latency": 0,
                                    "stability": 0.5,
                                    "speed": 1.2,
                                    "similarity_boost": 0.5,
                                    "clone_strength": 0.5
                                }
                            ],
                            "stability": 9.99,
                            "speed": 9.99,
                            "similarity_boost": 9.99,
                            "clone_strength": 9.99,
                            "pronunciation_dictionary_locators": [
                                {
                                    "pronunciation_dictionary_id": "value",
                                    "version_id": "value"
                                }
                            ]
                        },
                        "conversation": {
                            "text_only": true,
                            "max_duration_seconds": 123
                        },
                        "agent": {
                            "first_message": "value",
                            "language": "value",
                            "max_conversation_duration_message": "value",
                            "prompt": {
                                "prompt": "value",
                                "llm": "gpt-4o-mini",
                                "reasoning_effort": "none",
                                "tool_ids": [
                                    "value"
                                ],
                                "tool_ids_strict": true,
                                "native_mcp_server_ids": [
                                    "value"
                                ],
                                "knowledge_base": [
                                    {
                                        "type": "file",
                                        "name": "value",
                                        "id": "value",
                                        "usage_mode": "prompt"
                                    }
                                ]
                            }
                        }
                    },
                    "custom_llm_extra_body": {},
                    "user_id": "value",
                    "source_info": {
                        "source": "unknown",
                        "version": "value"
                    },
                    "branch_id": "value",
                    "environment": "value",
                    "starting_workflow_node_id": "value",
                    "procedure_ids": [
                        "value"
                    ],
                    "start_procedure_ids": [
                        "value"
                    ],
                    "dynamic_variables": {}
                }
            }
        ],
        "scheduled_time_unix": 123,
        "agent_phone_number_id": "value",
        "whatsapp_params": {
            "whatsapp_phone_number_id": "value",
            "whatsapp_call_permission_request_template_name": "value",
            "whatsapp_call_permission_request_template_language_code": "value"
        },
        "timezone": "value",
        "branch_id": "value",
        "environment": "value",
        "telephony_call_config": {
            "ringing_timeout_secs": 123,
            "twilio_call_recording_enabled": true,
            "twilio_machine_detection": {
                "mode": "enable",
                "actions": [
                    {
                        "action": "hang_up",
                        "answered_by": [
                            "human"
                        ]
                    }
                ]
            }
        },
        "target_concurrency_limit": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/batch-calling/submit"))
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

    let result = client.batch_calls.create(&body, None).await;

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
        "phone_number_id": "value",
        "phone_provider": "twilio",
        "whatsapp_params": {
            "whatsapp_phone_number_id": "value",
            "whatsapp_call_permission_request_template_name": "value",
            "whatsapp_call_permission_request_template_language_code": "value"
        },
        "name": "value",
        "agent_id": "value",
        "branch_id": "value",
        "environment": "value",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "timezone": "value",
        "total_calls_dispatched": 123,
        "total_calls_scheduled": 123,
        "total_calls_finished": 123,
        "last_updated_at_unix": 123,
        "status": "pending",
        "retry_count": 123,
        "telephony_call_config": {
            "ringing_timeout_secs": 123,
            "twilio_call_recording_enabled": true,
            "twilio_machine_detection": {
                "mode": "enable",
                "actions": [
                    {
                        "action": "hang_up",
                        "answered_by": [
                            "human"
                        ]
                    }
                ]
            }
        },
        "target_concurrency_limit": 123,
        "agent_name": "value",
        "branch_name": "value",
        "recipients": [
            {
                "id": "value",
                "phone_number": "value",
                "whatsapp_user_id": "value",
                "status": "pending",
                "created_at_unix": 123,
                "updated_at_unix": 123,
                "conversation_id": "value",
                "conversation_initiation_client_data": {
                    "conversation_config_override": {
                        "asr": {
                            "keywords": [
                                "value"
                            ]
                        },
                        "turn": {
                            "soft_timeout_config": {
                                "message": "value",
                                "additional_soft_timeout_messages": [
                                    "value"
                                ]
                            },
                            "noisy_environment_message": "value"
                        },
                        "tts": {
                            "model_id": "eleven_turbo_v2",
                            "voice_id": "value",
                            "supported_voices": [
                                {
                                    "label": "value",
                                    "voice_id": "value",
                                    "description": "value",
                                    "language": "value",
                                    "model_family": "turbo",
                                    "optimize_streaming_latency": 0,
                                    "stability": 0.5,
                                    "speed": 1.2,
                                    "similarity_boost": 0.5,
                                    "clone_strength": 0.5
                                }
                            ],
                            "stability": 9.99,
                            "speed": 9.99,
                            "similarity_boost": 9.99,
                            "clone_strength": 9.99,
                            "pronunciation_dictionary_locators": [
                                {
                                    "pronunciation_dictionary_id": "value",
                                    "version_id": "value"
                                }
                            ]
                        },
                        "conversation": {
                            "text_only": true,
                            "max_duration_seconds": 123
                        },
                        "agent": {
                            "first_message": "value",
                            "language": "value",
                            "max_conversation_duration_message": "value",
                            "prompt": {
                                "prompt": "value",
                                "llm": "gpt-4o-mini",
                                "reasoning_effort": "none",
                                "tool_ids": [
                                    "value"
                                ],
                                "tool_ids_strict": true,
                                "native_mcp_server_ids": [
                                    "value"
                                ],
                                "knowledge_base": [
                                    {
                                        "type": "file",
                                        "name": "value",
                                        "id": "value",
                                        "usage_mode": "prompt"
                                    }
                                ]
                            }
                        }
                    },
                    "custom_llm_extra_body": {},
                    "user_id": "value",
                    "source_info": {
                        "source": "unknown",
                        "version": "value"
                    },
                    "branch_id": "value",
                    "environment": "value",
                    "starting_workflow_node_id": "value",
                    "procedure_ids": [
                        "value"
                    ],
                    "start_procedure_ids": [
                        "value"
                    ],
                    "dynamic_variables": {},
                    "tool_mock_config": {
                        "mocking_strategy": "all",
                        "fallback_strategy": "call_real_tool",
                        "mocked_tool_names": [
                            "value"
                        ]
                    },
                    "tool_mock_overrides": {
                        "key": [
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
                }
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/batch-calling/value"))
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

    let result = client.batch_calls.get("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET" && entry.url.path() == "/v1/convai/batch-calling/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/batch-calling/value request"
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
        .and(path("/v1/convai/batch-calling/value"))
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

    let result = client.batch_calls.get("value", None).await;

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
        .and(path("/v1/convai/batch-calling/value"))
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

    let result = client.batch_calls.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE"
                && entry.url.path() == "/v1/convai/batch-calling/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/convai/batch-calling/value request"
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
        .and(path("/v1/convai/batch-calling/value"))
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

    let result = client.batch_calls.delete("value", None).await;

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
async fn cancel_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "id": "value",
        "phone_number_id": "value",
        "phone_provider": "twilio",
        "whatsapp_params": {
            "whatsapp_phone_number_id": "value",
            "whatsapp_call_permission_request_template_name": "value",
            "whatsapp_call_permission_request_template_language_code": "value"
        },
        "name": "value",
        "agent_id": "value",
        "branch_id": "value",
        "environment": "value",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "timezone": "value",
        "total_calls_dispatched": 123,
        "total_calls_scheduled": 123,
        "total_calls_finished": 123,
        "last_updated_at_unix": 123,
        "status": "pending",
        "retry_count": 123,
        "telephony_call_config": {
            "ringing_timeout_secs": 123,
            "twilio_call_recording_enabled": true,
            "twilio_machine_detection": {
                "mode": "enable",
                "actions": [
                    {
                        "action": "hang_up",
                        "answered_by": [
                            "human"
                        ]
                    }
                ]
            }
        },
        "target_concurrency_limit": 123,
        "agent_name": "value",
        "branch_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/batch-calling/value/cancel"))
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

    let result = client.batch_calls.cancel("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/batch-calling/value/cancel"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/batch-calling/value/cancel request"
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
async fn cancel_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/batch-calling/value/cancel"))
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

    let result = client.batch_calls.cancel("value", None).await;

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
async fn retry_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "id": "value",
        "phone_number_id": "value",
        "phone_provider": "twilio",
        "whatsapp_params": {
            "whatsapp_phone_number_id": "value",
            "whatsapp_call_permission_request_template_name": "value",
            "whatsapp_call_permission_request_template_language_code": "value"
        },
        "name": "value",
        "agent_id": "value",
        "branch_id": "value",
        "environment": "value",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "timezone": "value",
        "total_calls_dispatched": 123,
        "total_calls_scheduled": 123,
        "total_calls_finished": 123,
        "last_updated_at_unix": 123,
        "status": "pending",
        "retry_count": 123,
        "telephony_call_config": {
            "ringing_timeout_secs": 123,
            "twilio_call_recording_enabled": true,
            "twilio_machine_detection": {
                "mode": "enable",
                "actions": [
                    {
                        "action": "hang_up",
                        "answered_by": [
                            "human"
                        ]
                    }
                ]
            }
        },
        "target_concurrency_limit": 123,
        "agent_name": "value",
        "branch_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/batch-calling/value/retry"))
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

    let result = client.batch_calls.retry("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/batch-calling/value/retry"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/batch-calling/value/retry request"
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
async fn retry_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/batch-calling/value/retry"))
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

    let result = client.batch_calls.retry("value", None).await;

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
