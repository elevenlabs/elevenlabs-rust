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
        "speech_engine_id": "value",
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        },
        "metadata": {
            "created_at_unix_secs": 123,
            "updated_at_unix_secs": 123,
            "created_from": "cli",
            "last_updated_from": "cli"
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/speech-engine"))
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

    let result = client.speech_engine.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| entry.method.as_str() == "POST" && entry.url.path() == "/v1/speech-engine")
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/speech-engine request"
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
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/speech-engine"))
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

    let result = client.speech_engine.create(&body, None).await;

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
        "speech_engine_id": "value",
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        },
        "metadata": {
            "created_at_unix_secs": 123,
            "updated_at_unix_secs": 123,
            "created_from": "cli",
            "last_updated_from": "cli"
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/speech-engine/value"))
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

    let result = client.speech_engine.get("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET" && entry.url.path() == "/v1/speech-engine/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/speech-engine/value request"
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
        .and(path("/v1/speech-engine/value"))
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

    let result = client.speech_engine.get("value", None).await;

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
        "speech_engine_id": "value",
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        },
        "metadata": {
            "created_at_unix_secs": 123,
            "updated_at_unix_secs": 123,
            "created_from": "cli",
            "last_updated_from": "cli"
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/speech-engine/value"))
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

    let result = client.speech_engine.update("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH" && entry.url.path() == "/v1/speech-engine/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/speech-engine/value request"
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
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/speech-engine/value"))
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

    let result = client.speech_engine.update("value", &body, None).await;

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
        .and(path("/v1/speech-engine/value"))
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

    let result = client.speech_engine.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE" && entry.url.path() == "/v1/speech-engine/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/speech-engine/value request"
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
        .and(path("/v1/speech-engine/value"))
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

    let result = client.speech_engine.delete("value", None).await;

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
async fn duplicate_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "speech_engine_id": "value",
        "name": "value",
        "speech_engine": {
            "ws_url": "value",
            "request_headers": {
                "key": "value"
            }
        },
        "asr": {
            "quality": "high",
            "provider": "elevenlabs",
            "user_input_audio_format": "pcm_8000",
            "keywords": [
                "value"
            ]
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
            "expressive_mode": true,
            "suggested_audio_tags": [
                {
                    "tag": "value",
                    "description": "value"
                }
            ],
            "agent_output_audio_format": "pcm_8000",
            "optimize_streaming_latency": 0,
            "stability": 0.5,
            "speed": 1.2,
            "similarity_boost": 0.5,
            "clone_strength": 0.5,
            "text_normalisation_type": "system_prompt",
            "pronunciation_dictionary_locators": [
                {
                    "pronunciation_dictionary_id": "value",
                    "version_id": "value"
                }
            ],
            "enable_phoneme_tags": true,
            "audio_effects": {
                "filter_preset_id": "value",
                "distance": 0.5,
                "environment_id": "value",
                "background_noise_id": "value",
                "send_level": 0.5,
                "pan": 0.5
            }
        },
        "turn": {
            "turn_timeout": 9.99,
            "initial_wait_time": 9.99,
            "silence_end_call_timeout": 9.99,
            "turn_eagerness": "patient",
            "spelling_patience": "auto",
            "speculative_turn": true,
            "retranscribe_on_turn_timeout": true,
            "turn_model": "turn_v2",
            "interruption_ignore_terms": [
                "value"
            ],
            "interruption_ignore_term_languages": [
                "value"
            ],
            "merge_with_default_ignore_terms": true,
            "transcribe_on_disabled_interruptions": true
        },
        "vad": {},
        "conversation": {
            "text_only": true,
            "max_duration_seconds": 123,
            "client_events": [
                "conversation_initiation_metadata"
            ],
            "file_input": {
                "enabled": true,
                "max_files_in_memory": 30,
                "max_files_per_conversation": 123
            },
            "monitoring_enabled": true,
            "monitoring_events": [
                "conversation_initiation_metadata"
            ],
            "dtmf_input_settings": {
                "dtmf_input_timeout": 9.99,
                "hash_terminator": true,
                "redact_input": true
            },
            "background_sound": {
                "source_type": "preset",
                "source_id": "office2",
                "volume": 0.5,
                "crossfade_loop": true
            },
            "source_attribution": true
        },
        "privacy": {
            "record_voice": true,
            "retention_days": 123,
            "delete_transcript_and_pii": true,
            "delete_audio": true,
            "apply_to_existing_conversations": true,
            "zero_retention_mode": true,
            "conversation_history_redaction": {
                "enabled": true,
                "entities": [
                    "name"
                ],
                "excluded_data_collection_ids": [
                    "value"
                ]
            }
        },
        "call_limits": {
            "agent_concurrency_limit": 123,
            "daily_limit": 123,
            "bursting_enabled": true
        },
        "auth": {
            "enable_auth": true
        },
        "language": "value",
        "cascade_timeout_seconds": 9.99,
        "tags": [
            "value"
        ],
        "overrides": {
            "first_message": true
        },
        "metadata": {
            "created_at_unix_secs": 123,
            "updated_at_unix_secs": 123,
            "created_from": "cli",
            "last_updated_from": "cli"
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/speech-engine/value/duplicate"))
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

    let result = client.speech_engine.duplicate("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/speech-engine/value/duplicate"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/speech-engine/value/duplicate request"
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
async fn duplicate_surfaces_the_declared_422() {
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
        "name": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/speech-engine/value/duplicate"))
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

    let result = client.speech_engine.duplicate("value", &body, None).await;

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
