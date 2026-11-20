//! `SystemTool`: a built-in tool the agent runs itself, such as ending or transferring the call.

use serde::Serialize;
use serde_json::{json, Value};

use crate::agent_workflows::wire::canonical;
use crate::agent_workflows::Destination;
use crate::api::types::{SystemToolConfigInput, SystemToolConfigInputParams};

use super::errors::{ToolIssue, ToolIssueCode};
use super::options::{
    KnowledgeBaseToolOptions, LanguageDetectionOptions, PlayKeypadTouchToneOptions,
    SkipTurnOptions, ToolOptions, TransferOptions, TransferToNumberOptions,
    VoicemailDetectionOptions,
};
use super::schema::WireMap;
use super::tool::{accepted_options, set_config_option, tool_methods};
use super::webhook::COMMON_OPTIONS;
use super::ToolKind;

const PHONE_TRANSFER_OPTIONS: &[&str] = &[
    "transfer_type",
    "custom_sip_headers",
    "sip_refer_play_dialtone",
    "uui",
    "post_dial_digits",
];

/// The `system_tool_type`s the generated params enum has a variant for, as in
/// `src/api/types/system_tool_config_input_params.rs` (a test keeps the two in step). Params of
/// one of these types that don't match its variant would fall back to the enum's catch-all and
/// be sent unchecked, so `build()` reports them.
pub(crate) const MODELED_SYSTEM_TOOL_TYPES: &[&str] = &[
    "end_call",
    "end_procedure",
    "knowledge_base",
    "knowledge_base_rag",
    "language_detection",
    "play_keypad_touch_tone",
    "skip_turn",
    "start_procedure",
    "transfer_to_agent",
    "transfer_to_number",
    "voicemail_detection",
];

const AGENT_TRANSFER_OPTIONS: &[&str] = &[
    "delay_ms",
    "transfer_message",
    "enable_transferred_agent_first_message",
    "preserve_client_tts_overrides",
    "preserve_voice_settings",
];

/// A built-in tool the agent runs itself, such as ending the call or transferring it. Its `name`
/// defaults to its `system_tool_type`, and its description is left out so the API uses its own;
/// both can be set with `options()`.
///
/// ```
/// use elevenlabs_rust_sdk::agent_tools::{Destination, SystemTool, ToolOptions};
///
/// let transfer = SystemTool::transfer_to_number(Default::default())
///     .transfer("Caller asks for billing", Destination::phone("+15551234567"), Default::default())
///     .transfer(
///         "Caller asks for a manager",
///         Destination::sip_variable("manager_sip"),
///         ToolOptions::new().set("transfer_type", "sip_refer"),
///     );
/// // An entry for the agent's `prompt.built_in_tools`.
/// let entry = transfer.build_config().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SystemTool {
    /// The tool config fields other than `params`.
    config: WireMap,
    /// Problems found while building, reported by `build()`.
    issues: Vec<ToolIssue>,
    system_tool_type: String,
    /// The `params` fields other than `system_tool_type`.
    params: WireMap,
    /// Whether the loaded `params` were missing, not an object or without a `system_tool_type`.
    /// They are then kept in `config` as loaded, and `build()` reports them.
    malformed_params: bool,
}

impl SystemTool {
    pub(crate) fn load(system_tool_type: String, config: WireMap, params: WireMap) -> Self {
        SystemTool {
            config,
            issues: Vec::new(),
            system_tool_type,
            params,
            malformed_params: false,
        }
    }

    /// A loaded system tool whose `params` (kept in `config`, if any) aren't an object with a
    /// `system_tool_type`.
    pub(crate) fn load_malformed(config: WireMap) -> Self {
        SystemTool {
            malformed_params: true,
            ..SystemTool::load(String::new(), config, WireMap::new())
        }
    }

    fn create(system_tool_type: &str, options: impl Serialize) -> Self {
        let mut config = WireMap::new();
        config.insert("name".into(), json!(system_tool_type));
        let params = match serde_json::to_value(options) {
            Ok(Value::Object(params)) => params,
            _ => WireMap::new(),
        };
        SystemTool::load(system_tool_type.to_string(), config, params)
    }

