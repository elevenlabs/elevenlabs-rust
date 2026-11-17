//! HS256 verification of the JWT that the ElevenLabs Speech Engine API presents when it opens an
//! inbound WebSocket connection to a developer-owned Speech Engine server.
//!
//! The token arrives in the `X-ElevenLabs-Speech-Engine-Authorization` header and is an HS256 JWT
//! whose HMAC secret is the SHA-256 hash of the account's API key. Data-residency keys
//! (`..._residency_<region>`) are signed with the base key, so the suffix is stripped before
//! hashing.
//!
//! Hand-written (see `.fernignore`): the generator has no concept of the Speech Engine upstream
//! handshake. Mirrors `verify_speech_engine_jwt` / `verifySpeechEngineJwt` in the Python, TypeScript
//! and PHP SDKs so every SDK accepts the same tokens.

use base64::Engine as _;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::error::SpeechEngineError;

type HmacSha256 = Hmac<Sha256>;

const ISSUER: &str = "https://api.elevenlabs.io/convai/speech-engine";
const SUBJECT: &str = "convai_speech_engine_upstream";

/// Clock skew, in seconds, allowed on the `exp` and `iat` claims.
const LEEWAY_SECONDS: i64 = 60;

fn jwt_err(msg: impl Into<String>) -> SpeechEngineError {
    SpeechEngineError::Jwt(msg.into())
}

/// Verify an inbound Speech Engine connection token and return its decoded claims.
///
/// `value` is the `X-ElevenLabs-Speech-Engine-Authorization` header value, with or without a
/// `Bearer ` prefix. `api_key` is the account API key the engine was created with.
///
/// Returns the decoded JWT payload on success, or a [`SpeechEngineError::Jwt`] whose message
/// matches the other SDKs when the key is missing, the token is malformed, the signature does not
/// match, or a claim (issuer, subject, exp, iat) is invalid.
pub fn verify_speech_engine_jwt(value: &str, api_key: &str) -> Result<Value, SpeechEngineError> {
    // Normalize (and reject) the key up front: the HMAC secret is SHA-256 of the
    // residency-suffix-stripped key, so a whitespace-only key — or one whose residency suffix
    // leaves an empty base — would otherwise hash to SHA-256("") , a publicly known secret that
    // would let anyone forge an accepted token.
    let normalized_key = normalize_api_key_for_signing(api_key);
    if normalized_key.is_empty() {
        return Err(jwt_err(
            "No API key configured for Speech Engine verification",
        ));
    }

    let mut token = value.trim();
    if let Some(prefix) = token.get(..7) {
        if prefix.eq_ignore_ascii_case("bearer ") {
            token = token[7..].trim();
        }
    }

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(jwt_err("Invalid JWT: expected 3 parts"));
    }
    let (header_b64, payload_b64, signature_b64) = (parts[0], parts[1], parts[2]);

    let payload_bytes = base64_url_decode(payload_b64)
        .map_err(|_| jwt_err("Invalid JWT: failed to decode payload"))?;
    let payload: Value = serde_json::from_slice(&payload_bytes)
        .map_err(|_| jwt_err("Invalid JWT: failed to decode payload"))?;
    if !payload.is_object() {
        return Err(jwt_err("Invalid JWT: payload is not an object"));
    }

    // HMAC secret is the SHA-256 of the residency-suffix-stripped API key (normalized above).
    let secret = {
        let mut hasher = Sha256::new();
        hasher.update(normalized_key.as_bytes());
        hasher.finalize()
    };
    let signing_input = format!("{header_b64}.{payload_b64}");
    let mut mac =
        HmacSha256::new_from_slice(&secret).expect("HMAC-SHA256 accepts a key of any length");
    mac.update(signing_input.as_bytes());

    let actual = base64_url_decode(signature_b64)
        .map_err(|_| signature_mismatch())?;
    // Constant-time comparison via the MAC itself.
    if mac.verify_slice(&actual).is_err() {
        return Err(signature_mismatch());
    }

    if payload.get("iss").and_then(Value::as_str) != Some(ISSUER) {
        return Err(jwt_err("Invalid JWT: unexpected issuer"));
    }
    if payload.get("sub").and_then(Value::as_str) != Some(SUBJECT) {
        return Err(jwt_err("Invalid JWT: unexpected subject"));
    }

    let exp = payload
        .get("exp")
        .and_then(Value::as_i64)
        .ok_or_else(|| jwt_err("Invalid JWT: missing exp claim"))?;
    let iat = payload
        .get("iat")
        .and_then(Value::as_i64)
        .ok_or_else(|| jwt_err("Invalid JWT: missing iat claim"))?;

    let now = current_unix_time();
    if exp + LEEWAY_SECONDS < now {
        return Err(jwt_err("Invalid JWT: token has expired"));
    }
    if iat - LEEWAY_SECONDS > now {
        return Err(jwt_err("Invalid JWT: iat is in the future"));
    }

    Ok(payload)
}

