//! Cross-SDK golden-vector test for Speech Engine JWT verification, exercised through the crate's
//! public API. The (api_key, token) pair here must verify identically in every ElevenLabs SDK.

#![cfg(feature = "websocket")]

use elevenlabs_rust_sdk::speech_engine::verify_speech_engine_jwt;

const API_KEY: &str = "sk_0123456789abcdef0123456789abcdef";
const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL2FwaS5lbGV2ZW5sYWJzLmlvL2NvbnZhaS9zcGVlY2gtZW5naW5lIiwic3ViIjoiY29udmFpX3NwZWVjaF9lbmdpbmVfdXBzdHJlYW0iLCJpYXQiOjEyMzQ1Njc4OTAsImV4cCI6NDEwMjQ0NDgwMH0.UObM4hx1W8NvGViU3aqPv1-IEXhmXgWDgoS9RZZWILk";

#[test]
fn golden_vector_verifies_through_public_api() {
    let claims = verify_speech_engine_jwt(TOKEN, API_KEY).expect("golden vector must verify");
    assert_eq!(claims["iss"], "https://api.elevenlabs.io/convai/speech-engine");
    assert_eq!(claims["sub"], "convai_speech_engine_upstream");
}

#[test]
fn golden_vector_rejects_a_wrong_key() {
    assert!(verify_speech_engine_jwt(TOKEN, "sk_not_the_right_key").is_err());
}