    /// Lets the agent end the call.
    pub fn end_call() -> Self {
        SystemTool::create("end_call", json!({}))
    }

    /// Lets the agent stay quiet when the user asks for a moment.
    pub fn skip_turn(options: SkipTurnOptions) -> Self {
        SystemTool::create("skip_turn", options)
    }

    /// Lets the agent switch language.
    pub fn language_detection(options: LanguageDetectionOptions) -> Self {
        SystemTool::create("language_detection", options)
    }

    /// Lets the agent play keypad tones, for phone menus.
    pub fn play_keypad_touch_tone(options: PlayKeypadTouchToneOptions) -> Self {
        SystemTool::create("play_keypad_touch_tone", options)
    }

    /// Lets the agent detect voicemail and leave a message or hang up.
    pub fn voicemail_detection(options: VoicemailDetectionOptions) -> Self {
        SystemTool::create("voicemail_detection", options)
    }

    /// Lets the agent search its knowledge base.
    pub fn knowledge_base(options: KnowledgeBaseToolOptions) -> Self {
        SystemTool::create("knowledge_base", options)
    }

    /// Lets the agent transfer the call to a phone number or SIP URI. Add destinations with
    /// [`transfer`](Self::transfer).
    pub fn transfer_to_number(options: TransferToNumberOptions) -> Self {
        let mut tool = SystemTool::create("transfer_to_number", options);
        tool.params.insert("transfers".into(), json!([]));
        tool
    }

    /// Lets the agent hand the conversation to another agent. Add destinations with
    /// [`transfer`](Self::transfer).
    pub fn transfer_to_agent() -> Self {
        SystemTool::create("transfer_to_agent", json!({ "transfers": [] }))
    }

    tool_methods!(ToolKind::System, System, SystemToolConfigInput);

    /// The API's `system_tool_type`, e.g. `"end_call"`.
    pub fn system_tool_type(&self) -> &str {
        &self.system_tool_type
    }

    /// Adds a destination to a transfer tool. `condition` is plain text telling the LLM when to
    /// transfer there. `transfer_to_number` takes phone and SIP destinations (options
    /// `transfer_type`, `custom_sip_headers`, `sip_refer_play_dialtone`, `uui`,
    /// `post_dial_digits`); a SIP URI needs `transfer_type: "sip_refer"`. `transfer_to_agent`
    /// takes agent destinations (options `delay_ms`, `transfer_message`,
    /// `enable_transferred_agent_first_message`, `preserve_client_tts_overrides`,
    /// `preserve_voice_settings`).
    pub fn transfer(
        mut self,
        condition: impl Into<String>,
        destination: Destination,
        options: TransferOptions,
    ) -> Self {
        let condition = condition.into();
        let to_number = self.system_tool_type == "transfer_to_number";
        if !to_number && self.system_tool_type != "transfer_to_agent" {
            self.issues.push(ToolIssue::error(
                ToolIssueCode::InvalidOption,
                Some("transfer"),
                format!(
                    "transfer() only applies to transfer tools, not {}",
                    self.system_tool_type
                ),
            ));
            return self;
        }
        let mut target = match destination.to_wire() {
            Value::Object(model) => model,
            _ => WireMap::new(),
        };
        let is_phone = target
            .remove("type")
            .and_then(|t| t.as_str().map(str::to_string))
            == Some("phone_number".to_string());
        if is_phone != to_number {
            let message = if to_number {
                format!("transfer_to_number() needs a phone or SIP destination, not an agent (\"{condition}\")")
            } else {
                format!("transfer_to_agent() needs an agent destination (\"{condition}\")")
            };
            self.issues.push(ToolIssue::error(
                ToolIssueCode::InvalidDestination,
                Some("transfer"),
                message,
            ));
            return self;
        }
        let allowed = if to_number {
            PHONE_TRANSFER_OPTIONS
        } else {
            AGENT_TRANSFER_OPTIONS
        };
        let kind = self.system_tool_type.clone();
        let settings = accepted_options(options, allowed, &kind, &mut self.issues);
        // A SIP URI held in a dynamic variable resolves to a SIP URI too, so it also needs SIP REFER.
        let destination_type = target
            .get("transfer_destination")
            .and_then(|d| d.get("type"))
            .and_then(Value::as_str);
        let sip_uri = matches!(
            destination_type,
            Some("sip_uri" | "sip_uri_dynamic_variable")
        );
        let sip_refer = settings
            .iter()
            .any(|(key, value)| key == "transfer_type" && value.as_str() == Some("sip_refer"));
        if sip_uri && !sip_refer {
            self.issues.push(ToolIssue::error(
                ToolIssueCode::InvalidDestination,
                Some("transfer"),
                format!(
                    "A SIP URI destination needs transfer_type: \"sip_refer\" (\"{condition}\")"
                ),
            ));
            return self;
        }
        for (key, value) in settings {
            if !value.is_null() {
                target.insert(key, value);
            }
        }
        target.insert("condition".into(), json!(condition));
        let mut transfers = match self.params.remove("transfers") {
            Some(Value::Array(transfers)) => transfers,
            _ => Vec::new(),
        };
        transfers.push(Value::Object(target));
        self.params
            .insert("transfers".into(), Value::Array(transfers));
        self
    }

