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
        "tool_config": {
            "type": "webhook",
            "name": "value",
            "description": "value",
            "response_timeout_secs": 123,
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "assignments": [
                {
                    "source": "response",
                    "dynamic_variable": "value",
                    "value_path": "value",
                    "sanitize": true,
                    "preserve_native_type": true
                }
            ],
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "tool_error_handling_mode": "auto",
            "dynamic_variables": {
                "dynamic_variable_placeholders": {}
            },
            "execution_mode": "immediate",
            "api_schema": {
                "request_headers": {
                    "key": "value"
                },
                "url": "value",
                "method": "GET",
                "path_params_schema": {
                    "key": {
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
                    }
                },
                "query_params_schema": {
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required": [
                        "value"
                    ]
                },
                "request_body_schema": {
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_body_schema": {
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_filter": {
                    "mode": "all",
                    "filters": [
                        "value"
                    ],
                    "content_type": "application/json"
                },
                "content_type": "application/json",
                "auth_resolved_params": [
                    "value"
                ],
                "auth_connection": {
                    "auth_connection_id": "value"
                }
            },
            "follow_redirects": true,
            "follow_redirects_allowed_domains": [
                "value"
            ]
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "usage_stats": {
            "total_calls": 123,
            "avg_latency_secs": 9.99
        },
        "response_mocks": [
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
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "tool_config": {
            "type": "webhook",
            "name": "value",
            "description": "value",
            "response_timeout_secs": 123,
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "assignments": [
                {
                    "source": "response",
                    "dynamic_variable": "value",
                    "value_path": "value",
                    "sanitize": true,
                    "preserve_native_type": true
                }
            ],
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "tool_error_handling_mode": "auto",
            "dynamic_variables": {
                "dynamic_variable_placeholders": {}
            },
            "execution_mode": "immediate",
            "api_schema": {
                "request_headers": {
                    "key": "value"
                },
                "url": "value",
                "method": "GET",
                "path_params_schema": {
                    "key": {
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
                    }
                },
                "query_params_schema": {
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required": [
                        "value"
                    ]
                },
                "request_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_filter": {
                    "mode": "all",
                    "filters": [
                        "value"
                    ],
                    "content_type": "application/json"
                },
                "content_type": "application/json",
                "auth_resolved_params": [
                    "value"
                ],
                "auth_connection": {
                    "auth_connection_id": "value"
                }
            },
            "follow_redirects": true,
            "follow_redirects_allowed_domains": [
                "value"
            ]
        },
        "response_mocks": [
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
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/tools"))
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

    let result = client.tools.create(&body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| entry.method.as_str() == "POST" && entry.url.path() == "/v1/convai/tools")
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/convai/tools request"
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
        "tool_config": {
            "type": "webhook",
            "name": "value",
            "description": "value",
            "response_timeout_secs": 123,
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "assignments": [
                {
                    "source": "response",
                    "dynamic_variable": "value",
                    "value_path": "value",
                    "sanitize": true,
                    "preserve_native_type": true
                }
            ],
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "tool_error_handling_mode": "auto",
            "dynamic_variables": {
                "dynamic_variable_placeholders": {}
            },
            "execution_mode": "immediate",
            "api_schema": {
                "request_headers": {
                    "key": "value"
                },
                "url": "value",
                "method": "GET",
                "path_params_schema": {
                    "key": {
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
                    }
                },
                "query_params_schema": {
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required": [
                        "value"
                    ]
                },
                "request_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_filter": {
                    "mode": "all",
                    "filters": [
                        "value"
                    ],
                    "content_type": "application/json"
                },
                "content_type": "application/json",
                "auth_resolved_params": [
                    "value"
                ],
                "auth_connection": {
                    "auth_connection_id": "value"
                }
            },
            "follow_redirects": true,
            "follow_redirects_allowed_domains": [
                "value"
            ]
        },
        "response_mocks": [
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
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/convai/tools"))
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

    let result = client.tools.create(&body, None).await;

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
        "tool_config": {
            "type": "webhook",
            "name": "value",
            "description": "value",
            "response_timeout_secs": 123,
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "assignments": [
                {
                    "source": "response",
                    "dynamic_variable": "value",
                    "value_path": "value",
                    "sanitize": true,
                    "preserve_native_type": true
                }
            ],
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "tool_error_handling_mode": "auto",
            "dynamic_variables": {
                "dynamic_variable_placeholders": {}
            },
            "execution_mode": "immediate",
            "api_schema": {
                "request_headers": {
                    "key": "value"
                },
                "url": "value",
                "method": "GET",
                "path_params_schema": {
                    "key": {
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
                    }
                },
                "query_params_schema": {
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required": [
                        "value"
                    ]
                },
                "request_body_schema": {
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_body_schema": {
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_filter": {
                    "mode": "all",
                    "filters": [
                        "value"
                    ],
                    "content_type": "application/json"
                },
                "content_type": "application/json",
                "auth_resolved_params": [
                    "value"
                ],
                "auth_connection": {
                    "auth_connection_id": "value"
                }
            },
            "follow_redirects": true,
            "follow_redirects_allowed_domains": [
                "value"
            ]
        },
        "access_info": {
            "is_creator": true,
            "creator_name": "value",
            "creator_email": "value",
            "role": "admin",
            "anonymous_access_level_override": "admin",
            "access_source": "creator"
        },
        "usage_stats": {
            "total_calls": 123,
            "avg_latency_secs": 9.99
        },
        "response_mocks": [
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
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "tool_config": {
            "type": "webhook",
            "name": "value",
            "description": "value",
            "response_timeout_secs": 123,
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "assignments": [
                {
                    "source": "response",
                    "dynamic_variable": "value",
                    "value_path": "value",
                    "sanitize": true,
                    "preserve_native_type": true
                }
            ],
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "tool_error_handling_mode": "auto",
            "dynamic_variables": {
                "dynamic_variable_placeholders": {}
            },
            "execution_mode": "immediate",
            "api_schema": {
                "request_headers": {
                    "key": "value"
                },
                "url": "value",
                "method": "GET",
                "path_params_schema": {
                    "key": {
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
                    }
                },
                "query_params_schema": {
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required": [
                        "value"
                    ]
                },
                "request_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_filter": {
                    "mode": "all",
                    "filters": [
                        "value"
                    ],
                    "content_type": "application/json"
                },
                "content_type": "application/json",
                "auth_resolved_params": [
                    "value"
                ],
                "auth_connection": {
                    "auth_connection_id": "value"
                }
            },
            "follow_redirects": true,
            "follow_redirects_allowed_domains": [
                "value"
            ]
        },
        "response_mocks": [
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
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/convai/tools/value"))
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

    let result = client.tools.update("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "PATCH" && entry.url.path() == "/v1/convai/tools/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one PATCH /v1/convai/tools/value request"
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
        "tool_config": {
            "type": "webhook",
            "name": "value",
            "description": "value",
            "response_timeout_secs": 123,
            "disable_interruptions": true,
            "interruption_mode": "allow",
            "force_pre_tool_speech": true,
            "pre_tool_speech": "auto",
            "assignments": [
                {
                    "source": "response",
                    "dynamic_variable": "value",
                    "value_path": "value",
                    "sanitize": true,
                    "preserve_native_type": true
                }
            ],
            "tool_call_sound": "typing",
            "tool_call_sound_behavior": "auto",
            "tool_error_handling_mode": "auto",
            "dynamic_variables": {
                "dynamic_variable_placeholders": {}
            },
            "execution_mode": "immediate",
            "api_schema": {
                "request_headers": {
                    "key": "value"
                },
                "url": "value",
                "method": "GET",
                "path_params_schema": {
                    "key": {
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
                    }
                },
                "query_params_schema": {
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required": [
                        "value"
                    ]
                },
                "request_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_body_schema": {
                    "property_kind": "array",
                    "description": "value",
                    "dynamic_variable": "value",
                    "constant_value": {},
                    "is_omitted": true,
                    "type": "object",
                    "required": [
                        "value"
                    ],
                    "properties": {
                        "key": {
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
                        }
                    },
                    "required_constraints": {
                        "any_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ],
                        "all_of": [
                            {
                                "required": [
                                    "value"
                                ]
                            }
                        ]
                    }
                },
                "response_filter": {
                    "mode": "all",
                    "filters": [
                        "value"
                    ],
                    "content_type": "application/json"
                },
                "content_type": "application/json",
                "auth_resolved_params": [
                    "value"
                ],
                "auth_connection": {
                    "auth_connection_id": "value"
                }
            },
            "follow_redirects": true,
            "follow_redirects_allowed_domains": [
                "value"
            ]
        },
        "response_mocks": [
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
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("PATCH"))
        .and(path("/v1/convai/tools/value"))
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

    let result = client.tools.update("value", &body, None).await;

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
