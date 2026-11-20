use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn pause_all_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "paused_campaign_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/pause-all"))
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

    let result = client.outbound_campaigns.pause_all(None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/pause-all"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/outbound-campaigns/campaigns/pause-all request"
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
async fn pause_all_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/outbound-campaigns/campaigns/pause-all"))
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

    let result = client.outbound_campaigns.pause_all(None).await;

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
async fn stop_all_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "stopped_campaign_ids": [
            "value"
        ],
        "stopping_campaign_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/stop-all"))
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

    let result = client.outbound_campaigns.stop_all(None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/stop-all"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/outbound-campaigns/campaigns/stop-all request"
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
async fn stop_all_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/outbound-campaigns/campaigns/stop-all"))
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

    let result = client.outbound_campaigns.stop_all(None).await;

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
        "campaign": {
            "campaign_id": "value",
            "call_name": "value",
            "agent_id": "value",
            "channel_config": {
                "channel": "phone",
                "agent_phone_number_id": "value",
                "mode_config": {
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
                    },
                    "mode": "voice"
                }
            },
            "status": "active",
            "policy": {
                "timezone": "value",
                "retry": {
                    "retry_delay_seconds": 123,
                    "max_short_retries": 30,
                    "max_total_calls": 30,
                    "max_callbacks": 30,
                    "max_redirects": 30,
                    "interval_placement": "start",
                    "rotate_step_seconds": 123,
                    "max_consecutive_failures_per_number": 30
                },
                "calling_periods": [
                    {
                        "weekdays": [
                            123
                        ],
                        "intervals": []
                    }
                ],
                "date_overrides": [
                    {
                        "start_date": "2024-01-01",
                        "end_date": "2024-01-01",
                        "intervals": [],
                        "name": "value"
                    }
                ],
                "data_collection": {
                    "callback_time": "value",
                    "redirect_phone_number": "value",
                    "stop": "value",
                    "mappings": {
                        "key": "value"
                    }
                },
                "followup_context": {
                    "call_type": true,
                    "previous_conversation_id": true,
                    "previous_summary": true,
                    "previous_transcript": true
                }
            },
            "variant": "simple",
            "source": "ui",
            "created_at_unix": 123,
            "scheduled_time_unix": 123,
            "updated_at_unix": 123,
            "branch_id": "value",
            "environment": "value",
            "target_concurrency_limit": 123,
            "channel": "phone",
            "mode": "voice",
            "counts": {
                "leads": 123,
                "terminal_leads": 123,
                "open_attention": 123,
                "delivery": {
                    "sent": 123,
                    "delivered": 123,
                    "read": 123,
                    "replied": 123,
                    "failed": 123
                }
            },
            "agent_name": "value"
        },
        "created": true,
        "within_calling_window": true
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "call_name": "value",
        "agent_id": "value",
        "recipients": [
            {
                "phone_number": "value",
                "phone_numbers": [
                    "value"
                ],
                "whatsapp_user_id": "value",
                "id": "value",
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
        "timezone": "value",
        "branch_id": "value",
        "environment": "value",
        "target_concurrency_limit": 123,
        "idempotency_key": "value",
        "policy": {
            "timezone": "value",
            "retry": {
                "retry_delay_seconds": 123,
                "max_short_retries": 30,
                "max_total_calls": 30,
                "max_callbacks": 30,
                "max_redirects": 30,
                "interval_placement": "start",
                "rotate_step_seconds": 123,
                "max_consecutive_failures_per_number": 30
            },
            "calling_periods": [
                {
                    "weekdays": [
                        6
                    ],
                    "intervals": []
                }
            ],
            "date_overrides": [
                {
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "intervals": [],
                    "name": "value"
                }
            ],
            "data_collection": {
                "callback_time": "value",
                "redirect_phone_number": "value",
                "stop": "value",
                "mappings": {
                    "key": "value"
                }
            },
            "followup_context": {
                "call_type": true,
                "previous_conversation_id": true,
                "previous_summary": true,
                "previous_transcript": true
            }
        },
        "variant": "simple",
        "channel_config": {
            "channel": "phone",
            "agent_phone_number_id": "value",
            "mode_config": {
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
                },
                "mode": "voice"
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/submit"))
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

    let result = client.outbound_campaigns.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/submit"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/outbound-campaigns/campaigns/submit request"
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
                "phone_number": "value",
                "phone_numbers": [
                    "value"
                ],
                "whatsapp_user_id": "value",
                "id": "value",
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
        "timezone": "value",
        "branch_id": "value",
        "environment": "value",
        "target_concurrency_limit": 123,
        "idempotency_key": "value",
        "policy": {
            "timezone": "value",
            "retry": {
                "retry_delay_seconds": 123,
                "max_short_retries": 30,
                "max_total_calls": 30,
                "max_callbacks": 30,
                "max_redirects": 30,
                "interval_placement": "start",
                "rotate_step_seconds": 123,
                "max_consecutive_failures_per_number": 30
            },
            "calling_periods": [
                {
                    "weekdays": [
                        6
                    ],
                    "intervals": []
                }
            ],
            "date_overrides": [
                {
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "intervals": [],
                    "name": "value"
                }
            ],
            "data_collection": {
                "callback_time": "value",
                "redirect_phone_number": "value",
                "stop": "value",
                "mappings": {
                    "key": "value"
                }
            },
            "followup_context": {
                "call_type": true,
                "previous_conversation_id": true,
                "previous_summary": true,
                "previous_transcript": true
            }
        },
        "variant": "simple",
        "channel_config": {
            "channel": "phone",
            "agent_phone_number_id": "value",
            "mode_config": {
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
                },
                "mode": "voice"
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/submit"))
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

    let result = client.outbound_campaigns.create(&body, None).await;

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
        "campaign_id": "value",
        "call_name": "value",
        "agent_id": "value",
        "channel_config": {
            "channel": "phone",
            "agent_phone_number_id": "value",
            "mode_config": {
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
                },
                "mode": "voice"
            }
        },
        "status": "active",
        "policy": {
            "timezone": "value",
            "retry": {
                "retry_delay_seconds": 123,
                "max_short_retries": 30,
                "max_total_calls": 30,
                "max_callbacks": 30,
                "max_redirects": 30,
                "interval_placement": "start",
                "rotate_step_seconds": 123,
                "max_consecutive_failures_per_number": 30
            },
            "calling_periods": [
                {
                    "weekdays": [
                        123
                    ],
                    "intervals": []
                }
            ],
            "date_overrides": [
                {
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "intervals": [],
                    "name": "value"
                }
            ],
            "data_collection": {
                "callback_time": "value",
                "redirect_phone_number": "value",
                "stop": "value",
                "mappings": {
                    "key": "value"
                }
            },
            "followup_context": {
                "call_type": true,
                "previous_conversation_id": true,
                "previous_summary": true,
                "previous_transcript": true
            }
        },
        "variant": "simple",
        "source": "ui",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "updated_at_unix": 123,
        "branch_id": "value",
        "environment": "value",
        "target_concurrency_limit": 123,
        "channel": "phone",
        "mode": "voice",
        "counts": {
            "leads": 123,
            "terminal_leads": 123,
            "open_attention": 123,
            "delivery": {
                "sent": 123,
                "delivered": 123,
                "read": 123,
                "replied": 123,
                "failed": 123
            }
        },
        "agent_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/value"))
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

    let result = client.outbound_campaigns.get("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/outbound-campaigns/campaigns/value request"
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
        .and(path("/v1/convai/outbound-campaigns/campaigns/value"))
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

    let result = client.outbound_campaigns.get("value", None).await;

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
async fn get_progress_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "leads": {
            "total": 123,
            "uncalled": 123,
            "leads_needing_attention": 123,
            "active": 123,
            "finished": 123,
            "finished_by_disposition": {
                "key": 123
            }
        },
        "conversations": {
            "scheduled": 123,
            "scheduled_by_type": {
                "key": 123
            },
            "awaiting_permission": 123,
            "in_progress": 123,
            "awaiting_analysis": 123
        },
        "next_call_time_unix": 123
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path(
            "/v1/convai/outbound-campaigns/campaigns/value/progress",
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

    let result = client.outbound_campaigns.get_progress("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/value/progress"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/outbound-campaigns/campaigns/value/progress request"
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
async fn get_progress_surfaces_the_declared_422() {
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
            "/v1/convai/outbound-campaigns/campaigns/value/progress",
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

    let result = client.outbound_campaigns.get_progress("value", None).await;

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
async fn get_follow_up_requests_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "callbacks": {
            "requested": 123,
            "requested_by_disposition": {
                "key": 123
            },
            "distinct_leads": 123
        },
        "redirects": {
            "requested": 123,
            "requested_by_disposition": {
                "key": 123
            },
            "distinct_leads": 123
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path(
            "/v1/convai/outbound-campaigns/campaigns/value/follow-up-requests",
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
        .outbound_campaigns
        .get_follow_up_requests("value", None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path()
                    == "/v1/convai/outbound-campaigns/campaigns/value/follow-up-requests"
        })
        .collect();
    assert_eq!(matched.len(), 1, "expected exactly one GET /v1/convai/outbound-campaigns/campaigns/value/follow-up-requests request");
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
async fn get_follow_up_requests_surfaces_the_declared_422() {
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
            "/v1/convai/outbound-campaigns/campaigns/value/follow-up-requests",
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
        .outbound_campaigns
        .get_follow_up_requests("value", None)
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
async fn pause_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "campaign_id": "value",
        "call_name": "value",
        "agent_id": "value",
        "channel_config": {
            "channel": "phone",
            "agent_phone_number_id": "value",
            "mode_config": {
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
                },
                "mode": "voice"
            }
        },
        "status": "active",
        "policy": {
            "timezone": "value",
            "retry": {
                "retry_delay_seconds": 123,
                "max_short_retries": 30,
                "max_total_calls": 30,
                "max_callbacks": 30,
                "max_redirects": 30,
                "interval_placement": "start",
                "rotate_step_seconds": 123,
                "max_consecutive_failures_per_number": 30
            },
            "calling_periods": [
                {
                    "weekdays": [
                        123
                    ],
                    "intervals": []
                }
            ],
            "date_overrides": [
                {
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "intervals": [],
                    "name": "value"
                }
            ],
            "data_collection": {
                "callback_time": "value",
                "redirect_phone_number": "value",
                "stop": "value",
                "mappings": {
                    "key": "value"
                }
            },
            "followup_context": {
                "call_type": true,
                "previous_conversation_id": true,
                "previous_summary": true,
                "previous_transcript": true
            }
        },
        "variant": "simple",
        "source": "ui",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "updated_at_unix": 123,
        "branch_id": "value",
        "environment": "value",
        "target_concurrency_limit": 123,
        "channel": "phone",
        "mode": "voice",
        "counts": {
            "leads": 123,
            "terminal_leads": 123,
            "open_attention": 123,
            "delivery": {
                "sent": 123,
                "delivered": 123,
                "read": 123,
                "replied": 123,
                "failed": 123
            }
        },
        "agent_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/value/pause"))
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

    let result = client.outbound_campaigns.pause("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/value/pause"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/outbound-campaigns/campaigns/value/pause request"
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
async fn pause_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/outbound-campaigns/campaigns/value/pause"))
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

    let result = client.outbound_campaigns.pause("value", None).await;

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
async fn resume_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "campaign_id": "value",
        "call_name": "value",
        "agent_id": "value",
        "channel_config": {
            "channel": "phone",
            "agent_phone_number_id": "value",
            "mode_config": {
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
                },
                "mode": "voice"
            }
        },
        "status": "active",
        "policy": {
            "timezone": "value",
            "retry": {
                "retry_delay_seconds": 123,
                "max_short_retries": 30,
                "max_total_calls": 30,
                "max_callbacks": 30,
                "max_redirects": 30,
                "interval_placement": "start",
                "rotate_step_seconds": 123,
                "max_consecutive_failures_per_number": 30
            },
            "calling_periods": [
                {
                    "weekdays": [
                        123
                    ],
                    "intervals": []
                }
            ],
            "date_overrides": [
                {
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "intervals": [],
                    "name": "value"
                }
            ],
            "data_collection": {
                "callback_time": "value",
                "redirect_phone_number": "value",
                "stop": "value",
                "mappings": {
                    "key": "value"
                }
            },
            "followup_context": {
                "call_type": true,
                "previous_conversation_id": true,
                "previous_summary": true,
                "previous_transcript": true
            }
        },
        "variant": "simple",
        "source": "ui",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "updated_at_unix": 123,
        "branch_id": "value",
        "environment": "value",
        "target_concurrency_limit": 123,
        "channel": "phone",
        "mode": "voice",
        "counts": {
            "leads": 123,
            "terminal_leads": 123,
            "open_attention": 123,
            "delivery": {
                "sent": 123,
                "delivered": 123,
                "read": 123,
                "replied": 123,
                "failed": 123
            }
        },
        "agent_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/value/resume"))
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

    let result = client.outbound_campaigns.resume("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/value/resume"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/outbound-campaigns/campaigns/value/resume request"
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
async fn resume_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/outbound-campaigns/campaigns/value/resume"))
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

    let result = client.outbound_campaigns.resume("value", None).await;

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
async fn stop_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "campaign_id": "value",
        "call_name": "value",
        "agent_id": "value",
        "channel_config": {
            "channel": "phone",
            "agent_phone_number_id": "value",
            "mode_config": {
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
                },
                "mode": "voice"
            }
        },
        "status": "active",
        "policy": {
            "timezone": "value",
            "retry": {
                "retry_delay_seconds": 123,
                "max_short_retries": 30,
                "max_total_calls": 30,
                "max_callbacks": 30,
                "max_redirects": 30,
                "interval_placement": "start",
                "rotate_step_seconds": 123,
                "max_consecutive_failures_per_number": 30
            },
            "calling_periods": [
                {
                    "weekdays": [
                        123
                    ],
                    "intervals": []
                }
            ],
            "date_overrides": [
                {
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-01",
                    "intervals": [],
                    "name": "value"
                }
            ],
            "data_collection": {
                "callback_time": "value",
                "redirect_phone_number": "value",
                "stop": "value",
                "mappings": {
                    "key": "value"
                }
            },
            "followup_context": {
                "call_type": true,
                "previous_conversation_id": true,
                "previous_summary": true,
                "previous_transcript": true
            }
        },
        "variant": "simple",
        "source": "ui",
        "created_at_unix": 123,
        "scheduled_time_unix": 123,
        "updated_at_unix": 123,
        "branch_id": "value",
        "environment": "value",
        "target_concurrency_limit": 123,
        "channel": "phone",
        "mode": "voice",
        "counts": {
            "leads": 123,
            "terminal_leads": 123,
            "open_attention": 123,
            "delivery": {
                "sent": 123,
                "delivered": 123,
                "read": 123,
                "replied": 123,
                "failed": 123
            }
        },
        "agent_name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/outbound-campaigns/campaigns/value/stop"))
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

    let result = client.outbound_campaigns.stop("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/outbound-campaigns/campaigns/value/stop"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/outbound-campaigns/campaigns/value/stop request"
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
async fn stop_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/outbound-campaigns/campaigns/value/stop"))
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

    let result = client.outbound_campaigns.stop("value", None).await;

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