    /// Merges options for system tools (see [`ToolOptions`]; only the options every tool kind
    /// has). A key that doesn't apply is reported by `build()`.
    pub fn options(mut self, options: ToolOptions) -> Self {
        for (key, value) in accepted_options(options, COMMON_OPTIONS, "system", &mut self.issues) {
            set_config_option(&mut self.config, &key, value);
        }
        self
    }

    fn validate(&self) -> Vec<ToolIssue> {
        let is_transfer = matches!(
            self.system_tool_type.as_str(),
            "transfer_to_number" | "transfer_to_agent"
        );
        if !is_transfer {
            return Vec::new();
        }
        // A loaded transfer_to_number tool may use transfer_rules instead of transfers; the API takes exactly one.
        let has_entries = |key: &str| {
            self.params
                .get(key)
                .and_then(Value::as_array)
                .is_some_and(|entries| !entries.is_empty())
        };
        let has_transfers = has_entries("transfers");
        let has_rules = has_entries("transfer_rules");
        if !has_transfers && !has_rules {
            return vec![ToolIssue::error(
                ToolIssueCode::NoTransfers,
                Some("transfer"),
                format!("{} needs at least one transfer()", self.system_tool_type),
            )];
        }
        if has_transfers && has_rules {
            return vec![ToolIssue::error(
                ToolIssueCode::ConflictingTransfers,
                Some("transfer"),
                format!(
                    "{} has both transfers and transfer_rules; the API takes only one",
                    self.system_tool_type
                ),
            )];
        }
        Vec::new()
    }

    fn model_issues(&self) -> Vec<ToolIssue> {
        if self.malformed_params {
            return vec![ToolIssue::error(
                ToolIssueCode::InvalidModel,
                Some("params"),
                "The loaded params aren't an object with a system_tool_type, so the tool doesn't match the generated tool config model. Fix it in the source JSON",
            )];
        }
        // A type the generated enum doesn't model goes to its catch-all as loaded, which is fine;
        // a modeled one must match its own variant.
        if !MODELED_SYSTEM_TOOL_TYPES.contains(&self.system_tool_type.as_str()) {
            return Vec::new();
        }
        let mut params = self.params.clone();
        params.insert("system_tool_type".into(), json!(self.system_tool_type));
        let parsed: Result<SystemToolConfigInputParams, _> =
            serde_json::from_value(canonical(&Value::Object(params)));
        if matches!(
            parsed,
            Ok(SystemToolConfigInputParams::__Unknown(_)) | Err(_)
        ) {
            return vec![ToolIssue::error(
                ToolIssueCode::InvalidModel,
                Some("params"),
                format!(
                    "The params don't match the generated model for {}: a field likely has the wrong shape or type. Fix the options or the source JSON",
                    self.system_tool_type
                ),
            )];
        }
        Vec::new()
    }

    fn to_wire(&self) -> WireMap {
        let mut wire = self.config.clone();
        wire.insert("type".into(), json!("system"));
        if !self.malformed_params {
            let mut params = self.params.clone();
            params.insert("system_tool_type".into(), json!(self.system_tool_type));
            wire.insert("params".into(), Value::Object(params));
        }
        wire
    }
}
