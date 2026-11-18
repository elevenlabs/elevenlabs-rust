//! Webhook signature verification, exercised through the crate's public API. Mirrors the PHP and
//! Ruby SDK tests case for case, including the shared cross-SDK golden vector.

use std::time::{SystemTime, UNIX_EPOCH};

use elevenlabs_rust_sdk::webhooks::{construct_event, WebhookError};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Shared cross-SDK golden vector. The timestamp is far in the future so it stays inside the
/// (past-only) tolerance window forever; every ElevenLabs SDK must accept this exact
/// (secret, header, body) triple.
const SECRET: &str = "wsec_0123456789abcdef0123456789abcdef";
const BODY: &str = r#"{"type":"post_call_transcription","event_timestamp":1234567890,"data":{"agent_id":"abc123"}}"#;
const HEADER: &str =
    "t=4102444800,v0=01fc9faf23a5c155fbab279c4e0acb2ae059a567b14c10a19bd9f99c88315e99";

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn sign(timestamp: &str, body: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(SECRET.as_bytes()).unwrap();
    mac.update(format!("{timestamp}.{body}").as_bytes());
    let hex: String = mac
        .finalize()
        .into_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    format!("t={timestamp},v0={hex}")
}

#[test]
fn golden_vector_verifies_and_decodes() {
    let event = construct_event(BODY, HEADER, SECRET).expect("golden vector must verify");

    assert_eq!(event["type"], "post_call_transcription");
    assert_eq!(event["event_timestamp"], 1234567890);
    assert_eq!(event["data"]["agent_id"], "abc123");
}

#[test]
fn golden_vector_verifies_from_raw_bytes() {
    let event = construct_event(BODY.as_bytes(), HEADER, SECRET).expect("bytes must verify");
    assert_eq!(event["type"], "post_call_transcription");
}

#[test]
fn current_timestamp_verifies() {
    let timestamp = now().to_string();
    let body = r#"{"type":"ping"}"#;

    let event = construct_event(body, &sign(&timestamp, body), SECRET).unwrap();
    assert_eq!(event["type"], "ping");
}

#[test]
fn tampered_signature_is_rejected() {
    let header = format!("t=4102444800,v0={}", "0".repeat(64));

    let err = construct_event(BODY, &header, SECRET).unwrap_err();
    assert!(matches!(err, WebhookError::SignatureMismatch));
    assert!(err.to_string().contains("Signature hash does not match"));
}

#[test]
fn tampered_body_is_rejected() {
    let err = construct_event(format!("{BODY} "), HEADER, SECRET).unwrap_err();
    assert!(matches!(err, WebhookError::SignatureMismatch));
}

#[test]
fn wrong_secret_is_rejected() {
    let err = construct_event(BODY, HEADER, "wsec_not_the_right_secret").unwrap_err();
    assert!(matches!(err, WebhookError::SignatureMismatch));
}

#[test]
fn old_timestamp_is_rejected() {
    let timestamp = (now() - 31 * 60).to_string();
    let body = r#"{"type":"ping"}"#;

    let err = construct_event(body, &sign(&timestamp, body), SECRET).unwrap_err();
    assert!(matches!(err, WebhookError::TimestampOutsideTolerance));
    assert_eq!(err.to_string(), "Timestamp outside the tolerance zone");
}

#[test]
fn missing_header_is_rejected() {
    let err = construct_event(BODY, "", SECRET).unwrap_err();
    assert!(matches!(err, WebhookError::MissingSignatureHeader));
    assert_eq!(err.to_string(), "Missing signature header");
}

#[test]
fn missing_secret_is_rejected() {
    let err = construct_event(BODY, HEADER, "").unwrap_err();
    assert!(matches!(err, WebhookError::MissingSecret));
    assert_eq!(err.to_string(), "Webhook secret not configured");
}

#[test]
fn header_without_v0_scheme_is_rejected() {
    let err = construct_event(BODY, "t=4102444800", SECRET).unwrap_err();
    assert!(matches!(err, WebhookError::MissingSignatureScheme));
    assert_eq!(
        err.to_string(),
        "No signature hash found with expected scheme v0"
    );
}

#[test]
fn header_without_timestamp_is_rejected() {
    let err = construct_event(
        BODY,
        "v0=01fc9faf23a5c155fbab279c4e0acb2ae059a567b14c10a19bd9f99c88315e99",
        SECRET,
    )
    .unwrap_err();
    assert!(matches!(err, WebhookError::MissingSignatureScheme));
}

#[test]
fn valid_signature_over_non_json_body_is_an_invalid_payload() {
    let timestamp = now().to_string();
    let body = "not json";

    let err = construct_event(body, &sign(&timestamp, body), SECRET).unwrap_err();
    assert!(matches!(err, WebhookError::InvalidPayload(_)));
}
