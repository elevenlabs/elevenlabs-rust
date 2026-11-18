use thiserror::Error;

/// Reasons a webhook can fail verification in [`construct_event`](super::construct_event).
///
/// Hand-written (see `.fernignore`): webhook verification happens locally and never talks to the
/// API, so these live outside [`crate::error::ApiError`]. The messages match the Python, TypeScript,
/// PHP and Ruby SDKs so every SDK reports the same reason for the same input.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WebhookError {
    /// The `ElevenLabs-Signature` header was empty or not present.
    #[error("Missing signature header")]
    MissingSignatureHeader,

    /// The webhook signing secret was empty.
    #[error("Webhook secret not configured")]
    MissingSecret,

    /// The header did not contain both a `t=` timestamp and a `v0=` signature.
    #[error("No signature hash found with expected scheme v0")]
    MissingSignatureScheme,

    /// The timestamp is more than 30 minutes in the past (or is not a number), so the request
    /// is rejected as a possible replay.
    #[error("Timestamp outside the tolerance zone")]
    TimestampOutsideTolerance,

    /// The `v0` signature does not match the HMAC of the timestamp and raw body.
    #[error("Signature hash does not match the expected signature hash for payload")]
    SignatureMismatch,

    /// The signature was valid but the body is not valid JSON.
    #[error("Webhook payload is not valid JSON: {0}")]
    InvalidPayload(#[source] serde_json::Error),
}
