//! Options for tools, transfers, assignments and the system tool constructors.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::api::types::SearchStrategy;

/// Options for a tool's `options()` or a transfer system tool's `transfer()`, keyed by their
/// snake_case names (the API field names, e.g. `response_timeout_secs`, `pre_tool_speech`,
/// `transfer_type`). Values are anything serializable: plain JSON, numbers, or the SDK's generated
/// enums such as [`crate::PreToolSpeechMode`].
///
/// A key that doesn't apply to the tool's kind is reported by `build()` as `invalid_option`.
/// Setting a key to `null` removes it.
///
/// | Kind | Keys |
/// |---|---|
/// | all | `name`, `description`, `response_timeout_secs`, `interruption_mode`, `pre_tool_speech`, `tool_call_sound`, `tool_call_sound_behavior`, `tool_error_handling_mode` |
/// | webhook | `dynamic_variable_placeholders`, `execution_mode`, `content_type`, `response_filter`, `follow_redirects`, `follow_redirects_allowed_domains` |
/// | client | `dynamic_variable_placeholders`, `execution_mode`, `expects_response` |
/// | phone or SIP transfer | `transfer_type`, `custom_sip_headers`, `sip_refer_play_dialtone`, `uui`, `post_dial_digits` |
/// | agent transfer | `delay_ms`, `transfer_message`, `enable_transferred_agent_first_message`, `preserve_client_tts_overrides`, `preserve_voice_settings` |
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ToolOptions {
    /// Snake_case option names mapped to wire values, in the order they were set.
    pub raw: Map<String, Value>,
}

impl ToolOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the option `key` to `value`. A value that doesn't serialize to JSON is read as `null`.
    pub fn set<V: Serialize>(mut self, key: &str, value: V) -> Self {
        let value = serde_json::to_value(value).unwrap_or(Value::Null);
        self.raw.insert(key.to_string(), value);
        self
    }
}

/// The options of one `transfer()` entry. The same type and option names as
/// [`ToolOptions`] (and the workflow builder's transfer options).
pub type TransferOptions = ToolOptions;

/// Options for a webhook tool's `assign()`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AssignOptions {
    /// Remove the value from the tool response before the LLM and the transcript see it.
    pub sanitize: Option<bool>,
    /// Store lists and objects as they are instead of as JSON strings.
    pub preserve_native_type: Option<bool>,
}

/// Options for [`SystemTool::skip_turn`](super::SystemTool::skip_turn).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkipTurnOptions {
    /// Seconds to wait before the agent checks in. -1 waits for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_timeout_secs: Option<f64>,
}

/// Options for [`SystemTool::language_detection`](super::SystemTool::language_detection).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageDetectionOptions {
    /// Only switch language during the first two user turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_at_conversation_start: Option<bool>,
}

/// Options for [`SystemTool::play_keypad_touch_tone`](super::SystemTool::play_keypad_touch_tone).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayKeypadTouchToneOptions {
    /// Send tones as RTP events instead of in-band audio. SIP trunk numbers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_out_of_band_dtmf: Option<bool>,
    /// Don't speak after playing tones, so speech doesn't confuse an IVR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_turn_after_dtmf: Option<bool>,
}

/// Options for [`SystemTool::voicemail_detection`](super::SystemTool::voicemail_detection).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoicemailDetectionOptions {
    /// Message to leave on voicemail. Without one, the call ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voicemail_message: Option<String>,
}

/// Options for [`SystemTool::knowledge_base`](super::SystemTool::knowledge_base).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeBaseToolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_strategies: Option<Vec<SearchStrategy>>,
}

/// Options for [`SystemTool::transfer_to_number`](super::SystemTool::transfer_to_number).
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferToNumberOptions {
    /// Play a message to the caller while they wait for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_client_message: Option<bool>,
}
