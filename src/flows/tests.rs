//! `create_and_wait` against a mock server, with a fake clock so no test actually sleeps.

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::poll::{parse_retry_after, poll_delay, Pacer};
use super::{FlowsWaitError, FlowsWaitOptions};
use crate::api::types::{
    ImageGenerationRequest, MediaGenerationResponse, TemplateRunCreateRequest, TemplateRunStatus,
    TextToSpeechGenerationRequest, VideoGenerationRequest,
};
use crate::{ApiClient, ClientConfig, RequestOptions};

/// 2001-09-09T01:46:40Z, so the HTTP dates below are easy to read.
const NOW_SECS: u64 = 1_000_000_000;

struct FakePacer {
    elapsed: Mutex<Duration>,
    sleeps: Mutex<Vec<Duration>>,
    random: f64,
}

impl FakePacer {
    fn new() -> Self {
        Self::with_random(0.0)
    }

    fn with_random(random: f64) -> Self {
        Self {
            elapsed: Mutex::new(Duration::ZERO),
            sleeps: Mutex::new(Vec::new()),
            random,
        }
    }

    fn sleeps(&self) -> Vec<f64> {
        self.sleeps
            .lock()
            .unwrap()
            .iter()
            .map(Duration::as_secs_f64)
            .collect()
    }
}

impl Pacer for FakePacer {
    fn elapsed(&self) -> Duration {
        *self.elapsed.lock().unwrap()
    }

    fn now(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(NOW_SECS) + self.elapsed()
    }

    fn random(&self) -> f64 {
        self.random
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        self.sleeps.lock().unwrap().push(duration);
        *self.elapsed.lock().unwrap() += duration;
        Box::pin(async {})
    }
}

/// Mounts `replies` for `verb path`, answered once each in order.
async fn reply_in_order(
    server: &MockServer,
    verb: &str,
    route: &str,
    replies: Vec<(Value, Option<&str>)>,
) {
    for (body, retry_after) in replies {
        let mut template = ResponseTemplate::new(200).set_body_json(body);
        if let Some(value) = retry_after {
            template = template.insert_header("Retry-After", value);
        }
        Mock::given(method(verb))
            .and(path(route))
            .respond_with(template)
            .up_to_n_times(1)
            .mount(server)
            .await;
    }
}

fn client(server: &MockServer) -> ApiClient {
    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .max_retries(0)
        .build()
        .unwrap();
    ApiClient::new(config).unwrap()
}

async fn requests(server: &MockServer) -> Vec<(String, String)> {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .map(|r| (r.method.to_string(), r.url.path().to_owned()))
        .collect()
}

fn image_request() -> ImageGenerationRequest {
    serde_json::from_value(json!({
        "prompt": "a lighthouse at dusk",
        "quality": "low",
        "background": "transparent",
        "aspect_ratio": "1:1",
        "model_id": "gpt-image-1"
    }))
    .unwrap()
}

fn video_request() -> VideoGenerationRequest {
    serde_json::from_value(json!({
        "model_id": "creatify-aurora",
        "image": {"type": "generation", "generation_id": "img_1"},
        "audio": {"type": "generation", "generation_id": "aud_1"},
        "resolution": "480p"
    }))
    .unwrap()
}

fn tts_request() -> TextToSpeechGenerationRequest {
    serde_json::from_value(json!({
        "text": "hello",
        "voice": "JBFqnCBsd6RMkjVDRZzb",
        "model_id": "eleven_flash_v2_5"
    }))
    .unwrap()
}

fn run_request() -> TemplateRunCreateRequest {
    serde_json::from_value(json!({"inputs": {"prompt": "hello"}})).unwrap()
}

fn generation(status: &str) -> Value {
    match status {
        "completed" => json!({
            "id": "gen_1",
            "status": "completed",
            "content_url": "https://example.com/out.png",
            "content_mime_type": "image/png"
        }),
        "failed" => json!({
            "id": "gen_1",
            "status": "failed",
            "failure_reason": "internal_error",
            "error_message": "boom"
        }),
        other => json!({"id": "gen_1", "status": other}),
    }
}

fn run(status: &str) -> Value {
    json!({
        "id": "run_1",
        "template_id": "tpl_1",
        "version_id": "ver_1",
        "status": status,
        "outputs": {}
    })
}

