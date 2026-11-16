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
        "id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "from_conversation_metadata": {
            "conversation_id": "value",
            "agent_id": "value",
            "branch_id": "value",
            "workflow_node_id": "value",
            "original_agent_reply": [
                {
                    "role": "user",
                    "agent_metadata": {
                        "agent_id": "value",
                        "branch_id": "value",
                        "workflow_node_id": "value",
                        "version_id": "value"
                    },
                    "message": "value",
                    "multivoice_message": {
                        "parts": [
                            {
                                "text": "value",
                                "voice_label": "value",
                                "time_in_call_secs": 123
                            }
                        ]
                    },
                    "tool_calls": [
                        {
                            "type": "system",
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "params_as_json": "value",
                            "tool_has_been_called": true,
                            "tool_details": {
                                "type": "webhook",
                                "method": "value",
                                "url": "value",
                                "headers": {
                                    "key": "value"
                                },
                                "path_params": {
                                    "key": "value"
                                },
                                "query_params": {
                                    "key": "value"
                                },
                                "body": "value"
                            }
                        }
                    ],
                    "tool_results": [
                        {
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "result_value": "value",
                            "is_error": true,
                            "is_blocked": true,
                            "tool_has_been_called": true,
                            "tool_latency_secs": 9.99,
                            "error_type": "value",
                            "raw_error_message": "value",
                            "dynamic_variable_updates": [
                                {
                                    "variable_name": "value",
                                    "old_value": "value",
                                    "new_value": "value",
                                    "updated_at": 9.99,
                                    "tool_name": "value",
                                    "tool_request_id": "value"
                                }
                            ],
                            "type": "client"
                        }
                    ],
                    "feedback": {
                        "score": "like",
                        "time_in_call_secs": 123
                    },
                    "llm_override": "value",
                    "producing_llm": "value",
                    "time_in_call_secs": 123,
                    "conversation_turn_metrics": {
                        "metrics": {
                            "key": {
                                "elapsed_time": 9.99
                            }
                        },
                        "convai_asr_provider": "value",
                        "convai_tts_model": "value",
                        "convai_tts_cascade": "value",
                        "voice_id": "value"
                    },
                    "rag_retrieval_info": {
                        "chunks": [
                            {
                                "document_id": "value",
                                "chunk_id": "value",
                                "vector_distance": 9.99
                            }
                        ],
                        "embedding_model": "e5_mistral_7b_instruct",
                        "retrieval_query": "value",
                        "rag_latency_secs": 9.99,
                        "used_chunk_ids": [
                            "value"
                        ]
                    },
                    "llm_usage": {
                        "model_usage": {
                            "key": {
                                "input": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_read": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_write": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "output_total": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "interrupted": true,
                    "ignored_as_backchannel": true,
                    "platform_event": "user_turn_timeout",
                    "original_message": "value",
                    "reasoning": [
                        {
                            "summary": "value",
                            "provider_redact": true
                        }
                    ],
                    "source_medium": "audio",
                    "source_event_id": 123,
                    "used_static_kb_document_ids": [
                        "value"
                    ],
                    "user_identifier": "value",
                    "id": "value",
                    "created_at_unix_ms": 123,
                    "triggered_guardrails": [
                        {
                            "guardrail_type": "custom",
                            "guardrail_name": "value",
                            "execution_mode": "streaming",
                            "guardrail_details": {
                                "type": "llm",
                                "rationale": "value"
                            }
                        }
                    ]
                }
            ]
        },
        "dynamic_variables": {},
        "chat_history": [
            {
                "role": "user",
                "agent_metadata": {
                    "agent_id": "value",
                    "branch_id": "value",
                    "workflow_node_id": "value",
                    "version_id": "value"
                },
                "message": "value",
                "multivoice_message": {
                    "parts": [
                        {
                            "text": "value",
                            "voice_label": "value",
                            "time_in_call_secs": 123
                        }
                    ]
                },
                "tool_calls": [
                    {
                        "type": "system",
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "params_as_json": "value",
                        "tool_has_been_called": true,
                        "tool_details": {
                            "type": "webhook",
                            "method": "value",
                            "url": "value",
                            "headers": {
                                "key": "value"
                            },
                            "path_params": {
                                "key": "value"
                            },
                            "query_params": {
                                "key": "value"
                            },
                            "body": "value"
                        }
                    }
                ],
                "tool_results": [
                    {
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "result_value": "value",
                        "is_error": true,
                        "is_blocked": true,
                        "tool_has_been_called": true,
                        "tool_latency_secs": 9.99,
                        "error_type": "value",
                        "raw_error_message": "value",
                        "dynamic_variable_updates": [
                            {
                                "variable_name": "value",
                                "old_value": "value",
                                "new_value": "value",
                                "updated_at": 9.99,
                                "tool_name": "value",
                                "tool_request_id": "value"
                            }
                        ],
                        "type": "client"
                    }
                ],
                "feedback": {
                    "score": "like",
                    "time_in_call_secs": 123
                },
                "llm_override": "value",
                "producing_llm": "value",
                "time_in_call_secs": 123,
                "conversation_turn_metrics": {
                    "metrics": {
                        "key": {
                            "elapsed_time": 9.99
                        }
                    },
                    "convai_asr_provider": "value",
                    "convai_tts_model": "value",
                    "convai_tts_cascade": "value",
                    "voice_id": "value"
                },
                "rag_retrieval_info": {
                    "chunks": [
                        {
                            "document_id": "value",
                            "chunk_id": "value",
                            "vector_distance": 9.99
                        }
                    ],
                    "embedding_model": "e5_mistral_7b_instruct",
                    "retrieval_query": "value",
                    "rag_latency_secs": 9.99,
                    "used_chunk_ids": [
                        "value"
                    ]
                },
                "llm_usage": {
                    "model_usage": {
                        "key": {
                            "input": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_read": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_write": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "output_total": {
                                "tokens": 123,
                                "price": 9.99
                            }
                        }
                    }
                },
                "interrupted": true,
                "ignored_as_backchannel": true,
                "platform_event": "user_turn_timeout",
                "original_message": "value",
                "reasoning": [
                    {
                        "summary": "value",
                        "provider_redact": true
                    }
                ],
                "source_medium": "audio",
                "source_event_id": 123,
                "used_static_kb_document_ids": [
                    "value"
                ],
                "user_identifier": "value",
                "id": "value",
                "created_at_unix_ms": 123,
                "triggered_guardrails": [
                    {
                        "guardrail_type": "custom",
                        "guardrail_name": "value",
                        "execution_mode": "streaming",
                        "guardrail_details": {
                            "type": "llm",
                            "rationale": "value"
                        }
                    }
                ]
            }
        ],
        "conversation_initiation_source": "unknown",
        "environment": "value",
        "type": "llm",
        "success_condition": "value",
        "success_examples": [
            {
                "response": "value",
                "type": "success"
            }
        ],
        "failure_examples": [
            {
                "response": "value",
                "type": "failure"
            }
        ],
        "name": "value",
        "parent_folder_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/create"))
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

    let result = client.tests.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST" && entry.url.path() == "/v1/convai/agent-testing/create"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/agent-testing/create request"
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
        "from_conversation_metadata": {
            "conversation_id": "value",
            "agent_id": "value",
            "branch_id": "value",
            "workflow_node_id": "value",
            "original_agent_reply": [
                {
                    "role": "user",
                    "agent_metadata": {
                        "agent_id": "value",
                        "branch_id": "value",
                        "workflow_node_id": "value",
                        "version_id": "value"
                    },
                    "message": "value",
                    "multivoice_message": {
                        "parts": [
                            {
                                "text": "value",
                                "voice_label": "value",
                                "time_in_call_secs": 123
                            }
                        ]
                    },
                    "tool_calls": [
                        {
                            "type": "system",
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "params_as_json": "value",
                            "tool_has_been_called": true,
                            "tool_details": {
                                "type": "webhook",
                                "method": "value",
                                "url": "value",
                                "headers": {
                                    "key": "value"
                                },
                                "path_params": {
                                    "key": "value"
                                },
                                "query_params": {
                                    "key": "value"
                                },
                                "body": "value"
                            }
                        }
                    ],
                    "tool_results": [
                        {
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "result_value": "value",
                            "is_error": true,
                            "is_blocked": true,
                            "tool_has_been_called": true,
                            "tool_latency_secs": 9.99,
                            "error_type": "value",
                            "raw_error_message": "value",
                            "dynamic_variable_updates": [
                                {
                                    "variable_name": "value",
                                    "old_value": "value",
                                    "new_value": "value",
                                    "updated_at": 9.99,
                                    "tool_name": "value",
                                    "tool_request_id": "value"
                                }
                            ],
                            "type": "client"
                        }
                    ],
                    "feedback": {
                        "score": "like",
                        "time_in_call_secs": 123
                    },
                    "llm_override": "value",
                    "producing_llm": "value",
                    "time_in_call_secs": 123,
                    "conversation_turn_metrics": {
                        "metrics": {
                            "key": {
                                "elapsed_time": 9.99
                            }
                        },
                        "convai_asr_provider": "value",
                        "convai_tts_model": "value",
                        "convai_tts_cascade": "value",
                        "voice_id": "value"
                    },
                    "rag_retrieval_info": {
                        "chunks": [
                            {
                                "document_id": "value",
                                "chunk_id": "value",
                                "vector_distance": 9.99
                            }
                        ],
                        "embedding_model": "e5_mistral_7b_instruct",
                        "retrieval_query": "value",
                        "rag_latency_secs": 9.99,
                        "used_chunk_ids": [
                            "value"
                        ]
                    },
                    "llm_usage": {
                        "model_usage": {
                            "key": {
                                "input": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_read": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_write": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "output_total": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "interrupted": true,
                    "ignored_as_backchannel": true,
                    "platform_event": "user_turn_timeout",
                    "original_message": "value",
                    "reasoning": [
                        {
                            "summary": "value",
                            "provider_redact": true
                        }
                    ],
                    "source_medium": "audio",
                    "source_event_id": 123,
                    "used_static_kb_document_ids": [
                        "value"
                    ],
                    "user_identifier": "value",
                    "id": "value",
                    "created_at_unix_ms": 123,
                    "triggered_guardrails": [
                        {
                            "guardrail_type": "custom",
                            "guardrail_name": "value",
                            "execution_mode": "streaming",
                            "guardrail_details": {
                                "type": "llm",
                                "rationale": "value"
                            }
                        }
                    ]
                }
            ]
        },
        "dynamic_variables": {},
        "chat_history": [
            {
                "role": "user",
                "agent_metadata": {
                    "agent_id": "value",
                    "branch_id": "value",
                    "workflow_node_id": "value",
                    "version_id": "value"
                },
                "message": "value",
                "multivoice_message": {
                    "parts": [
                        {
                            "text": "value",
                            "voice_label": "value",
                            "time_in_call_secs": 123
                        }
                    ]
                },
                "tool_calls": [
                    {
                        "type": "system",
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "params_as_json": "value",
                        "tool_has_been_called": true,
                        "tool_details": {
                            "type": "webhook",
                            "method": "value",
                            "url": "value",
                            "headers": {
                                "key": "value"
                            },
                            "path_params": {
                                "key": "value"
                            },
                            "query_params": {
                                "key": "value"
                            },
                            "body": "value"
                        }
                    }
                ],
                "tool_results": [
                    {
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "result_value": "value",
                        "is_error": true,
                        "is_blocked": true,
                        "tool_has_been_called": true,
                        "tool_latency_secs": 9.99,
                        "error_type": "value",
                        "raw_error_message": "value",
                        "dynamic_variable_updates": [
                            {
                                "variable_name": "value",
                                "old_value": "value",
                                "new_value": "value",
                                "updated_at": 9.99,
                                "tool_name": "value",
                                "tool_request_id": "value"
                            }
                        ],
                        "type": "client"
                    }
                ],
                "feedback": {
                    "score": "like",
                    "time_in_call_secs": 123
                },
                "llm_override": "value",
                "producing_llm": "value",
                "time_in_call_secs": 123,
                "conversation_turn_metrics": {
                    "metrics": {
                        "key": {
                            "elapsed_time": 9.99
                        }
                    },
                    "convai_asr_provider": "value",
                    "convai_tts_model": "value",
                    "convai_tts_cascade": "value",
                    "voice_id": "value"
                },
                "rag_retrieval_info": {
                    "chunks": [
                        {
                            "document_id": "value",
                            "chunk_id": "value",
                            "vector_distance": 9.99
                        }
                    ],
                    "embedding_model": "e5_mistral_7b_instruct",
                    "retrieval_query": "value",
                    "rag_latency_secs": 9.99,
                    "used_chunk_ids": [
                        "value"
                    ]
                },
                "llm_usage": {
                    "model_usage": {
                        "key": {
                            "input": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_read": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_write": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "output_total": {
                                "tokens": 123,
                                "price": 9.99
                            }
                        }
                    }
                },
                "interrupted": true,
                "ignored_as_backchannel": true,
                "platform_event": "user_turn_timeout",
                "original_message": "value",
                "reasoning": [
                    {
                        "summary": "value",
                        "provider_redact": true
                    }
                ],
                "source_medium": "audio",
                "source_event_id": 123,
                "used_static_kb_document_ids": [
                    "value"
                ],
                "user_identifier": "value",
                "id": "value",
                "created_at_unix_ms": 123,
                "triggered_guardrails": [
                    {
                        "guardrail_type": "custom",
                        "guardrail_name": "value",
                        "execution_mode": "streaming",
                        "guardrail_details": {
                            "type": "llm",
                            "rationale": "value"
                        }
                    }
                ]
            }
        ],
        "conversation_initiation_source": "unknown",
        "environment": "value",
        "type": "llm",
        "success_condition": "value",
        "success_examples": [
            {
                "response": "value",
                "type": "success"
            }
        ],
        "failure_examples": [
            {
                "response": "value",
                "type": "failure"
            }
        ],
        "name": "value",
        "parent_folder_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/create"))
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

    let result = client.tests.create(&body, None).await;

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
async fn move__sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response =
        serde_json::from_str::<serde_json::Value>(r#"{}"#).expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "entity_ids": [
            "value"
        ],
        "move_to": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/bulk-move"))
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

    let result = client.tests.move_(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/agent-testing/bulk-move"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/agent-testing/bulk-move request"
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
async fn move__surfaces_the_declared_422() {
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
        "entity_ids": [
            "value"
        ],
        "move_to": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/bulk-move"))
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

    let result = client.tests.move_(&body, None).await;

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
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "from_conversation_metadata": {
            "conversation_id": "value",
            "agent_id": "value",
            "branch_id": "value",
            "workflow_node_id": "value",
            "original_agent_reply": [
                {
                    "role": "user",
                    "agent_metadata": {
                        "agent_id": "value",
                        "branch_id": "value",
                        "workflow_node_id": "value",
                        "version_id": "value"
                    },
                    "message": "value",
                    "multivoice_message": {
                        "parts": [
                            {
                                "text": "value",
                                "voice_label": "value",
                                "time_in_call_secs": 123
                            }
                        ]
                    },
                    "tool_calls": [
                        {
                            "type": "system",
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "params_as_json": "value",
                            "tool_has_been_called": true,
                            "tool_details": {
                                "type": "webhook",
                                "method": "value",
                                "url": "value",
                                "headers": {
                                    "key": "value"
                                },
                                "path_params": {
                                    "key": "value"
                                },
                                "query_params": {
                                    "key": "value"
                                },
                                "body": "value"
                            }
                        }
                    ],
                    "tool_results": [
                        {
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "result_value": "value",
                            "is_error": true,
                            "is_blocked": true,
                            "tool_has_been_called": true,
                            "tool_latency_secs": 9.99,
                            "error_type": "value",
                            "raw_error_message": "value",
                            "dynamic_variable_updates": [
                                {
                                    "variable_name": "value",
                                    "old_value": "value",
                                    "new_value": "value",
                                    "updated_at": 9.99,
                                    "tool_name": "value",
                                    "tool_request_id": "value"
                                }
                            ],
                            "type": "client"
                        }
                    ],
                    "feedback": {
                        "score": "like",
                        "time_in_call_secs": 123
                    },
                    "llm_override": "value",
                    "producing_llm": "value",
                    "time_in_call_secs": 123,
                    "conversation_turn_metrics": {
                        "metrics": {
                            "key": {
                                "elapsed_time": 9.99
                            }
                        },
                        "convai_asr_provider": "value",
                        "convai_tts_model": "value",
                        "convai_tts_cascade": "value",
                        "voice_id": "value"
                    },
                    "rag_retrieval_info": {
                        "chunks": [
                            {
                                "document_id": "value",
                                "chunk_id": "value",
                                "vector_distance": 9.99
                            }
                        ],
                        "embedding_model": "e5_mistral_7b_instruct",
                        "retrieval_query": "value",
                        "rag_latency_secs": 9.99,
                        "used_chunk_ids": [
                            "value"
                        ]
                    },
                    "llm_usage": {
                        "model_usage": {
                            "key": {
                                "input": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_read": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_write": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "output_total": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "interrupted": true,
                    "ignored_as_backchannel": true,
                    "platform_event": "user_turn_timeout",
                    "original_message": "value",
                    "reasoning": [
                        {
                            "summary": "value",
                            "provider_redact": true
                        }
                    ],
                    "source_medium": "audio",
                    "source_event_id": 123,
                    "used_static_kb_document_ids": [
                        "value"
                    ],
                    "user_identifier": "value",
                    "id": "value",
                    "created_at_unix_ms": 123,
                    "triggered_guardrails": [
                        {
                            "guardrail_type": "custom",
                            "guardrail_name": "value",
                            "execution_mode": "streaming",
                            "guardrail_details": {
                                "type": "llm",
                                "rationale": "value"
                            }
                        }
                    ]
                }
            ]
        },
        "dynamic_variables": {},
        "chat_history": [
            {
                "role": "user",
                "agent_metadata": {
                    "agent_id": "value",
                    "branch_id": "value",
                    "workflow_node_id": "value",
                    "version_id": "value"
                },
                "message": "value",
                "multivoice_message": {
                    "parts": [
                        {
                            "text": "value",
                            "voice_label": "value",
                            "time_in_call_secs": 123
                        }
                    ]
                },
                "tool_calls": [
                    {
                        "type": "system",
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "params_as_json": "value",
                        "tool_has_been_called": true,
                        "tool_details": {
                            "type": "webhook",
                            "method": "value",
                            "url": "value",
                            "headers": {
                                "key": "value"
                            },
                            "path_params": {
                                "key": "value"
                            },
                            "query_params": {
                                "key": "value"
                            },
                            "body": "value"
                        }
                    }
                ],
                "tool_results": [
                    {
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "result_value": "value",
                        "is_error": true,
                        "is_blocked": true,
                        "tool_has_been_called": true,
                        "tool_latency_secs": 9.99,
                        "error_type": "value",
                        "raw_error_message": "value",
                        "dynamic_variable_updates": [
                            {
                                "variable_name": "value",
                                "old_value": "value",
                                "new_value": "value",
                                "updated_at": 9.99,
                                "tool_name": "value",
                                "tool_request_id": "value"
                            }
                        ],
                        "type": "client"
                    }
                ],
                "feedback": {
                    "score": "like",
                    "time_in_call_secs": 123
                },
                "llm_override": "value",
                "producing_llm": "value",
                "time_in_call_secs": 123,
                "conversation_turn_metrics": {
                    "metrics": {
                        "key": {
                            "elapsed_time": 9.99
                        }
                    },
                    "convai_asr_provider": "value",
                    "convai_tts_model": "value",
                    "convai_tts_cascade": "value",
                    "voice_id": "value"
                },
                "rag_retrieval_info": {
                    "chunks": [
                        {
                            "document_id": "value",
                            "chunk_id": "value",
                            "vector_distance": 9.99
                        }
                    ],
                    "embedding_model": "e5_mistral_7b_instruct",
                    "retrieval_query": "value",
                    "rag_latency_secs": 9.99,
                    "used_chunk_ids": [
                        "value"
                    ]
                },
                "llm_usage": {
                    "model_usage": {
                        "key": {
                            "input": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_read": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_write": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "output_total": {
                                "tokens": 123,
                                "price": 9.99
                            }
                        }
                    }
                },
                "interrupted": true,
                "ignored_as_backchannel": true,
                "platform_event": "user_turn_timeout",
                "original_message": "value",
                "reasoning": [
                    {
                        "summary": "value",
                        "provider_redact": true
                    }
                ],
                "source_medium": "audio",
                "source_event_id": 123,
                "used_static_kb_document_ids": [
                    "value"
                ],
                "user_identifier": "value",
                "id": "value",
                "created_at_unix_ms": 123,
                "triggered_guardrails": [
                    {
                        "guardrail_type": "custom",
                        "guardrail_name": "value",
                        "execution_mode": "streaming",
                        "guardrail_details": {
                            "type": "llm",
                            "rationale": "value"
                        }
                    }
                ]
            }
        ],
        "conversation_initiation_source": "unknown",
        "environment": "value",
        "type": "llm",
        "success_condition": "value",
        "success_examples": [
            {
                "response": "value",
                "type": "success"
            }
        ],
        "failure_examples": [
            {
                "response": "value",
                "type": "failure"
            }
        ],
        "id": "value",
        "name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/agent-testing/value"))
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

    let result = client.tests.get("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET" && entry.url.path() == "/v1/convai/agent-testing/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/agent-testing/value request"
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
        .and(path("/v1/convai/agent-testing/value"))
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

    let result = client.tests.get("value", None).await;

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
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "from_conversation_metadata": {
            "conversation_id": "value",
            "agent_id": "value",
            "branch_id": "value",
            "workflow_node_id": "value",
            "original_agent_reply": [
                {
                    "role": "user",
                    "agent_metadata": {
                        "agent_id": "value",
                        "branch_id": "value",
                        "workflow_node_id": "value",
                        "version_id": "value"
                    },
                    "message": "value",
                    "multivoice_message": {
                        "parts": [
                            {
                                "text": "value",
                                "voice_label": "value",
                                "time_in_call_secs": 123
                            }
                        ]
                    },
                    "tool_calls": [
                        {
                            "type": "system",
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "params_as_json": "value",
                            "tool_has_been_called": true,
                            "tool_details": {
                                "type": "webhook",
                                "method": "value",
                                "url": "value",
                                "headers": {
                                    "key": "value"
                                },
                                "path_params": {
                                    "key": "value"
                                },
                                "query_params": {
                                    "key": "value"
                                },
                                "body": "value"
                            }
                        }
                    ],
                    "tool_results": [
                        {
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "result_value": "value",
                            "is_error": true,
                            "is_blocked": true,
                            "tool_has_been_called": true,
                            "tool_latency_secs": 9.99,
                            "error_type": "value",
                            "raw_error_message": "value",
                            "dynamic_variable_updates": [
                                {
                                    "variable_name": "value",
                                    "old_value": "value",
                                    "new_value": "value",
                                    "updated_at": 9.99,
                                    "tool_name": "value",
                                    "tool_request_id": "value"
                                }
                            ],
                            "type": "client"
                        }
                    ],
                    "feedback": {
                        "score": "like",
                        "time_in_call_secs": 123
                    },
                    "llm_override": "value",
                    "producing_llm": "value",
                    "time_in_call_secs": 123,
                    "conversation_turn_metrics": {
                        "metrics": {
                            "key": {
                                "elapsed_time": 9.99
                            }
                        },
                        "convai_asr_provider": "value",
                        "convai_tts_model": "value",
                        "convai_tts_cascade": "value",
                        "voice_id": "value"
                    },
                    "rag_retrieval_info": {
                        "chunks": [
                            {
                                "document_id": "value",
                                "chunk_id": "value",
                                "vector_distance": 9.99
                            }
                        ],
                        "embedding_model": "e5_mistral_7b_instruct",
                        "retrieval_query": "value",
                        "rag_latency_secs": 9.99,
                        "used_chunk_ids": [
                            "value"
                        ]
                    },
                    "llm_usage": {
                        "model_usage": {
                            "key": {
                                "input": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_read": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_write": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "output_total": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "interrupted": true,
                    "ignored_as_backchannel": true,
                    "platform_event": "user_turn_timeout",
                    "original_message": "value",
                    "reasoning": [
                        {
                            "summary": "value",
                            "provider_redact": true
                        }
                    ],
                    "source_medium": "audio",
                    "source_event_id": 123,
                    "used_static_kb_document_ids": [
                        "value"
                    ],
                    "user_identifier": "value",
                    "id": "value",
                    "created_at_unix_ms": 123,
                    "triggered_guardrails": [
                        {
                            "guardrail_type": "custom",
                            "guardrail_name": "value",
                            "execution_mode": "streaming",
                            "guardrail_details": {
                                "type": "llm",
                                "rationale": "value"
                            }
                        }
                    ]
                }
            ]
        },
        "dynamic_variables": {},
        "chat_history": [
            {
                "role": "user",
                "agent_metadata": {
                    "agent_id": "value",
                    "branch_id": "value",
                    "workflow_node_id": "value",
                    "version_id": "value"
                },
                "message": "value",
                "multivoice_message": {
                    "parts": [
                        {
                            "text": "value",
                            "voice_label": "value",
                            "time_in_call_secs": 123
                        }
                    ]
                },
                "tool_calls": [
                    {
                        "type": "system",
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "params_as_json": "value",
                        "tool_has_been_called": true,
                        "tool_details": {
                            "type": "webhook",
                            "method": "value",
                            "url": "value",
                            "headers": {
                                "key": "value"
                            },
                            "path_params": {
                                "key": "value"
                            },
                            "query_params": {
                                "key": "value"
                            },
                            "body": "value"
                        }
                    }
                ],
                "tool_results": [
                    {
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "result_value": "value",
                        "is_error": true,
                        "is_blocked": true,
                        "tool_has_been_called": true,
                        "tool_latency_secs": 9.99,
                        "error_type": "value",
                        "raw_error_message": "value",
                        "dynamic_variable_updates": [
                            {
                                "variable_name": "value",
                                "old_value": "value",
                                "new_value": "value",
                                "updated_at": 9.99,
                                "tool_name": "value",
                                "tool_request_id": "value"
                            }
                        ],
                        "type": "client"
                    }
                ],
                "feedback": {
                    "score": "like",
                    "time_in_call_secs": 123
                },
                "llm_override": "value",
                "producing_llm": "value",
                "time_in_call_secs": 123,
                "conversation_turn_metrics": {
                    "metrics": {
                        "key": {
                            "elapsed_time": 9.99
                        }
                    },
                    "convai_asr_provider": "value",
                    "convai_tts_model": "value",
                    "convai_tts_cascade": "value",
                    "voice_id": "value"
                },
                "rag_retrieval_info": {
                    "chunks": [
                        {
                            "document_id": "value",
                            "chunk_id": "value",
                            "vector_distance": 9.99
                        }
                    ],
                    "embedding_model": "e5_mistral_7b_instruct",
                    "retrieval_query": "value",
                    "rag_latency_secs": 9.99,
                    "used_chunk_ids": [
                        "value"
                    ]
                },
                "llm_usage": {
                    "model_usage": {
                        "key": {
                            "input": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_read": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_write": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "output_total": {
                                "tokens": 123,
                                "price": 9.99
                            }
                        }
                    }
                },
                "interrupted": true,
                "ignored_as_backchannel": true,
                "platform_event": "user_turn_timeout",
                "original_message": "value",
                "reasoning": [
                    {
                        "summary": "value",
                        "provider_redact": true
                    }
                ],
                "source_medium": "audio",
                "source_event_id": 123,
                "used_static_kb_document_ids": [
                    "value"
                ],
                "user_identifier": "value",
                "id": "value",
                "created_at_unix_ms": 123,
                "triggered_guardrails": [
                    {
                        "guardrail_type": "custom",
                        "guardrail_name": "value",
                        "execution_mode": "streaming",
                        "guardrail_details": {
                            "type": "llm",
                            "rationale": "value"
                        }
                    }
                ]
            }
        ],
        "conversation_initiation_source": "unknown",
        "environment": "value",
        "type": "llm",
        "success_condition": "value",
        "success_examples": [
            {
                "response": "value",
                "type": "success"
            }
        ],
        "failure_examples": [
            {
                "response": "value",
                "type": "failure"
            }
        ],
        "id": "value",
        "name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "from_conversation_metadata": {
            "conversation_id": "value",
            "agent_id": "value",
            "branch_id": "value",
            "workflow_node_id": "value",
            "original_agent_reply": [
                {
                    "role": "user",
                    "agent_metadata": {
                        "agent_id": "value",
                        "branch_id": "value",
                        "workflow_node_id": "value",
                        "version_id": "value"
                    },
                    "message": "value",
                    "multivoice_message": {
                        "parts": [
                            {
                                "text": "value",
                                "voice_label": "value",
                                "time_in_call_secs": 123
                            }
                        ]
                    },
                    "tool_calls": [
                        {
                            "type": "system",
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "params_as_json": "value",
                            "tool_has_been_called": true,
                            "tool_details": {
                                "type": "webhook",
                                "method": "value",
                                "url": "value",
                                "headers": {
                                    "key": "value"
                                },
                                "path_params": {
                                    "key": "value"
                                },
                                "query_params": {
                                    "key": "value"
                                },
                                "body": "value"
                            }
                        }
                    ],
                    "tool_results": [
                        {
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "result_value": "value",
                            "is_error": true,
                            "is_blocked": true,
                            "tool_has_been_called": true,
                            "tool_latency_secs": 9.99,
                            "error_type": "value",
                            "raw_error_message": "value",
                            "dynamic_variable_updates": [
                                {
                                    "variable_name": "value",
                                    "old_value": "value",
                                    "new_value": "value",
                                    "updated_at": 9.99,
                                    "tool_name": "value",
                                    "tool_request_id": "value"
                                }
                            ],
                            "type": "client"
                        }
                    ],
                    "feedback": {
                        "score": "like",
                        "time_in_call_secs": 123
                    },
                    "llm_override": "value",
                    "producing_llm": "value",
                    "time_in_call_secs": 123,
                    "conversation_turn_metrics": {
                        "metrics": {
                            "key": {
                                "elapsed_time": 9.99
                            }
                        },
                        "convai_asr_provider": "value",
                        "convai_tts_model": "value",
                        "convai_tts_cascade": "value",
                        "voice_id": "value"
                    },
                    "rag_retrieval_info": {
                        "chunks": [
                            {
                                "document_id": "value",
                                "chunk_id": "value",
                                "vector_distance": 9.99
                            }
                        ],
                        "embedding_model": "e5_mistral_7b_instruct",
                        "retrieval_query": "value",
                        "rag_latency_secs": 9.99,
                        "used_chunk_ids": [
                            "value"
                        ]
                    },
                    "llm_usage": {
                        "model_usage": {
                            "key": {
                                "input": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_read": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_write": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "output_total": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "interrupted": true,
                    "ignored_as_backchannel": true,
                    "platform_event": "user_turn_timeout",
                    "original_message": "value",
                    "reasoning": [
                        {
                            "summary": "value",
                            "provider_redact": true
                        }
                    ],
                    "source_medium": "audio",
                    "source_event_id": 123,
                    "used_static_kb_document_ids": [
                        "value"
                    ],
                    "user_identifier": "value",
                    "id": "value",
                    "created_at_unix_ms": 123,
                    "triggered_guardrails": [
                        {
                            "guardrail_type": "custom",
                            "guardrail_name": "value",
                            "execution_mode": "streaming",
                            "guardrail_details": {
                                "type": "llm",
                                "rationale": "value"
                            }
                        }
                    ]
                }
            ]
        },
        "dynamic_variables": {},
        "chat_history": [
            {
                "role": "user",
                "agent_metadata": {
                    "agent_id": "value",
                    "branch_id": "value",
                    "workflow_node_id": "value",
                    "version_id": "value"
                },
                "message": "value",
                "multivoice_message": {
                    "parts": [
                        {
                            "text": "value",
                            "voice_label": "value",
                            "time_in_call_secs": 123
                        }
                    ]
                },
                "tool_calls": [
                    {
                        "type": "system",
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "params_as_json": "value",
                        "tool_has_been_called": true,
                        "tool_details": {
                            "type": "webhook",
                            "method": "value",
                            "url": "value",
                            "headers": {
                                "key": "value"
                            },
                            "path_params": {
                                "key": "value"
                            },
                            "query_params": {
                                "key": "value"
                            },
                            "body": "value"
                        }
                    }
                ],
                "tool_results": [
                    {
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "result_value": "value",
                        "is_error": true,
                        "is_blocked": true,
                        "tool_has_been_called": true,
                        "tool_latency_secs": 9.99,
                        "error_type": "value",
                        "raw_error_message": "value",
                        "dynamic_variable_updates": [
                            {
                                "variable_name": "value",
                                "old_value": "value",
                                "new_value": "value",
                                "updated_at": 9.99,
                                "tool_name": "value",
                                "tool_request_id": "value"
                            }
                        ],
                        "type": "client"
                    }
                ],
                "feedback": {
                    "score": "like",
                    "time_in_call_secs": 123
                },
                "llm_override": "value",
                "producing_llm": "value",
                "time_in_call_secs": 123,
                "conversation_turn_metrics": {
                    "metrics": {
                        "key": {
                            "elapsed_time": 9.99
                        }
                    },
                    "convai_asr_provider": "value",
                    "convai_tts_model": "value",
                    "convai_tts_cascade": "value",
                    "voice_id": "value"
                },
                "rag_retrieval_info": {
                    "chunks": [
                        {
                            "document_id": "value",
                            "chunk_id": "value",
                            "vector_distance": 9.99
                        }
                    ],
                    "embedding_model": "e5_mistral_7b_instruct",
                    "retrieval_query": "value",
                    "rag_latency_secs": 9.99,
                    "used_chunk_ids": [
                        "value"
                    ]
                },
                "llm_usage": {
                    "model_usage": {
                        "key": {
                            "input": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_read": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_write": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "output_total": {
                                "tokens": 123,
                                "price": 9.99
                            }
                        }
                    }
                },
                "interrupted": true,
                "ignored_as_backchannel": true,
                "platform_event": "user_turn_timeout",
                "original_message": "value",
                "reasoning": [
                    {
                        "summary": "value",
                        "provider_redact": true
                    }
                ],
                "source_medium": "audio",
                "source_event_id": 123,
                "used_static_kb_document_ids": [
                    "value"
                ],
                "user_identifier": "value",
                "id": "value",
                "created_at_unix_ms": 123,
                "triggered_guardrails": [
                    {
                        "guardrail_type": "custom",
                        "guardrail_name": "value",
                        "execution_mode": "streaming",
                        "guardrail_details": {
                            "type": "llm",
                            "rationale": "value"
                        }
                    }
                ]
            }
        ],
        "conversation_initiation_source": "unknown",
        "environment": "value",
        "type": "llm",
        "success_condition": "value",
        "success_examples": [
            {
                "response": "value",
                "type": "success"
            }
        ],
        "failure_examples": [
            {
                "response": "value",
                "type": "failure"
            }
        ],
        "name": "value",
        "parent_folder_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PUT"))
        .and(path("/v1/convai/agent-testing/value"))
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

    let result = client.tests.update("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PUT" && entry.url.path() == "/v1/convai/agent-testing/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PUT /v1/convai/agent-testing/value request"
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
        "from_conversation_metadata": {
            "conversation_id": "value",
            "agent_id": "value",
            "branch_id": "value",
            "workflow_node_id": "value",
            "original_agent_reply": [
                {
                    "role": "user",
                    "agent_metadata": {
                        "agent_id": "value",
                        "branch_id": "value",
                        "workflow_node_id": "value",
                        "version_id": "value"
                    },
                    "message": "value",
                    "multivoice_message": {
                        "parts": [
                            {
                                "text": "value",
                                "voice_label": "value",
                                "time_in_call_secs": 123
                            }
                        ]
                    },
                    "tool_calls": [
                        {
                            "type": "system",
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "params_as_json": "value",
                            "tool_has_been_called": true,
                            "tool_details": {
                                "type": "webhook",
                                "method": "value",
                                "url": "value",
                                "headers": {
                                    "key": "value"
                                },
                                "path_params": {
                                    "key": "value"
                                },
                                "query_params": {
                                    "key": "value"
                                },
                                "body": "value"
                            }
                        }
                    ],
                    "tool_results": [
                        {
                            "request_id": "value",
                            "tool_name": "value",
                            "tool_id": "value",
                            "result_value": "value",
                            "is_error": true,
                            "is_blocked": true,
                            "tool_has_been_called": true,
                            "tool_latency_secs": 9.99,
                            "error_type": "value",
                            "raw_error_message": "value",
                            "dynamic_variable_updates": [
                                {
                                    "variable_name": "value",
                                    "old_value": "value",
                                    "new_value": "value",
                                    "updated_at": 9.99,
                                    "tool_name": "value",
                                    "tool_request_id": "value"
                                }
                            ],
                            "type": "client"
                        }
                    ],
                    "feedback": {
                        "score": "like",
                        "time_in_call_secs": 123
                    },
                    "llm_override": "value",
                    "producing_llm": "value",
                    "time_in_call_secs": 123,
                    "conversation_turn_metrics": {
                        "metrics": {
                            "key": {
                                "elapsed_time": 9.99
                            }
                        },
                        "convai_asr_provider": "value",
                        "convai_tts_model": "value",
                        "convai_tts_cascade": "value",
                        "voice_id": "value"
                    },
                    "rag_retrieval_info": {
                        "chunks": [
                            {
                                "document_id": "value",
                                "chunk_id": "value",
                                "vector_distance": 9.99
                            }
                        ],
                        "embedding_model": "e5_mistral_7b_instruct",
                        "retrieval_query": "value",
                        "rag_latency_secs": 9.99,
                        "used_chunk_ids": [
                            "value"
                        ]
                    },
                    "llm_usage": {
                        "model_usage": {
                            "key": {
                                "input": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_read": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "input_cache_write": {
                                    "tokens": 123,
                                    "price": 9.99
                                },
                                "output_total": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "interrupted": true,
                    "ignored_as_backchannel": true,
                    "platform_event": "user_turn_timeout",
                    "original_message": "value",
                    "reasoning": [
                        {
                            "summary": "value",
                            "provider_redact": true
                        }
                    ],
                    "source_medium": "audio",
                    "source_event_id": 123,
                    "used_static_kb_document_ids": [
                        "value"
                    ],
                    "user_identifier": "value",
                    "id": "value",
                    "created_at_unix_ms": 123,
                    "triggered_guardrails": [
                        {
                            "guardrail_type": "custom",
                            "guardrail_name": "value",
                            "execution_mode": "streaming",
                            "guardrail_details": {
                                "type": "llm",
                                "rationale": "value"
                            }
                        }
                    ]
                }
            ]
        },
        "dynamic_variables": {},
        "chat_history": [
            {
                "role": "user",
                "agent_metadata": {
                    "agent_id": "value",
                    "branch_id": "value",
                    "workflow_node_id": "value",
                    "version_id": "value"
                },
                "message": "value",
                "multivoice_message": {
                    "parts": [
                        {
                            "text": "value",
                            "voice_label": "value",
                            "time_in_call_secs": 123
                        }
                    ]
                },
                "tool_calls": [
                    {
                        "type": "system",
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "params_as_json": "value",
                        "tool_has_been_called": true,
                        "tool_details": {
                            "type": "webhook",
                            "method": "value",
                            "url": "value",
                            "headers": {
                                "key": "value"
                            },
                            "path_params": {
                                "key": "value"
                            },
                            "query_params": {
                                "key": "value"
                            },
                            "body": "value"
                        }
                    }
                ],
                "tool_results": [
                    {
                        "request_id": "value",
                        "tool_name": "value",
                        "tool_id": "value",
                        "result_value": "value",
                        "is_error": true,
                        "is_blocked": true,
                        "tool_has_been_called": true,
                        "tool_latency_secs": 9.99,
                        "error_type": "value",
                        "raw_error_message": "value",
                        "dynamic_variable_updates": [
                            {
                                "variable_name": "value",
                                "old_value": "value",
                                "new_value": "value",
                                "updated_at": 9.99,
                                "tool_name": "value",
                                "tool_request_id": "value"
                            }
                        ],
                        "type": "client"
                    }
                ],
                "feedback": {
                    "score": "like",
                    "time_in_call_secs": 123
                },
                "llm_override": "value",
                "producing_llm": "value",
                "time_in_call_secs": 123,
                "conversation_turn_metrics": {
                    "metrics": {
                        "key": {
                            "elapsed_time": 9.99
                        }
                    },
                    "convai_asr_provider": "value",
                    "convai_tts_model": "value",
                    "convai_tts_cascade": "value",
                    "voice_id": "value"
                },
                "rag_retrieval_info": {
                    "chunks": [
                        {
                            "document_id": "value",
                            "chunk_id": "value",
                            "vector_distance": 9.99
                        }
                    ],
                    "embedding_model": "e5_mistral_7b_instruct",
                    "retrieval_query": "value",
                    "rag_latency_secs": 9.99,
                    "used_chunk_ids": [
                        "value"
                    ]
                },
                "llm_usage": {
                    "model_usage": {
                        "key": {
                            "input": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_read": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "input_cache_write": {
                                "tokens": 123,
                                "price": 9.99
                            },
                            "output_total": {
                                "tokens": 123,
                                "price": 9.99
                            }
                        }
                    }
                },
                "interrupted": true,
                "ignored_as_backchannel": true,
                "platform_event": "user_turn_timeout",
                "original_message": "value",
                "reasoning": [
                    {
                        "summary": "value",
                        "provider_redact": true
                    }
                ],
                "source_medium": "audio",
                "source_event_id": 123,
                "used_static_kb_document_ids": [
                    "value"
                ],
                "user_identifier": "value",
                "id": "value",
                "created_at_unix_ms": 123,
                "triggered_guardrails": [
                    {
                        "guardrail_type": "custom",
                        "guardrail_name": "value",
                        "execution_mode": "streaming",
                        "guardrail_details": {
                            "type": "llm",
                            "rationale": "value"
                        }
                    }
                ]
            }
        ],
        "conversation_initiation_source": "unknown",
        "environment": "value",
        "type": "llm",
        "success_condition": "value",
        "success_examples": [
            {
                "response": "value",
                "type": "success"
            }
        ],
        "failure_examples": [
            {
                "response": "value",
                "type": "failure"
            }
        ],
        "name": "value",
        "parent_folder_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PUT"))
        .and(path("/v1/convai/agent-testing/value"))
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

    let result = client.tests.update("value", &body, None).await;

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
    let expected_response =
        serde_json::from_str::<serde_json::Value>(r#"{}"#).expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/convai/agent-testing/value"))
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

    let result = client.tests.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE"
                && entry.url.path() == "/v1/convai/agent-testing/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/convai/agent-testing/value request"
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
        .and(path("/v1/convai/agent-testing/value"))
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

    let result = client.tests.delete("value", None).await;

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
async fn summaries_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "tests": {
            "key": {
                "id": "value",
                "name": "value",
                "access_info": {
                    "is_creator": true,
                    "creator_name": "value",
                    "creator_email": "value",
                    "role": "admin",
                    "anonymous_access_level_override": "admin",
                    "access_source": "creator"
                },
                "created_at_unix_secs": 123,
                "last_updated_at_unix_secs": 123,
                "type": "llm",
                "entity_type": "test",
                "folder_parent_id": "value",
                "folder_path": [
                    {
                        "id": "value",
                        "name": "value"
                    }
                ],
                "children_count": 123,
                "conversation_initiation_source": "unknown"
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "test_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/summaries"))
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

    let result = client.tests.summaries(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/agent-testing/summaries"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/agent-testing/summaries request"
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
async fn summaries_surfaces_the_declared_422() {
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
        "test_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/summaries"))
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

    let result = client.tests.summaries(&body, None).await;

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
