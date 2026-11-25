use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn run_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agent_id": "value",
        "agent_name": "value",
        "conversation_product": "value",
        "status": "initiated",
        "user_id": "value",
        "branch_id": "value",
        "version_id": "value",
        "metadata": {
            "start_time_unix_secs": 123,
            "accepted_time_unix_secs": 123,
            "call_duration_secs": 123,
            "queue_wait_secs": 9.99,
            "cost": 123,
            "deletion_settings": {
                "deletion_time_unix_secs": 123,
                "deleted_logs_at_time_unix_secs": 123,
                "deleted_audio_at_time_unix_secs": 123,
                "deleted_transcript_at_time_unix_secs": 123,
                "delete_transcript_and_pii": true,
                "delete_audio": true
            },
            "feedback": {
                "type": "thumbs",
                "overall_score": "like",
                "likes": 123,
                "dislikes": 123,
                "rating": 123,
                "comment": "value"
            },
            "authorization_method": "invalid",
            "charging": {
                "dev_discount": true,
                "is_burst": true,
                "tier": "value",
                "llm_usage": {
                    "irreversible_generation": {
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
                                },
                                "output_total_reasoning_portion": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "initiated_generation": {
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
                                },
                                "output_total_reasoning_portion": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    }
                },
                "llm_price": 9.99,
                "llm_charge": 123,
                "call_charge": 123,
                "platform_charge": 123,
                "platform_usage": {
                    "category_usage": {
                        "key": {
                            "credits": 123,
                            "price": 9.99,
                            "quantity": 9.99
                        }
                    }
                },
                "platform_price": 9.99,
                "free_minutes_consumed": 9.99,
                "free_llm_dollars_consumed": 9.99,
                "tts_usage": {
                    "primary_tts_model": "value",
                    "total_audio_output_seconds": 9.99,
                    "total_characters": 123,
                    "per_voice_usage": [
                        {
                            "voice_id": "value",
                            "audio_output_seconds": 9.99
                        }
                    ]
                },
                "asr_usage": {
                    "asr_model": "value",
                    "total_transcription_calls": 123,
                    "total_audio_input_seconds": 9.99
                },
                "analysis": {
                    "total": {
                        "price": 9.99,
                        "charge": 123,
                        "runs": 123,
                        "price_per_feature": {
                            "key": 9.99
                        },
                        "charge_per_feature": {
                            "key": 123
                        }
                    },
                    "last_run": {
                        "price": 9.99,
                        "charge": 123,
                        "price_per_feature": {
                            "key": 9.99
                        },
                        "charge_per_feature": {
                            "key": 123
                        }
                    }
                },
                "voice_surcharge_charge": 123
            },
            "phone_call": {
                "direction": "inbound",
                "phone_number_id": "value",
                "agent_number": "value",
                "external_number": "value",
                "call_result": "answered",
                "type": "twilio",
                "stream_sid": "value",
                "call_sid": "value"
            },
            "batch_call": {
                "batch_call_id": "value",
                "batch_call_recipient_id": "value",
                "campaign": {
                    "campaign_id": "value",
                    "campaign_lead_id": "value",
                    "cold_attempt": 123,
                    "conversation_type": "initial"
                }
            },
            "termination_reason": "value",
            "error": {
                "code": 123,
                "reason": "value"
            },
            "warnings": [
                "value"
            ],
            "main_language": "value",
            "rag_usage": {
                "usage_count": 123,
                "embedding_model": "value"
            },
            "text_only": true,
            "features_usage": {
                "language_detection": {
                    "enabled": true,
                    "used": true
                },
                "transfer_to_agent": {
                    "enabled": true,
                    "used": true
                },
                "transfer_to_number": {
                    "enabled": true,
                    "used": true
                },
                "multivoice": {
                    "enabled": true,
                    "used": true
                },
                "dtmf_tones": {
                    "enabled": true,
                    "used": true
                },
                "external_mcp_servers": {
                    "enabled": true,
                    "used": true
                },
                "pii_zrm_workspace": true,
                "pii_zrm_agent": true,
                "tool_dynamic_variable_updates": {
                    "enabled": true,
                    "used": true
                },
                "is_livekit": true,
                "voicemail_detection": {
                    "enabled": true,
                    "used": true
                },
                "dtmf_input": {
                    "enabled": true,
                    "used": true
                },
                "workflow": {
                    "enabled": true,
                    "tool_node": {
                        "enabled": true,
                        "used": true
                    },
                    "standalone_agent_node": {
                        "enabled": true,
                        "used": true
                    },
                    "phone_number_node": {
                        "enabled": true,
                        "used": true
                    },
                    "end_node": {
                        "enabled": true,
                        "used": true
                    },
                    "override_agent_node": {
                        "enabled": true,
                        "used": true
                    }
                },
                "agent_testing": {
                    "enabled": true,
                    "tests_ran_after_last_modification": true,
                    "tests_ran_in_last_7_days": true
                },
                "versioning": {
                    "enabled": true,
                    "used": true
                },
                "file_input": {
                    "enabled": true,
                    "used": true
                },
                "freeform_procedure": {
                    "enabled": true,
                    "used": true
                },
                "structured_procedure": {
                    "enabled": true,
                    "used": true
                }
            },
            "eleven_assistant": {
                "is_eleven_assistant": true
            },
            "initiator_id": "value",
            "conversation_initiation_source": "unknown",
            "conversation_initiation_source_version": "value",
            "timezone": "value",
            "async_metadata": {
                "delivery_status": "pending",
                "delivery_timestamp": 123,
                "delivery_error": "value",
                "external_system": "value",
                "external_id": "value",
                "external_link": "value",
                "retry_count": 123,
                "last_retry_timestamp": 123,
                "last_processed_external_message_id": "value",
                "integration_trigger_id": "value",
                "platform_metadata": {
                    "conversation_initiation_source": "plain_integration",
                    "thread_id": "value",
                    "workspace_id": "value"
                }
            },
            "whatsapp": {
                "direction": "inbound",
                "whatsapp_phone_number_id": "value",
                "whatsapp_user_id": "value",
                "awaiting_first_user_message": true,
                "recipient": {
                    "phone": "value",
                    "bsuid": "value"
                },
                "call_permission_request": {
                    "state": "blocked",
                    "requested_at_unix": 123,
                    "responded_at_unix": 123,
                    "permanent": true,
                    "granted_by_user_call": true
                },
                "call_result": "answered",
                "opening_message": {
                    "template_name": "value",
                    "wamid": "value",
                    "sent_at_unix": 123,
                    "delivered_at_unix": 123,
                    "read_at_unix": 123,
                    "replied_at_unix": 123,
                    "failed_at_unix": 123,
                    "error_code": "value",
                    "pricing_category": "value",
                    "billable": true
                }
            },
            "sms": {
                "direction": "inbound",
                "phone_number_id": "value",
                "sms_user_phone_number": "value",
                "agent_phone_number": "value"
            },
            "agent_created_from": "cli",
            "agent_last_updated_from": "cli",
            "voice_rewards": [
                {
                    "voice_id": "value",
                    "reward_usd_cents": 9.99
                }
            ],
            "cost_fiat": 9.99
        },
        "analysis": {
            "evaluation_criteria_results": {
                "key": {
                    "criteria_id": "value",
                    "result": "success",
                    "rationale": "value",
                    "scoring_mode": "binary",
                    "score": 123,
                    "max_score": 123
                }
            },
            "data_collection_results": {
                "key": {
                    "data_collection_id": "value",
                    "name": "value",
                    "value": {},
                    "json_schema": {
                        "type": "boolean",
                        "description": "value",
                        "enum": [
                            "value"
                        ],
                        "is_system_provided": true,
                        "dynamic_variable": "value",
                        "allowed_values": {
                            "dynamic_variable": "value"
                        },
                        "allowed_values_dynamic_variable": "value",
                        "constant_value": "value",
                        "is_omitted": true
                    },
                    "rationale": "value"
                }
            },
            "evaluation_criteria_results_list": [
                {
                    "criteria_id": "value",
                    "result": "success",
                    "rationale": "value",
                    "scoring_mode": "binary",
                    "score": 123,
                    "max_score": 123
                }
            ],
            "data_collection_results_list": [
                {
                    "data_collection_id": "value",
                    "name": "value",
                    "value": {},
                    "json_schema": {
                        "type": "boolean",
                        "description": "value",
                        "enum": [
                            "value"
                        ],
                        "is_system_provided": true,
                        "dynamic_variable": "value",
                        "allowed_values": {
                            "dynamic_variable": "value"
                        },
                        "allowed_values_dynamic_variable": "value",
                        "constant_value": "value",
                        "is_omitted": true
                    },
                    "rationale": "value"
                }
            ],
            "call_successful": "success",
            "call_success_score": 9.99,
            "transcript_summary": "value",
            "call_summary_title": "value",
            "scoped": [
                {
                    "scope": "conversation",
                    "source_agent_id": "value",
                    "source_branch_id": "value",
                    "evaluation_criteria_results": {
                        "key": {
                            "criteria_id": "value",
                            "result": "success",
                            "rationale": "value",
                            "scoring_mode": "binary",
                            "score": 123,
                            "max_score": 123
                        }
                    },
                    "data_collection_results": {
                        "key": {
                            "data_collection_id": "value",
                            "name": "value",
                            "value": {},
                            "json_schema": {
                                "type": "boolean",
                                "description": "value",
                                "enum": [
                                    "value"
                                ],
                                "is_system_provided": true,
                                "dynamic_variable": "value",
                                "allowed_values": {
                                    "dynamic_variable": "value"
                                },
                                "allowed_values_dynamic_variable": "value",
                                "constant_value": "value",
                                "is_omitted": true
                            },
                            "rationale": "value"
                        }
                    },
                    "successful": "success",
                    "success_score": 9.99
                }
            ]
        },
        "user_message_count": 123,
        "agent_message_count": 123,
        "visited_agents": [
            {
                "agent_id": "value",
                "branch_id": "value"
            }
        ],
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
        },
        "environment": "value",
        "conversation_id": "value",
        "has_audio": true,
        "has_user_audio": true,
        "has_response_audio": true,
        "has_auxiliary_audio": true,
        "transcript": [
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
                            },
                            "output_total_reasoning_portion": {
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
                ],
                "file_input": {
                    "file_id": "value",
                    "original_filename": "value",
                    "mime_type": "value",
                    "file_url": "value"
                },
                "file_inputs": [
                    {
                        "file_id": "value",
                        "original_filename": "value",
                        "mime_type": "value",
                        "file_url": "value"
                    }
                ],
                "contextual_update_info": {
                    "context_id": "value",
                    "is_superseded": true
                },
                "reasoned": true
            }
        ],
        "tag_ids": [
            "value"
        ],
        "otlp_traces": {}
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/conversations/value/analysis/run"))
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

    let result = client.analysis.run("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/conversations/value/analysis/run"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/conversations/value/analysis/run request"
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
async fn run_surfaces_the_declared_422() {
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
        .and(path("/v1/convai/conversations/value/analysis/run"))
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

    let result = client.analysis.run("value", None).await;

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
async fn run_evaluation_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "agent_id": "value",
        "agent_name": "value",
        "conversation_product": "value",
        "status": "initiated",
        "user_id": "value",
        "branch_id": "value",
        "version_id": "value",
        "metadata": {
            "start_time_unix_secs": 123,
            "accepted_time_unix_secs": 123,
            "call_duration_secs": 123,
            "queue_wait_secs": 9.99,
            "cost": 123,
            "deletion_settings": {
                "deletion_time_unix_secs": 123,
                "deleted_logs_at_time_unix_secs": 123,
                "deleted_audio_at_time_unix_secs": 123,
                "deleted_transcript_at_time_unix_secs": 123,
                "delete_transcript_and_pii": true,
                "delete_audio": true
            },
            "feedback": {
                "type": "thumbs",
                "overall_score": "like",
                "likes": 123,
                "dislikes": 123,
                "rating": 123,
                "comment": "value"
            },
            "authorization_method": "invalid",
            "charging": {
                "dev_discount": true,
                "is_burst": true,
                "tier": "value",
                "llm_usage": {
                    "irreversible_generation": {
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
                                },
                                "output_total_reasoning_portion": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    },
                    "initiated_generation": {
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
                                },
                                "output_total_reasoning_portion": {
                                    "tokens": 123,
                                    "price": 9.99
                                }
                            }
                        }
                    }
                },
                "llm_price": 9.99,
                "llm_charge": 123,
                "call_charge": 123,
                "platform_charge": 123,
                "platform_usage": {
                    "category_usage": {
                        "key": {
                            "credits": 123,
                            "price": 9.99,
                            "quantity": 9.99
                        }
                    }
                },
                "platform_price": 9.99,
                "free_minutes_consumed": 9.99,
                "free_llm_dollars_consumed": 9.99,
                "tts_usage": {
                    "primary_tts_model": "value",
                    "total_audio_output_seconds": 9.99,
                    "total_characters": 123,
                    "per_voice_usage": [
                        {
                            "voice_id": "value",
                            "audio_output_seconds": 9.99
                        }
                    ]
                },
                "asr_usage": {
                    "asr_model": "value",
                    "total_transcription_calls": 123,
                    "total_audio_input_seconds": 9.99
                },
                "analysis": {
                    "total": {
                        "price": 9.99,
                        "charge": 123,
                        "runs": 123,
                        "price_per_feature": {
                            "key": 9.99
                        },
                        "charge_per_feature": {
                            "key": 123
                        }
                    },
                    "last_run": {
                        "price": 9.99,
                        "charge": 123,
                        "price_per_feature": {
                            "key": 9.99
                        },
                        "charge_per_feature": {
                            "key": 123
                        }
                    }
                },
                "voice_surcharge_charge": 123
            },
            "phone_call": {
                "direction": "inbound",
                "phone_number_id": "value",
                "agent_number": "value",
                "external_number": "value",
                "call_result": "answered",
                "type": "twilio",
                "stream_sid": "value",
                "call_sid": "value"
            },
            "batch_call": {
                "batch_call_id": "value",
                "batch_call_recipient_id": "value",
                "campaign": {
                    "campaign_id": "value",
                    "campaign_lead_id": "value",
                    "cold_attempt": 123,
                    "conversation_type": "initial"
                }
            },
            "termination_reason": "value",
            "error": {
                "code": 123,
                "reason": "value"
            },
            "warnings": [
                "value"
            ],
            "main_language": "value",
            "rag_usage": {
                "usage_count": 123,
                "embedding_model": "value"
            },
            "text_only": true,
            "features_usage": {
                "language_detection": {
                    "enabled": true,
                    "used": true
                },
                "transfer_to_agent": {
                    "enabled": true,
                    "used": true
                },
                "transfer_to_number": {
                    "enabled": true,
                    "used": true
                },
                "multivoice": {
                    "enabled": true,
                    "used": true
                },
                "dtmf_tones": {
                    "enabled": true,
                    "used": true
                },
                "external_mcp_servers": {
                    "enabled": true,
                    "used": true
                },
                "pii_zrm_workspace": true,
                "pii_zrm_agent": true,
                "tool_dynamic_variable_updates": {
                    "enabled": true,
                    "used": true
                },
                "is_livekit": true,
                "voicemail_detection": {
                    "enabled": true,
                    "used": true
                },
                "dtmf_input": {
                    "enabled": true,
                    "used": true
                },
                "workflow": {
                    "enabled": true,
                    "tool_node": {
                        "enabled": true,
                        "used": true
                    },
                    "standalone_agent_node": {
                        "enabled": true,
                        "used": true
                    },
                    "phone_number_node": {
                        "enabled": true,
                        "used": true
                    },
                    "end_node": {
                        "enabled": true,
                        "used": true
                    },
                    "override_agent_node": {
                        "enabled": true,
                        "used": true
                    }
                },
                "agent_testing": {
                    "enabled": true,
                    "tests_ran_after_last_modification": true,
                    "tests_ran_in_last_7_days": true
                },
                "versioning": {
                    "enabled": true,
                    "used": true
                },
                "file_input": {
                    "enabled": true,
                    "used": true
                },
                "freeform_procedure": {
                    "enabled": true,
                    "used": true
                },
                "structured_procedure": {
                    "enabled": true,
                    "used": true
                }
            },
            "eleven_assistant": {
                "is_eleven_assistant": true
            },
            "initiator_id": "value",
            "conversation_initiation_source": "unknown",
            "conversation_initiation_source_version": "value",
            "timezone": "value",
            "async_metadata": {
                "delivery_status": "pending",
                "delivery_timestamp": 123,
                "delivery_error": "value",
                "external_system": "value",
                "external_id": "value",
                "external_link": "value",
                "retry_count": 123,
                "last_retry_timestamp": 123,
                "last_processed_external_message_id": "value",
                "integration_trigger_id": "value",
                "platform_metadata": {
                    "conversation_initiation_source": "plain_integration",
                    "thread_id": "value",
                    "workspace_id": "value"
                }
            },
            "whatsapp": {
                "direction": "inbound",
                "whatsapp_phone_number_id": "value",
                "whatsapp_user_id": "value",
                "awaiting_first_user_message": true,
                "recipient": {
                    "phone": "value",
                    "bsuid": "value"
                },
                "call_permission_request": {
                    "state": "blocked",
                    "requested_at_unix": 123,
                    "responded_at_unix": 123,
                    "permanent": true,
                    "granted_by_user_call": true
                },
                "call_result": "answered",
                "opening_message": {
                    "template_name": "value",
                    "wamid": "value",
                    "sent_at_unix": 123,
                    "delivered_at_unix": 123,
                    "read_at_unix": 123,
                    "replied_at_unix": 123,
                    "failed_at_unix": 123,
                    "error_code": "value",
                    "pricing_category": "value",
                    "billable": true
                }
            },
            "sms": {
                "direction": "inbound",
                "phone_number_id": "value",
                "sms_user_phone_number": "value",
                "agent_phone_number": "value"
            },
            "agent_created_from": "cli",
            "agent_last_updated_from": "cli",
            "voice_rewards": [
                {
                    "voice_id": "value",
                    "reward_usd_cents": 9.99
                }
            ],
            "cost_fiat": 9.99
        },
        "analysis": {
            "evaluation_criteria_results": {
                "key": {
                    "criteria_id": "value",
                    "result": "success",
                    "rationale": "value",
                    "scoring_mode": "binary",
                    "score": 123,
                    "max_score": 123
                }
            },
            "data_collection_results": {
                "key": {
                    "data_collection_id": "value",
                    "name": "value",
                    "value": {},
                    "json_schema": {
                        "type": "boolean",
                        "description": "value",
                        "enum": [
                            "value"
                        ],
                        "is_system_provided": true,
                        "dynamic_variable": "value",
                        "allowed_values": {
                            "dynamic_variable": "value"
                        },
                        "allowed_values_dynamic_variable": "value",
                        "constant_value": "value",
                        "is_omitted": true
                    },
                    "rationale": "value"
                }
            },
            "evaluation_criteria_results_list": [
                {
                    "criteria_id": "value",
                    "result": "success",
                    "rationale": "value",
                    "scoring_mode": "binary",
                    "score": 123,
                    "max_score": 123
                }
            ],
            "data_collection_results_list": [
                {
                    "data_collection_id": "value",
                    "name": "value",
                    "value": {},
                    "json_schema": {
                        "type": "boolean",
                        "description": "value",
                        "enum": [
                            "value"
                        ],
                        "is_system_provided": true,
                        "dynamic_variable": "value",
                        "allowed_values": {
                            "dynamic_variable": "value"
                        },
                        "allowed_values_dynamic_variable": "value",
                        "constant_value": "value",
                        "is_omitted": true
                    },
                    "rationale": "value"
                }
            ],
            "call_successful": "success",
            "call_success_score": 9.99,
            "transcript_summary": "value",
            "call_summary_title": "value",
            "scoped": [
                {
                    "scope": "conversation",
                    "source_agent_id": "value",
                    "source_branch_id": "value",
                    "evaluation_criteria_results": {
                        "key": {
                            "criteria_id": "value",
                            "result": "success",
                            "rationale": "value",
                            "scoring_mode": "binary",
                            "score": 123,
                            "max_score": 123
                        }
                    },
                    "data_collection_results": {
                        "key": {
                            "data_collection_id": "value",
                            "name": "value",
                            "value": {},
                            "json_schema": {
                                "type": "boolean",
                                "description": "value",
                                "enum": [
                                    "value"
                                ],
                                "is_system_provided": true,
                                "dynamic_variable": "value",
                                "allowed_values": {
                                    "dynamic_variable": "value"
                                },
                                "allowed_values_dynamic_variable": "value",
                                "constant_value": "value",
                                "is_omitted": true
                            },
                            "rationale": "value"
                        }
                    },
                    "successful": "success",
                    "success_score": 9.99
                }
            ]
        },
        "user_message_count": 123,
        "agent_message_count": 123,
        "visited_agents": [
            {
                "agent_id": "value",
                "branch_id": "value"
            }
        ],
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
        },
        "environment": "value",
        "conversation_id": "value",
        "has_audio": true,
        "has_user_audio": true,
        "has_response_audio": true,
        "has_auxiliary_audio": true,
        "transcript": [
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
                            },
                            "output_total_reasoning_portion": {
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
                ],
                "file_input": {
                    "file_id": "value",
                    "original_filename": "value",
                    "mime_type": "value",
                    "file_url": "value"
                },
                "file_inputs": [
                    {
                        "file_id": "value",
                        "original_filename": "value",
                        "mime_type": "value",
                        "file_url": "value"
                    }
                ],
                "contextual_update_info": {
                    "context_id": "value",
                    "is_superseded": true
                },
                "reasoned": true
            }
        ],
        "tag_ids": [
            "value"
        ],
        "otlp_traces": {}
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "evaluation_id": "value",
        "scope": "conversation",
        "evaluated_agent_id": "value",
        "evaluated_branch_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path(
            "/v1/convai/conversations/value/analysis/evaluations/run",
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
    let body = serde_json::from_value(request_body.clone()).unwrap();

    let result = client.analysis.run_evaluation("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/convai/conversations/value/analysis/evaluations/run"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/conversations/value/analysis/evaluations/run request"
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
async fn run_evaluation_surfaces_the_declared_422() {
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
        "evaluation_id": "value",
        "scope": "conversation",
        "evaluated_agent_id": "value",
        "evaluated_branch_id": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path(
            "/v1/convai/conversations/value/analysis/evaluations/run",
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
    let body = serde_json::from_value(request_body.clone()).unwrap();

    let result = client.analysis.run_evaluation("value", &body, None).await;

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
