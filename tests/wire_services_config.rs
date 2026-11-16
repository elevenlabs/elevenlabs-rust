use elevenlabs_rust_sdk::ApiError;
#[allow(unused_imports)]
use elevenlabs_rust_sdk::*;
use elevenlabs_rust_sdk::{ApiClient, ClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "version": 123,
        "config": {
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
            "agent_policies": {
                "key": {
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
                }
            },
            "whatsapp_voice_policy": {
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
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/convai/outbound-campaigns/config"))
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

    let result = client.config_param.get(None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/convai/outbound-campaigns/config"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/convai/outbound-campaigns/config request"
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
        .and(path("/v1/convai/outbound-campaigns/config"))
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

    let result = client.config_param.get(None).await;

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
        "version": 123,
        "config": {
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
            "agent_policies": {
                "key": {
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
                }
            },
            "whatsapp_voice_policy": {
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
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "expected_version": 123,
        "config": {
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
            "agent_policies": {
                "key": {
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
                }
            },
            "whatsapp_voice_policy": {
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
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PUT"))
        .and(path("/v1/convai/outbound-campaigns/config"))
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

    let result = client.config_param.update(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PUT"
                && entry.url.path() == "/v1/convai/outbound-campaigns/config"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PUT /v1/convai/outbound-campaigns/config request"
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
        "expected_version": 123,
        "config": {
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
            "agent_policies": {
                "key": {
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
                }
            },
            "whatsapp_voice_policy": {
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
            }
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PUT"))
        .and(path("/v1/convai/outbound-campaigns/config"))
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

    let result = client.config_param.update(&body, None).await;

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
