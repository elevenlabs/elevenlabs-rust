//! The hand-written `MusicDetailed::compose_detailed`, end to end against a mock server that answers
//! with a `multipart/mixed` body. Mirrors `tests/test_music_multipart.py` in the Python SDK.

use elevenlabs_rust_sdk::music_detailed::{MusicDetailed, DEFAULT_FILENAME};
use elevenlabs_rust_sdk::{
    ApiClient, ApiError, BodyComposeMusicWithADetailedResponseV1MusicDetailedPost, ClientConfig,
    ComposeDetailedOutputFormat, ComposeDetailedRequest,
};
use wiremock::matchers::{body_partial_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const BOUNDARY: &str = "--boundary123";

fn metadata() -> serde_json::Value {
    serde_json::json!({
        "composition_plan": {"positive_global_styles": ["pop"], "negative_global_styles": [], "sections": []},
        "song_metadata": {
            "title": "Test", "description": "A test song", "genres": ["pop"], "languages": ["en"], "is_explicit": false
        },
        "words_timestamps": null
    })
}

/// Every byte value, so an off-by-one at either end of the audio part shows up.
fn audio() -> Vec<u8> {
    (0..=255u8).cycle().take(1024).collect()
}

fn multipart(line_break: &str, json: Option<String>, disposition: bool) -> Vec<u8> {
    let json = json.unwrap_or_else(|| metadata().to_string());
    let mut head = format!(
        "{BOUNDARY}{line_break}Content-Type: application/json{line_break}{line_break}{json}{line_break}{BOUNDARY}{line_break}Content-Type: audio/mpeg{line_break}"
    );
    if disposition {
        head.push_str(&format!(
            "Content-Disposition: attachment; filename=\"test_song.mp3\"{line_break}"
        ));
    }
    head.push_str(line_break);
    let mut body = head.into_bytes();
    body.extend(audio());
    body.extend(format!("{line_break}{BOUNDARY}--{line_break}").into_bytes());
    body
}

fn reply(body: Vec<u8>) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(body, "multipart/mixed; boundary=boundary123")
}

fn request() -> ComposeDetailedRequest {
    ComposeDetailedRequest {
        output_format: Some(ComposeDetailedOutputFormat::Mp3_44100_128),
        body: Some(BodyComposeMusicWithADetailedResponseV1MusicDetailedPost {
            prompt: Some("Epic orchestral music".into()),
            music_length_ms: Some(10_000),
            ..Default::default()
        }),
        ..Default::default()
    }
}

async fn music(server: &MockServer) -> MusicDetailed {
    let config = ClientConfig::builder()
        .base_url(server.uri())
        .api_key("test_api_key")
        .max_retries(0)
        .build()
        .expect("config should build");
    MusicDetailed::new(&ApiClient::new(config).expect("client should construct"))
}

#[tokio::test]
async fn parses_metadata_and_exact_audio_with_crlf_line_endings() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(multipart("\r\n", None, true)))
        .mount(&server)
        .await;

    let song = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();

    assert_eq!(song.json, metadata());
    assert_eq!(
        song.audio,
        audio(),
        "audio must be byte-for-byte, with the closing delimiter stripped"
    );
    assert_eq!(song.filename, "test_song.mp3");
}

#[tokio::test]
async fn parses_lf_line_endings() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(multipart("\n", None, true)))
        .mount(&server)
        .await;

    let song = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();

    assert_eq!(song.json, metadata());
    assert_eq!(song.audio, audio());
}

#[tokio::test]
async fn sends_the_generated_request() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .and(query_param("output_format", "mp3_44100_128"))
        .and(body_partial_json(
            serde_json::json!({"prompt": "Epic orchestral music", "music_length_ms": 10000}),
        ))
        .respond_with(reply(multipart("\r\n", None, true)))
        .expect(1)
        .mount(&server)
        .await;

    music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();
}

#[tokio::test]
async fn returns_the_song_id_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(multipart("\r\n", None, true)).insert_header("song-id", "song_abc"))
        .mount(&server)
        .await;

    let song = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();

    assert_eq!(song.song_id.as_deref(), Some("song_abc"));
}

#[tokio::test]
async fn song_id_is_none_without_the_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(multipart("\r\n", None, true)))
        .mount(&server)
        .await;

    let song = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();

    assert_eq!(song.song_id, None);
}

#[tokio::test]
async fn defaults_the_filename_without_content_disposition() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(multipart("\r\n", None, false)))
        .mount(&server)
        .await;

    let song = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();

    assert_eq!(song.filename, DEFAULT_FILENAME);
    assert_eq!(song.audio, audio());
}

#[tokio::test]
async fn parses_pretty_printed_json() {
    let server = MockServer::start().await;
    let pretty = serde_json::to_string_pretty(&metadata())
        .unwrap()
        .replace('\n', "\r\n");
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(multipart("\r\n", Some(pretty), true)))
        .mount(&server)
        .await;

    let song = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap();

    assert_eq!(song.json, metadata());
}

#[tokio::test]
async fn errors_without_a_json_part() {
    let server = MockServer::start().await;
    let mut body = format!("{BOUNDARY}\r\nContent-Type: audio/mpeg\r\n\r\n").into_bytes();
    body.extend(audio());
    body.extend(format!("\r\n{BOUNDARY}--\r\n").into_bytes());
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(body))
        .mount(&server)
        .await;

    let error = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap_err();

    match error {
        ApiError::DecodeError { message, .. } => {
            assert!(message.contains("JSON part"), "{message}")
        }
        other => panic!("expected DecodeError, got {other:?}"),
    }
}

#[tokio::test]
async fn errors_without_an_audio_part() {
    let server = MockServer::start().await;
    let body =
        format!("{BOUNDARY}\r\nContent-Type: application/json\r\n\r\n{{}}\r\n{BOUNDARY}--\r\n")
            .into_bytes();
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(reply(body))
        .mount(&server)
        .await;

    let error = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap_err();

    match error {
        ApiError::DecodeError { message, .. } => {
            assert!(message.contains("audio part"), "{message}")
        }
        other => panic!("expected DecodeError, got {other:?}"),
    }
}

#[tokio::test]
async fn api_errors_keep_the_generated_error_handling() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/music/detailed"))
        .respond_with(
            ResponseTemplate::new(422).set_body_raw(r#"{"detail":[]}"#, "application/json"),
        )
        .mount(&server)
        .await;

    let error = music(&server)
        .await
        .compose_detailed(&request(), None)
        .await
        .unwrap_err();

    assert!(
        matches!(
            error,
            ApiError::UnprocessableEntityError(_) | ApiError::Http { status: 422, .. }
        ),
        "expected the generated 422 mapping, got {error:?}"
    );
}