fn headers(retry_after: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("retry-after", HeaderValue::from_str(retry_after).unwrap());
    headers
}

fn now() -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(NOW_SECS)
}

#[test]
fn poll_delay_uses_retry_after_with_jitter() {
    let pacer = FakePacer::with_random(0.5);
    let delay = poll_delay(&headers("5"), Duration::from_secs(1), &pacer);
    assert!((delay.as_secs_f64() - 5.5).abs() < 1e-9, "{delay:?}");
    let pacer = FakePacer::with_random(0.999_999);
    let delay = poll_delay(&HeaderMap::new(), Duration::from_secs(10), &pacer);
    assert!(delay > Duration::from_secs(10) && delay < Duration::from_secs(12));
}

#[test]
fn poll_delay_never_goes_below_the_min_interval() {
    let pacer = FakePacer::new();
    let two = Duration::from_secs(2);
    assert_eq!(poll_delay(&HeaderMap::new(), two, &pacer), two);
    assert_eq!(poll_delay(&headers("0"), two, &pacer), two);
    assert_eq!(poll_delay(&headers("1"), two, &pacer), two);
    assert_eq!(poll_delay(&headers("not-a-date"), two, &pacer), two);
}

#[test]
fn retry_after_accepts_seconds_and_http_dates() {
    assert_eq!(
        parse_retry_after(&headers(" 12 "), now()),
        Some(Duration::from_secs(12))
    );
    // 10s after NOW_SECS.
    assert_eq!(
        parse_retry_after(&headers("Sun, 09 Sep 2001 01:46:50 GMT"), now()),
        Some(Duration::from_secs(10))
    );
    // A date in the past is a wait of zero.
    assert_eq!(
        parse_retry_after(&headers("Sun, 09 Sep 2001 01:46:30 GMT"), now()),
        Some(Duration::ZERO)
    );
    assert_eq!(parse_retry_after(&headers("-5"), now()), None);
    assert_eq!(parse_retry_after(&headers("1.5"), now()), None);
    assert_eq!(parse_retry_after(&HeaderMap::new(), now()), None);

    let pacer = FakePacer::new();
    let three = Duration::from_secs(3);
    assert_eq!(
        poll_delay(&headers("Sun, 09 Sep 2001 01:46:48 GMT"), three, &pacer),
        Duration::from_secs(8)
    );
    assert_eq!(
        poll_delay(&headers("Sun, 09 Sep 2001 01:46:30 GMT"), three, &pacer),
        three
    );
}

#[tokio::test]
async fn image_create_and_wait_polls_until_completed() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(json!({"id": "gen_1", "status": "pending"}), Some("3"))],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/image/gen_1",
        vec![
            (generation("generating"), Some("7")),
            (generation("completed"), None),
        ],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let result = client
        .flows()
        .image()
        .create_and_wait_with(&image_request(), None, FlowsWaitOptions::default(), &pacer)
        .await
        .unwrap();

    assert_eq!(
        result.as_completed().unwrap().content_url,
        "https://example.com/out.png"
    );
    assert_eq!(pacer.sleeps(), vec![3.0, 7.0]);
    assert_eq!(
        requests(&server).await,
        vec![
            ("POST".into(), "/v1/flows/image".into()),
            ("GET".into(), "/v1/flows/image/gen_1".into()),
            ("GET".into(), "/v1/flows/image/gen_1".into()),
        ]
    );
}

#[tokio::test]
async fn retry_after_http_date_paces_the_status_checks() {
    let server = MockServer::start().await;
    // 10s after the fake clock's start.
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(
            json!({"id": "gen_1", "status": "pending"}),
            Some("Sun, 09 Sep 2001 01:46:50 GMT"),
        )],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/image/gen_1",
        vec![(generation("completed"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    client
        .flows()
        .image()
        .create_and_wait_with(&image_request(), None, FlowsWaitOptions::default(), &pacer)
        .await
        .unwrap();

    assert_eq!(pacer.sleeps(), vec![10.0]);
}

#[tokio::test]
async fn min_poll_interval_floors_a_shorter_retry_after() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(json!({"id": "gen_1", "status": "pending"}), Some("1"))],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/image/gen_1",
        vec![
            (generation("pending"), None),
            (generation("completed"), None),
        ],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    client
        .flows()
        .image()
        .create_and_wait_with(
            &image_request(),
            None,
            FlowsWaitOptions::default().min_poll_interval(Duration::from_secs(5)),
            &pacer,
        )
        .await
        .unwrap();

    // Retry-After 1 is raised to the floor; no header falls back to it.
    assert_eq!(pacer.sleeps(), vec![5.0, 5.0]);
}

