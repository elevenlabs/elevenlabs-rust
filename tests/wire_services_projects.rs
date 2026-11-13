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
        r#"{
        "projects": [
            {
                "project_id": "value",
                "name": "value",
                "create_date_unix": 123,
                "created_by_user_id": "value",
                "default_title_voice_ref_id": "value",
                "default_paragraph_voice_ref_id": "value",
                "default_model_id": "value",
                "last_conversion_date_unix": 123,
                "can_be_downloaded": true,
                "title": "value",
                "author": "value",
                "description": "value",
                "genres": [
                    "value"
                ],
                "cover_image_url": "value",
                "target_audience": "children",
                "language": "value",
                "content_type": "value",
                "original_publication_date": "value",
                "mature_content": true,
                "isbn_number": "value",
                "volume_normalization": true,
                "state": "creating",
                "access_level": "admin",
                "fiction": "fiction",
                "quality_check_on": true,
                "quality_check_on_when_bulk_convert": true,
                "creation_meta": {
                    "creation_progress": 9.99,
                    "status": "draft",
                    "type": "blank"
                },
                "source_type": "blank",
                "chapters_enabled": true,
                "captions_enabled": true,
                "caption_style": {
                    "template": {
                        "key": "value",
                        "label": "value",
                        "requires_high_fps": true
                    },
                    "text_font": "value",
                    "text_scale": 9.99,
                    "text_color": "value",
                    "text_align": "start",
                    "text_style": "normal",
                    "text_weight": "normal",
                    "text_transform": "none",
                    "text_blend_mode": "normal",
                    "text_shadow": {
                        "enabled": true,
                        "color": "value",
                        "opacity": 9.99,
                        "blur": 9.99,
                        "offset_x": 9.99,
                        "offset_y": 9.99
                    },
                    "text_outline": {
                        "enabled": true,
                        "color": "value",
                        "opacity": 9.99,
                        "width": 9.99
                    },
                    "background_enabled": true,
                    "background_color": "value",
                    "background_opacity": 9.99,
                    "background_blur": 9.99,
                    "background_border_radius": 9.99,
                    "word_highlights_enabled": true,
                    "word_highlights_color": "value",
                    "word_highlights_background_color": "value",
                    "word_highlights_opacity": 9.99,
                    "word_highlights_border_radius": 9.99,
                    "word_highlights_blur": 9.99,
                    "section_animation": {
                        "enter_type": "none",
                        "exit_type": "none"
                    },
                    "word_animation": {
                        "enter_type": "none",
                        "exit_type": "none"
                    },
                    "character_animation": {
                        "enter_type": "none",
                        "exit_type": "none"
                    },
                    "cursor_enabled": true,
                    "width_pct": 9.99,
                    "horizontal_placement": {
                        "align": "left",
                        "translate_pct": 9.99
                    },
                    "vertical_placement": {
                        "align": "top",
                        "translate_pct": 9.99
                    },
                    "auto_break_enabled": true,
                    "max_lines_per_section": 123,
                    "max_words_per_line": 123
                },
                "caption_style_template_overrides": {
                    "key": {
                        "template": {
                            "key": "value",
                            "label": "value",
                            "requires_high_fps": true
                        },
                        "text_font": "value",
                        "text_scale": 9.99,
                        "text_color": "value",
                        "text_align": "start",
                        "text_style": "normal",
                        "text_weight": "normal",
                        "text_transform": "none",
                        "text_blend_mode": "normal",
                        "text_shadow": {
                            "enabled": true,
                            "color": "value",
                            "opacity": 9.99,
                            "blur": 9.99,
                            "offset_x": 9.99,
                            "offset_y": 9.99
                        },
                        "text_outline": {
                            "enabled": true,
                            "color": "value",
                            "opacity": 9.99,
                            "width": 9.99
                        },
                        "background_enabled": true,
                        "background_color": "value",
                        "background_opacity": 9.99,
                        "background_blur": 9.99,
                        "background_border_radius": 9.99,
                        "word_highlights_enabled": true,
                        "word_highlights_color": "value",
                        "word_highlights_background_color": "value",
                        "word_highlights_opacity": 9.99,
                        "word_highlights_border_radius": 9.99,
                        "word_highlights_blur": 9.99,
                        "section_animation": {
                            "enter_type": "none",
                            "exit_type": "none"
                        },
                        "word_animation": {
                            "enter_type": "none",
                            "exit_type": "none"
                        },
                        "character_animation": {
                            "enter_type": "none",
                            "exit_type": "none"
                        },
                        "cursor_enabled": true,
                        "width_pct": 9.99,
                        "horizontal_placement": {
                            "align": "left",
                            "translate_pct": 9.99
                        },
                        "vertical_placement": {
                            "align": "top",
                            "translate_pct": 9.99
                        },
                        "auto_break_enabled": true,
                        "max_lines_per_section": 123,
                        "max_words_per_line": 123
                    }
                },
                "public_share_id": "value",
                "aspect_ratio": "16:9",
                "agent_settings": {
                    "tool_settings": {
                        "key": {
                            "skip_confirmation": true
                        }
                    }
                },
                "default_title_voice_id": "value",
                "default_paragraph_voice_id": "value"
            }
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/studio/projects"))
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

    let result = client.projects.list(None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| entry.method.as_str() == "GET" && entry.url.path() == "/v1/studio/projects")
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/studio/projects request"
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
        .and(path("/v1/studio/projects"))
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

    let result = client.projects.list(None).await;

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
        "project": {
            "project_id": "value",
            "name": "value",
            "create_date_unix": 123,
            "created_by_user_id": "value",
            "default_title_voice_ref_id": "value",
            "default_paragraph_voice_ref_id": "value",
            "default_model_id": "value",
            "last_conversion_date_unix": 123,
            "can_be_downloaded": true,
            "title": "value",
            "author": "value",
            "description": "value",
            "genres": [
                "value"
            ],
            "cover_image_url": "value",
            "target_audience": "children",
            "language": "value",
            "content_type": "value",
            "original_publication_date": "value",
            "mature_content": true,
            "isbn_number": "value",
            "volume_normalization": true,
            "state": "creating",
            "access_level": "admin",
            "fiction": "fiction",
            "quality_check_on": true,
            "quality_check_on_when_bulk_convert": true,
            "creation_meta": {
                "creation_progress": 9.99,
                "status": "draft",
                "type": "blank"
            },
            "source_type": "blank",
            "chapters_enabled": true,
            "captions_enabled": true,
            "caption_style": {
                "template": {
                    "key": "value",
                    "label": "value",
                    "requires_high_fps": true
                },
                "text_font": "value",
                "text_scale": 9.99,
                "text_color": "value",
                "text_align": "start",
                "text_style": "normal",
                "text_weight": "normal",
                "text_transform": "none",
                "text_blend_mode": "normal",
                "text_shadow": {
                    "enabled": true,
                    "color": "value",
                    "opacity": 9.99,
                    "blur": 9.99,
                    "offset_x": 9.99,
                    "offset_y": 9.99
                },
                "text_outline": {
                    "enabled": true,
                    "color": "value",
                    "opacity": 9.99,
                    "width": 9.99
                },
                "background_enabled": true,
                "background_color": "value",
                "background_opacity": 9.99,
                "background_blur": 9.99,
                "background_border_radius": 9.99,
                "word_highlights_enabled": true,
                "word_highlights_color": "value",
                "word_highlights_background_color": "value",
                "word_highlights_opacity": 9.99,
                "word_highlights_border_radius": 9.99,
                "word_highlights_blur": 9.99,
                "section_animation": {
                    "enter_type": "none",
                    "exit_type": "none"
                },
                "word_animation": {
                    "enter_type": "none",
                    "exit_type": "none"
                },
                "character_animation": {
                    "enter_type": "none",
                    "exit_type": "none"
                },
                "cursor_enabled": true,
                "width_pct": 9.99,
                "horizontal_placement": {
                    "align": "left",
                    "translate_pct": 9.99
                },
                "vertical_placement": {
                    "align": "top",
                    "translate_pct": 9.99
                },
                "auto_break_enabled": true,
                "max_lines_per_section": 123,
                "max_words_per_line": 123
            },
            "caption_style_template_overrides": {
                "key": {
                    "template": {
                        "key": "value",
                        "label": "value",
                        "requires_high_fps": true
                    },
                    "text_font": "value",
                    "text_scale": 9.99,
                    "text_color": "value",
                    "text_align": "start",
                    "text_style": "normal",
                    "text_weight": "normal",
                    "text_transform": "none",
                    "text_blend_mode": "normal",
                    "text_shadow": {
                        "enabled": true,
                        "color": "value",
                        "opacity": 9.99,
                        "blur": 9.99,
                        "offset_x": 9.99,
                        "offset_y": 9.99
                    },
                    "text_outline": {
                        "enabled": true,
                        "color": "value",
                        "opacity": 9.99,
                        "width": 9.99
                    },
                    "background_enabled": true,
                    "background_color": "value",
                    "background_opacity": 9.99,
                    "background_blur": 9.99,
                    "background_border_radius": 9.99,
                    "word_highlights_enabled": true,
                    "word_highlights_color": "value",
                    "word_highlights_background_color": "value",
                    "word_highlights_opacity": 9.99,
                    "word_highlights_border_radius": 9.99,
                    "word_highlights_blur": 9.99,
                    "section_animation": {
                        "enter_type": "none",
                        "exit_type": "none"
                    },
                    "word_animation": {
                        "enter_type": "none",
                        "exit_type": "none"
                    },
                    "character_animation": {
                        "enter_type": "none",
                        "exit_type": "none"
                    },
                    "cursor_enabled": true,
                    "width_pct": 9.99,
                    "horizontal_placement": {
                        "align": "left",
                        "translate_pct": 9.99
                    },
                    "vertical_placement": {
                        "align": "top",
                        "translate_pct": 9.99
                    },
                    "auto_break_enabled": true,
                    "max_lines_per_section": 123,
                    "max_words_per_line": 123
                }
            },
            "public_share_id": "value",
            "aspect_ratio": "16:9",
            "agent_settings": {
                "tool_settings": {
                    "key": {
                        "skip_confirmation": true
                    }
                }
            },
            "default_title_voice_id": "value",
            "default_paragraph_voice_id": "value"
        }
    }"#,
    )
    .expect("fixture is valid JSON");
    let request_body = serde_json::from_str::<serde_json::Value>(
        r#"{
        "name": "value",
        "default_title_voice_id": "value",
        "default_paragraph_voice_id": "value",
        "title": "value",
        "author": "value",
        "isbn_number": "value",
        "volume_normalization": true
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/studio/projects/value"))
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

    let result = client.projects.update("value", &body, None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST" && entry.url.path() == "/v1/studio/projects/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/studio/projects/value request"
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
        "default_title_voice_id": "value",
        "default_paragraph_voice_id": "value",
        "title": "value",
        "author": "value",
        "isbn_number": "value",
        "volume_normalization": true
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/studio/projects/value"))
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

    let result = client.projects.update("value", &body, None).await;

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
        "status": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/studio/projects/value"))
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

    let result = client.projects.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE" && entry.url.path() == "/v1/studio/projects/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/studio/projects/value request"
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
        .and(path("/v1/studio/projects/value"))
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

    let result = client.projects.delete("value", None).await;

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
async fn convert_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "status": "value"
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("POST"))
        .and(path("/v1/studio/projects/value/convert"))
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

    let result = client.projects.convert("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "POST"
                && entry.url.path() == "/v1/studio/projects/value/convert"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one POST /v1/studio/projects/value/convert request"
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
async fn convert_surfaces_the_declared_422() {
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
        .and(path("/v1/studio/projects/value/convert"))
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

    let result = client.projects.convert("value", None).await;

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
async fn get_muted_tracks_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response = serde_json::from_str::<serde_json::Value>(
        r#"{
        "chapter_ids": [
            "value"
        ]
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/studio/projects/value/muted-tracks"))
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

    let result = client.projects.get_muted_tracks("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/studio/projects/value/muted-tracks"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/studio/projects/value/muted-tracks request"
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
async fn get_muted_tracks_surfaces_the_declared_422() {
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
        .and(path("/v1/studio/projects/value/muted-tracks"))
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

    let result = client.projects.get_muted_tracks("value", None).await;

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
