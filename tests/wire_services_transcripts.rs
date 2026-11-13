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
        "transcript_format": "srt",
        "srt": "value",
        "webvtt": "value",
        "json": {
            "language": "value",
            "utterances": [
                {
                    "text": "value",
                    "speaker_id": "value",
                    "start_s": 9.99,
                    "end_s": 9.99,
                    "words": [
                        {
                            "text": "value",
                            "word_type": "value",
                            "start_s": 9.99,
                            "end_s": 9.99,
                            "characters": [
                                {
                                    "text": "value",
                                    "start_s": 9.99,
                                    "end_s": 9.99
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/dubbing/value/transcripts/value/format/srt"))
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
        .transcripts
        .get("value", "value", &FormatType::Srt, None)
        .await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/dubbing/value/transcripts/value/format/srt"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/dubbing/value/transcripts/value/format/srt request"
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
        .and(path("/v1/dubbing/value/transcripts/value/format/srt"))
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
        .transcripts
        .get("value", "value", &FormatType::Srt, None)
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
        "language_code": "value",
        "language_probability": 9.99,
        "text": "value",
        "words": [
            {
                "text": "value",
                "start": 9.99,
                "end": 9.99,
                "type": "word",
                "speaker_id": "value",
                "logprob": 9.99,
                "characters": [
                    {
                        "text": "value",
                        "start": 9.99,
                        "end": 9.99
                    }
                ],
                "channel_index": 123
            }
        ],
        "channel_index": 123,
        "additional_formats": [
            {
                "requested_format": "value",
                "file_extension": "value",
                "content_type": "value",
                "is_base64_encoded": true,
                "content": "value"
            }
        ],
        "transcription_id": "value",
        "entities": [
            {
                "text": "value",
                "entity_type": "value",
                "start_char": 123,
                "end_char": 123
            }
        ],
        "audio_duration_secs": 9.99,
        "edited_transcript": {
            "kind": "transcript",
            "text": "value",
            "edited_text": "value"
        }
    }"#,
    )
    .expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/speech-to-text/transcripts/value"))
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

    let result = client.transcripts.get_2("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "GET"
                && entry.url.path() == "/v1/speech-to-text/transcripts/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one GET /v1/speech-to-text/transcripts/value request"
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
async fn get_surfaces_the_declared_401() {
    let server = MockServer::start().await;
    let error_body =
        serde_json::from_str::<serde_json::Value>(r#"{}"#).expect("fixture is valid JSON");

    Mock::given(method("GET"))
        .and(path("/v1/speech-to-text/transcripts/value"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.transcripts.get_2("value", None).await;

    match result {
        Ok(_) => panic!("expected the declared 401 to surface as an error"),
        Err(ApiError::Http { status, message }) => {
            assert_eq!(status, 401);
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&message)
                    .expect("error body should be JSON"),
                error_body
            );
        }
        Err(other) => panic!("expected ApiError::Http, got {:?}", other),
    }
}

#[tokio::test]
async fn delete_sends_the_real_wire_request() {
    let server = MockServer::start().await;
    let expected_response =
        serde_json::from_str::<serde_json::Value>(r#"{}"#).expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/speech-to-text/transcripts/value"))
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

    let result = client.transcripts.delete("value", None).await;
    assert!(result.is_ok(), "call failed: {:?}", result.as_ref().err());

    let received = server.received_requests().await.expect("requests recorded");
    let matched: Vec<_> = received
        .iter()
        .filter(|entry| {
            entry.method.as_str() == "DELETE"
                && entry.url.path() == "/v1/speech-to-text/transcripts/value"
        })
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "expected exactly one DELETE /v1/speech-to-text/transcripts/value request"
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
async fn delete_surfaces_the_declared_401() {
    let server = MockServer::start().await;
    let error_body =
        serde_json::from_str::<serde_json::Value>(r#"{}"#).expect("fixture is valid JSON");

    Mock::given(method("DELETE"))
        .and(path("/v1/speech-to-text/transcripts/value"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .expect("config should build");
    let client = ApiClient::new(config).expect("client should construct");

    let result = client.transcripts.delete("value", None).await;

    match result {
        Ok(_) => panic!("expected the declared 401 to surface as an error"),
        Err(ApiError::Http { status, message }) => {
            assert_eq!(status, 401);
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&message)
                    .expect("error body should be JSON"),
                error_body
            );
        }
        Err(other) => panic!("expected ApiError::Http, got {:?}", other),
    }
}