#[tokio::test]
async fn failed_generation_is_returned_not_raised() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/video",
        vec![(json!({"id": "gen_1", "status": "pending"}), None)],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/video/gen_1",
        vec![(generation("failed"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let result = client
        .flows()
        .video()
        .create_and_wait_with(&video_request(), None, FlowsWaitOptions::default(), &pacer)
        .await
        .unwrap();

    assert!(matches!(result, MediaGenerationResponse::Failed { .. }));
    assert_eq!(pacer.sleeps(), vec![3.0]);
    assert_eq!(requests(&server).await[1].1, "/v1/flows/video/gen_1");
}

#[tokio::test]
async fn text_to_speech_create_and_wait_uses_the_flows_endpoints() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/text-to-speech",
        vec![(json!({"id": "gen_1", "status": "pending"}), Some("3"))],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/text-to-speech/gen_1",
        vec![(generation("completed"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let result = client
        .flows()
        .text_to_speech()
        .create_and_wait_with(
            &tts_request(),
            None,
            FlowsWaitOptions::default().wait_timeout(Duration::MAX),
            &pacer,
        )
        .await
        .unwrap();

    assert!(result.is_completed());
    assert_eq!(pacer.sleeps(), vec![3.0]);
}

#[tokio::test]
async fn timeout_returns_the_generation_id_and_last_status() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(json!({"id": "gen_1", "status": "pending"}), Some("30"))],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/image/gen_1",
        vec![(generation("generating"), Some("30"))],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let err = client
        .flows()
        .image()
        .create_and_wait_with(
            &image_request(),
            None,
            FlowsWaitOptions::default().wait_timeout(Duration::from_secs(10)),
            &pacer,
        )
        .await
        .unwrap_err();

    // The 30s hint is cut to the 10s left; the check after it is the last.
    assert_eq!(pacer.sleeps(), vec![10.0]);
    let FlowsWaitError::Timeout(timeout) = err else {
        panic!("expected a timeout, got {err:?}");
    };
    assert_eq!(timeout.id, "gen_1");
    assert_eq!(timeout.template_id, None);
    assert_eq!(timeout.last_status, "generating");
    assert_eq!(timeout.wait_timeout, Duration::from_secs(10));
    assert_eq!(
        timeout.to_string(),
        "Generation gen_1 is still generating after 10s; call `get` with this id to keep waiting"
    );
    assert_eq!(requests(&server).await.len(), 2);
}

#[tokio::test]
async fn zero_timeout_returns_straight_after_the_create() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(json!({"id": "gen_1", "status": "pending"}), Some("3"))],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let err = client
        .flows()
        .image()
        .create_and_wait_with(
            &image_request(),
            None,
            FlowsWaitOptions::default().wait_timeout(Duration::ZERO),
            &pacer,
        )
        .await
        .unwrap_err();

    let FlowsWaitError::Timeout(timeout) = err else {
        panic!("expected a timeout, got {err:?}");
    };
    assert_eq!(timeout.last_status, "pending");
    assert!(pacer.sleeps().is_empty());
    assert_eq!(requests(&server).await.len(), 1);
}

#[tokio::test]
async fn min_poll_interval_below_the_floor_is_rejected_before_any_request() {
    let server = MockServer::start().await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let err = client
        .flows()
        .image()
        .create_and_wait_with(
            &image_request(),
            None,
            FlowsWaitOptions::default().min_poll_interval(Duration::from_millis(499)),
            &pacer,
        )
        .await
        .unwrap_err();
    assert!(
        matches!(&err, FlowsWaitError::InvalidOptions(m) if m.contains("at least 0.5s")),
        "{err:?}"
    );

    let err = client
        .flows()
        .runs()
        .create_and_wait_with(
            "tpl_1",
            &run_request(),
            None,
            FlowsWaitOptions::default().min_poll_interval(Duration::ZERO),
            &pacer,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, FlowsWaitError::InvalidOptions(_)));
    assert!(requests(&server).await.is_empty());
}

