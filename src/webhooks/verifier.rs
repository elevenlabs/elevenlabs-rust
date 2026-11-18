use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

use super::WebhookError;

type HmacSha256 = Hmac<Sha256>;

/// How far in the past, in seconds, a webhook timestamp may be before it is rejected as a
/// possible replay. Timestamps in the future are accepted, as in every other ElevenLabs SDK.
pub const TOLERANCE_SECONDS: i64 = 30 * 60;

/// Verify a webhook signature and return the decoded event payload.
///
/// * `raw_body` — the request body exactly as received. Pass the unparsed bytes: re-encoding a
///   decoded body changes them and breaks verification.
/// * `sig_header` — the value of the `ElevenLabs-Signature` header.
/// * `secret` — your webhook signing secret.
///
/// The signature is compared in constant time. On success the body is parsed as JSON and
/// returned as a [`serde_json::Value`]; deserialize it into your own type if you need one.
///
/// # Errors
///
/// Returns a [`WebhookError`] if the header or secret is missing, the header has no `t=`/`v0=`
/// pair, the timestamp is more than [`TOLERANCE_SECONDS`] old, the signature does not match, or
/// the body is not valid JSON.
pub fn construct_event(
    raw_body: impl AsRef<[u8]>,
    sig_header: &str,
    secret: &str,
) -> Result<Value, WebhookError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    construct_event_at(raw_body.as_ref(), sig_header, secret, now)
}

/// [`construct_event`] with the current time injected, so the tolerance boundary can be tested.
pub(crate) fn construct_event_at(
    raw_body: &[u8],
    sig_header: &str,
    secret: &str,
    now: i64,
) -> Result<Value, WebhookError> {
    if sig_header.is_empty() {
        return Err(WebhookError::MissingSignatureHeader);
    }
    if secret.is_empty() {
        return Err(WebhookError::MissingSecret);
    }

    let mut timestamp = None;
    let mut signature = None;
    for part in sig_header.split(',') {
        if let Some(t) = part.strip_prefix("t=") {
            timestamp = Some(t);
        } else if part.starts_with("v0=") {
            signature = Some(part);
        }
    }
    let (timestamp, signature) = match (timestamp, signature) {
        (Some(t), Some(s)) if !t.is_empty() => (t, s),
        _ => return Err(WebhookError::MissingSignatureScheme),
    };

    // A non-numeric timestamp is treated as 0 (and so rejected as stale), matching PHP and Ruby.
    let request_timestamp = timestamp.parse::<i64>().unwrap_or(0);
    if request_timestamp < now - TOLERANCE_SECONDS {
        return Err(WebhookError::TimestampOutsideTolerance);
    }

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC-SHA256 accepts a key of any length");
    mac.update(timestamp.as_bytes());
    mac.update(b".");
    mac.update(raw_body);
    let expected = format!("v0={}", to_hex(&mac.finalize().into_bytes()));

    if !constant_time_eq(expected.as_bytes(), signature.as_bytes()) {
        return Err(WebhookError::SignatureMismatch);
    }

    serde_json::from_slice(raw_body).map_err(WebhookError::InvalidPayload)
}

fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut out, b| {
            let _ = write!(out, "{b:02x}");
            out
        })
}

/// Compare two byte strings without short-circuiting on the first difference. The length check
/// leaks only the length, which is fixed (`v0=` plus 64 hex characters) for a valid signature.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "wsec_0123456789abcdef0123456789abcdef";

    fn sign(timestamp: &str, body: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        mac.update(format!("{timestamp}.{body}").as_bytes());
        format!("t={timestamp},v0={}", to_hex(&mac.finalize().into_bytes()))
    }

    #[test]
    fn accepts_a_timestamp_exactly_at_the_tolerance_boundary() {
        let now = 1_700_000_000;
        let ts = (now - TOLERANCE_SECONDS).to_string();
        let body = r#"{"type":"ping"}"#;
        assert!(construct_event_at(body.as_bytes(), &sign(&ts, body), SECRET, now).is_ok());
    }

    #[test]
    fn rejects_a_timestamp_one_second_past_the_boundary() {
        let now = 1_700_000_000;
        let ts = (now - TOLERANCE_SECONDS - 1).to_string();
        let body = r#"{"type":"ping"}"#;
        assert!(matches!(
            construct_event_at(body.as_bytes(), &sign(&ts, body), SECRET, now),
            Err(WebhookError::TimestampOutsideTolerance)
        ));
    }

    #[test]
    fn rejects_a_non_numeric_timestamp_as_stale() {
        let body = r#"{"type":"ping"}"#;
        assert!(matches!(
            construct_event_at(body.as_bytes(), &sign("abc", body), SECRET, 1_700_000_000),
            Err(WebhookError::TimestampOutsideTolerance)
        ));
    }

    #[test]
    fn constant_time_eq_compares_length_and_content() {
        assert!(constant_time_eq(b"v0=abc", b"v0=abc"));
        assert!(!constant_time_eq(b"v0=abc", b"v0=abd"));
        assert!(!constant_time_eq(b"v0=abc", b"v0=ab"));
    }
}