fn signature_mismatch() -> SpeechEngineError {
    jwt_err(
        "Invalid JWT: signature mismatch — make sure the API key used by your Speech Engine \
         server matches the one used to create the engine.",
    )
}

/// Strip a trailing `_residency_<region>` suffix (lowercase alphanumeric) so a data-residency key
/// hashes to the same secret as the base key it was derived from.
fn normalize_api_key_for_signing(api_key: &str) -> String {
    let trimmed = api_key.trim();
    const MARKER: &str = "_residency_";
    if let Some(idx) = trimmed.rfind(MARKER) {
        let suffix = &trimmed[idx + MARKER.len()..];
        if !suffix.is_empty()
            && suffix
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        {
            return trimmed[..idx].to_string();
        }
    }
    trimmed.to_string()
}

/// Decode a base64url string (RFC 7515), tolerating missing or present padding.
fn base64_url_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let trimmed = input.trim_end_matches('=');
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(trimmed)
}

fn current_unix_time() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Shared cross-SDK golden vector. This exact (api_key, token) pair must verify in every SDK.
    const GOLDEN_API_KEY: &str = "sk_0123456789abcdef0123456789abcdef";
    const GOLDEN_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL2FwaS5lbGV2ZW5sYWJzLmlvL2NvbnZhaS9zcGVlY2gtZW5naW5lIiwic3ViIjoiY29udmFpX3NwZWVjaF9lbmdpbmVfdXBzdHJlYW0iLCJpYXQiOjEyMzQ1Njc4OTAsImV4cCI6NDEwMjQ0NDgwMH0.UObM4hx1W8NvGViU3aqPv1-IEXhmXgWDgoS9RZZWILk";

    /// Build a signed HS256 token the same way the API does, for exercising claim checks.
    fn sign_token(api_key: &str, payload: &Value) -> String {
        let b64 = |b: &[u8]| base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b);
        let header_b64 = b64(br#"{"alg":"HS256","typ":"JWT"}"#);
        let payload_b64 = b64(&serde_json::to_vec(payload).unwrap());
        let secret = {
            let mut hasher = Sha256::new();
            hasher.update(normalize_api_key_for_signing(api_key).as_bytes());
            hasher.finalize()
        };
        let mut mac = HmacSha256::new_from_slice(&secret).unwrap();
        mac.update(format!("{header_b64}.{payload_b64}").as_bytes());
        let sig = mac.finalize().into_bytes();
        format!("{header_b64}.{payload_b64}.{}", b64(&sig))
    }

    fn valid_payload() -> Value {
        let now = current_unix_time();
        json!({
            "iss": ISSUER,
            "sub": SUBJECT,
            "iat": now - 10,
            "exp": now + 3600,
        })
    }

    #[test]
    fn golden_vector_verifies() {
        let claims = verify_speech_engine_jwt(GOLDEN_TOKEN, GOLDEN_API_KEY)
            .expect("golden vector must verify byte-for-byte");
        assert_eq!(claims["iss"], ISSUER);
        assert_eq!(claims["sub"], SUBJECT);
        assert_eq!(claims["iat"], 1234567890i64);
        assert_eq!(claims["exp"], 4102444800i64);
    }

    #[test]
    fn residency_suffix_is_stripped_before_hashing() {
        // The golden token is signed with the base key; a residency variant must still verify.
        verify_speech_engine_jwt(
            GOLDEN_TOKEN,
            "sk_0123456789abcdef0123456789abcdef_residency_us",
        )
        .expect("residency key must hash to the base-key secret");
    }

    #[test]
    fn bearer_prefix_is_tolerated() {
        let token = format!("Bearer {GOLDEN_TOKEN}");
        verify_speech_engine_jwt(&token, GOLDEN_API_KEY).expect("bearer prefix must be stripped");
    }

    #[test]
    fn missing_api_key_is_rejected() {
        let err = verify_speech_engine_jwt(GOLDEN_TOKEN, "").unwrap_err();
        assert!(err.to_string().contains("No API key configured"));
    }

    #[test]
    fn whitespace_only_api_key_is_rejected() {
        // Must be rejected before deriving the secret: a key that normalizes to "" would hash to
        // SHA-256("") — a publicly known secret that would accept forged tokens.
        let err = verify_speech_engine_jwt(GOLDEN_TOKEN, "   ").unwrap_err();
        assert!(err.to_string().contains("No API key configured"));
    }

    #[test]
    fn api_key_that_is_only_a_residency_suffix_is_rejected() {
        // The residency suffix is stripped for signing, leaving an empty base key.
        let err = verify_speech_engine_jwt(GOLDEN_TOKEN, "_residency_us").unwrap_err();
        assert!(err.to_string().contains("No API key configured"));
    }

    #[test]
    fn wrong_api_key_is_a_signature_mismatch() {
        let err = verify_speech_engine_jwt(GOLDEN_TOKEN, "sk_wrongwrongwrongwrong").unwrap_err();
        assert!(err.to_string().contains("signature mismatch"));
    }

    #[test]
    fn tampered_payload_is_rejected() {
        // Flip a character in the payload segment; the signature no longer matches.
        let mut parts: Vec<String> = GOLDEN_TOKEN.split('.').map(str::to_string).collect();
        parts[1].push('x');
        let tampered = parts.join(".");
        assert!(verify_speech_engine_jwt(&tampered, GOLDEN_API_KEY).is_err());
    }

    #[test]
    fn wrong_number_of_parts_is_rejected() {
        let err = verify_speech_engine_jwt("a.b", GOLDEN_API_KEY).unwrap_err();
        assert!(err.to_string().contains("expected 3 parts"));
    }

    #[test]
    fn expired_token_is_rejected() {
        let now = current_unix_time();
        let token = sign_token(
            GOLDEN_API_KEY,
            &json!({ "iss": ISSUER, "sub": SUBJECT, "iat": now - 7200, "exp": now - 3600 }),
        );
        let err = verify_speech_engine_jwt(&token, GOLDEN_API_KEY).unwrap_err();
        assert!(err.to_string().contains("has expired"));
    }

    #[test]
    fn future_iat_is_rejected() {
        let now = current_unix_time();
        let token = sign_token(
            GOLDEN_API_KEY,
            &json!({ "iss": ISSUER, "sub": SUBJECT, "iat": now + 3600, "exp": now + 7200 }),
        );
        let err = verify_speech_engine_jwt(&token, GOLDEN_API_KEY).unwrap_err();
        assert!(err.to_string().contains("iat is in the future"));
    }

    #[test]
    fn unexpected_issuer_is_rejected() {
        let mut payload = valid_payload();
        payload["iss"] = json!("https://evil.example.com");
        let token = sign_token(GOLDEN_API_KEY, &payload);
        let err = verify_speech_engine_jwt(&token, GOLDEN_API_KEY).unwrap_err();
        assert!(err.to_string().contains("unexpected issuer"));
    }

    #[test]
    fn unexpected_subject_is_rejected() {
        let mut payload = valid_payload();
        payload["sub"] = json!("something_else");
        let token = sign_token(GOLDEN_API_KEY, &payload);
        let err = verify_speech_engine_jwt(&token, GOLDEN_API_KEY).unwrap_err();
        assert!(err.to_string().contains("unexpected subject"));
    }

    #[test]
    fn leeway_allows_a_just_expired_token() {
        let now = current_unix_time();
        // exp 30s in the past is within the 60s leeway.
        let token = sign_token(
            GOLDEN_API_KEY,
            &json!({ "iss": ISSUER, "sub": SUBJECT, "iat": now - 120, "exp": now - 30 }),
        );
        verify_speech_engine_jwt(&token, GOLDEN_API_KEY)
            .expect("a token expired within the leeway window must still verify");
    }
}