#[tokio::test]
async fn template_run_create_and_wait() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/templates/tpl_1/runs",
        vec![(run("pending"), Some("2"))],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/templates/tpl_1/runs/run_1",
        vec![(run("completed"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let result = client
        .flows()
        .runs()
        .create_and_wait_with(
            "tpl_1",
            &run_request(),
            None,
            FlowsWaitOptions::default().min_poll_interval(Duration::from_secs(1)),
            &pacer,
        )
        .await
        .unwrap();

    assert_eq!(result.status, TemplateRunStatus::Completed);
    assert_eq!(pacer.sleeps(), vec![2.0]);
    assert_eq!(
        requests(&server).await,
        vec![
            ("POST".into(), "/v1/flows/templates/tpl_1/runs".into()),
            ("GET".into(), "/v1/flows/templates/tpl_1/runs/run_1".into()),
        ]
    );
}

#[tokio::test]
async fn template_run_already_terminal_on_create_skips_polling() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/templates/tpl_1/runs",
        vec![(run("failed"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let result = client
        .flows()
        .runs()
        .create_and_wait_with(
            "tpl_1",
            &run_request(),
            None,
            FlowsWaitOptions::default().wait_timeout(Duration::ZERO),
            &pacer,
        )
        .await
        .unwrap();

    assert_eq!(result.status, TemplateRunStatus::Failed);
    assert!(pacer.sleeps().is_empty());
    assert_eq!(requests(&server).await.len(), 1);
}

#[tokio::test]
async fn template_run_timeout_carries_the_template_id() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/templates/tpl_1/runs",
        vec![(run("pending"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let err = client
        .flows()
        .runs()
        .create_and_wait_with(
            "tpl_1",
            &run_request(),
            None,
            FlowsWaitOptions::default().wait_timeout(Duration::ZERO),
            &pacer,
        )
        .await
        .unwrap_err();

    let FlowsWaitError::Timeout(timeout) = err else {
        panic!("expected a timeout, got {err:?}");
    };
    assert_eq!(timeout.id, "run_1");
    assert_eq!(timeout.template_id.as_deref(), Some("tpl_1"));
    let message = timeout.to_string();
    assert!(message.contains("of template tpl_1"), "{message}");
    assert!(
        message.contains("`runs.get` with `template_id` and `id`"),
        "{message}"
    );
}

#[tokio::test]
async fn body_parameters_go_to_the_create_only() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(json!({"id": "gen_1", "status": "pending"}), None)],
    )
    .await;
    reply_in_order(
        &server,
        "GET",
        "/v1/flows/image/gen_1",
        vec![(generation("completed"), None)],
    )
    .await;
    let client = client(&server);
    let pacer = FakePacer::new();
    let options = RequestOptions::new()
        .additional_body_param("extra", 1)
        .additional_header("x-test", "1")
        .additional_query_param("q", "1");

    client
        .flows()
        .image()
        .create_and_wait_with(
            &image_request(),
            Some(options),
            FlowsWaitOptions::default(),
            &pacer,
        )
        .await
        .unwrap();

    let received = server.received_requests().await.unwrap();
    assert_eq!(received.len(), 2);
    let create_body: Value = serde_json::from_slice(&received[0].body).unwrap();
    assert_eq!(create_body["extra"], 1);
    assert_eq!(create_body["prompt"], "a lighthouse at dusk");
    assert!(received[1].body.is_empty());
    for request in &received {
        assert_eq!(request.headers.get("x-test").unwrap(), "1");
        assert_eq!(request.headers.get("xi-api-key").unwrap(), "test_api_key");
        assert_eq!(request.url.query(), Some("q=1"));
    }
}

#[tokio::test]
async fn a_failed_status_check_is_returned_as_an_api_error() {
    let server = MockServer::start().await;
    reply_in_order(
        &server,
        "POST",
        "/v1/flows/image",
        vec![(json!({"id": "gen_1", "status": "pending"}), None)],
    )
    .await;
    Mock::given(method("GET"))
        .and(path("/v1/flows/image/gen_1"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;
    let client = client(&server);
    let pacer = FakePacer::new();

    let err = client
        .flows()
        .image()
        .create_and_wait_with(&image_request(), None, FlowsWaitOptions::default(), &pacer)
        .await
        .unwrap_err();

    assert!(
        matches!(
            err,
            FlowsWaitError::Api(crate::ApiError::Http { status: 404, .. })
        ),
        "{err:?}"
    );
}
