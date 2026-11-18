//! `create_and_wait` through the public API with the real clock. The pacing itself is covered,
//! without sleeping, by the unit tests in `src/flows/tests.rs`.

use std::time::{Duration, Instant};

use elevenlabs_rust_sdk::flows::{FlowsWaitError, FlowsWaitOptions};
use elevenlabs_rust_sdk::{ApiClient, ClientConfig, ImageClient, ImageGenerationRequest};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(server: &MockServer) -> ApiClient {
    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .build()
        .unwrap();
    ApiClient::new(config).unwrap()
}

fn image_request() -> ImageGenerationRequest {
    serde_json::from_value(json!({"prompt": "a lighthouse", "model_id": "gpt-image-1"})).unwrap()
}

async fn mount_create(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/v1/flows/image"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"id": "gen_1", "status": "pending"})),
        )
        .mount(server)
        .await;
}

#[tokio::test]
async fn waits_for_the_min_interval_then_returns_the_completed_generation() {
    let server = MockServer::start().await;
    mount_create(&server).await;
    Mock::given(method("GET"))
        .and(path("/v1/flows/image/gen_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "gen_1",
            "status": "completed",
            "content_url": "https://example.com/out.png",
            "content_mime_type": "image/png"
        })))
        .mount(&server)
        .await;

    let started = Instant::now();
    let result = client(&server)
        .flows()
        .image()
        .create_and_wait(
            &image_request(),
            None,
            FlowsWaitOptions::default().min_poll_interval(Duration::from_millis(500)),
        )
        .await
        .unwrap();

    assert!(result.is_completed());
    let waited = started.elapsed();
    // 0.5s plus at most 20% jitter, with headroom for the requests themselves.
    assert!(waited >= Duration::from_millis(500), "{waited:?}");
    assert!(waited < Duration::from_secs(3), "{waited:?}");
}

#[tokio::test]
async fn a_replaced_sub_client_never_receives_the_root_credentials() {
    let root_server = MockServer::start().await;
    let other_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/flows/image"))
        .and(header("xi-api-key", "test_api_key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"id": "gen_1", "status": "pending"})),
        )
        .expect(1)
        .mount(&root_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/flows/image/gen_1"))
        .and(header("xi-api-key", "test_api_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "gen_1",
            "status": "completed",
            "content_url": "https://example.com/out.png",
            "content_mime_type": "image/png"
        })))
        .expect(1)
        .mount(&root_server)
        .await;

    let mut client = client(&root_server);
    client.image = ImageClient::new(
        ClientConfig::builder()
            .base_url(other_server.uri())
            .api_key("other_api_key")
            .build()
            .unwrap(),
    )
    .unwrap();

    let result = client
        .flows()
        .image()
        .create_and_wait(
            &image_request(),
            None,
            FlowsWaitOptions::default().min_poll_interval(Duration::from_millis(500)),
        )
        .await
        .unwrap();

    assert!(result.is_completed());
    let stray = other_server.received_requests().await.unwrap_or_default();
    assert!(
        stray.is_empty(),
        "{} request(s) reached the replaced sub-client's base URL",
        stray.len()
    );
}

#[tokio::test]
async fn zero_wait_timeout_returns_a_timeout_after_the_create() {
    let server = MockServer::start().await;
    mount_create(&server).await;

    let err = client(&server)
        .flows()
        .image()
        .create_and_wait(
            &image_request(),
            None,
            FlowsWaitOptions::default().wait_timeout(Duration::ZERO),
        )
        .await
        .unwrap_err();

    match err {
        FlowsWaitError::Timeout(timeout) => {
            assert_eq!(timeout.id, "gen_1");
            assert_eq!(timeout.last_status, "pending");
        }
        other => panic!("expected a timeout, got {other:?}"),
    }
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
