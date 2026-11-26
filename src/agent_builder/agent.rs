//! `Agent`: an agent's settings, its tools and its workflow in one definition.

use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};
use std::fmt;

use serde_json::{json, Map, Value};

use crate::agent_tools::json::js_canonical;
use crate::agent_tools::{Tool, ToolKind};
use crate::agent_workflows::Workflow;
use crate::api::types::{
    AgentConfig, Llm, AgentPlatformSettingsRequestModel,
    BodyCreateAgentV1ConvaiAgentsCreatePost, ConversationalConfig, PromptAgentApiModelInput,
    TtsConversationalConfigInput, TtsConversationalModel,
};

use super::errors::{AgentIssue, AgentIssueCode, AgentValidationError, Severity};
use super::wire::{
    deep_merge, get_path, get_path_in, map_raw_workflow_tool_ids, raw_workflow_tool_ids, set_path,
    WireMap, BUILT_IN_TOOLS_PATH, TOOL_IDS_PATH,
};

/// The settings an agent can be given in code. Every field is optional, so
/// [`Agent::update`] takes the same shape. Set fields directly or with the chainable setters:
/// `AgentOptions::new().prompt("...").first_message("Hi!")`.
#[derive(Default)]
pub struct AgentOptions {
    /// Renames the agent (for `update()`; `Agent::new` takes the name itself).
    pub name: Option<String>,
    /// The agent's instructions: `conversation_config.agent.prompt.prompt`.
    pub prompt: Option<String>,
    /// What the agent says first: `conversation_config.agent.first_message`.
    pub first_message: Option<String>,
    /// `conversation_config.agent.language`, e.g. `"en"`.
    pub language: Option<String>,
    /// `conversation_config.agent.prompt.llm`.
    pub llm: Option<Llm>,
    /// `conversation_config.agent.prompt.temperature`.
    pub temperature: Option<f64>,
    /// `conversation_config.tts.voice_id`.
    pub voice_id: Option<String>,
    /// `conversation_config.tts.model_id`.
    pub tts_model: Option<TtsConversationalModel>,
    /// `tags`.
    pub tags: Option<Vec<String>>,
    /// Tools the agent can use anywhere. Webhook and client tools are created or updated by
    /// `push` and attached by id; system tools go into `prompt.built_in_tools`. Replaces the
    /// agent's tools on `update()`.
    pub tools: Option<Vec<Tool>>,
    /// Ids of existing tools to attach as they are. `push` doesn't create or change them.
    pub tool_ids: Option<Vec<String>>,
    /// The agent's workflow. Its tool nodes can take [`Tool`] objects, which `push` manages too.
    pub workflow: Option<Workflow>,
    /// Any other conversation settings, merged under the typed options above. Setting a field
    /// here that a typed option also sets is reported by `validate()`.
    pub conversation_config: Option<ConversationalConfig>,
    /// Platform settings (privacy, evaluation, widget...), merged the same way.
    pub platform_settings: Option<AgentPlatformSettingsRequestModel>,
}

impl AgentOptions {
    pub fn new() -> Self {
        Self::default()
    }
    /// See [`AgentOptions::name`].
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    /// See [`AgentOptions::prompt`].
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }
    /// See [`AgentOptions::first_message`].
    pub fn first_message(mut self, first_message: impl Into<String>) -> Self {
        self.first_message = Some(first_message.into());
        self
    }
    /// See [`AgentOptions::language`].
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
    /// See [`AgentOptions::llm`].
    pub fn llm(mut self, llm: Llm) -> Self {
        self.llm = Some(llm);
        self
    }
    /// See [`AgentOptions::temperature`].
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    /// See [`AgentOptions::voice_id`].
    pub fn voice_id(mut self, voice_id: impl Into<String>) -> Self {
        self.voice_id = Some(voice_id.into());
        self
    }
    /// See [`AgentOptions::tts_model`].
    pub fn tts_model(mut self, tts_model: TtsConversationalModel) -> Self {
        self.tts_model = Some(tts_model);
        self
    }
    /// See [`AgentOptions::tags`].
    pub fn tags<S: Into<String>>(mut self, tags: impl IntoIterator<Item = S>) -> Self {
        self.tags = Some(tags.into_iter().map(Into::into).collect());
        self
    }
    /// See [`AgentOptions::tools`].
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }
    /// See [`AgentOptions::tool_ids`].
    pub fn tool_ids<S: Into<String>>(mut self, tool_ids: impl IntoIterator<Item = S>) -> Self {
        self.tool_ids = Some(tool_ids.into_iter().map(Into::into).collect());
        self
    }
    /// See [`AgentOptions::workflow`].
    pub fn workflow(mut self, workflow: Workflow) -> Self {
        self.workflow = Some(workflow);
        self
    }
    /// See [`AgentOptions::conversation_config`].
    pub fn conversation_config(mut self, config: ConversationalConfig) -> Self {
        self.conversation_config = Some(config);
        self
    }
    /// See [`AgentOptions::platform_settings`].
    pub fn platform_settings(mut self, settings: AgentPlatformSettingsRequestModel) -> Self {
        self.platform_settings = Some(settings);
        self
    }
}

/// A typed option's value. Each one is the piece of the generated `ConversationalConfig` it
/// sets, so a renamed field fails to compile; serde turns it into API format.
#[derive(Clone)]
enum Typed {
    Prompt(String),
    FirstMessage(String),
    Language(String),
    Llm(Llm),
    Temperature(f64),
    VoiceId(String),
    TtsModel(TtsConversationalModel),
}

impl Typed {
    fn key(&self) -> &'static str {
        match self {
            Typed::Prompt(_) => "prompt",
            Typed::FirstMessage(_) => "first_message",
            Typed::Language(_) => "language",
            Typed::Llm(_) => "llm",
            Typed::Temperature(_) => "temperature",
            Typed::VoiceId(_) => "voice_id",
            Typed::TtsModel(_) => "tts_model",
        }
    }

    fn piece(&self) -> ConversationalConfig {
        let prompt = |prompt: PromptAgentApiModelInput| ConversationalConfig {
            agent: Some(AgentConfig {
                prompt: Some(prompt),
                ..Default::default()
            }),
            ..Default::default()
        };
        let agent = |agent: AgentConfig| ConversationalConfig {
            agent: Some(agent),
            ..Default::default()
        };
        let tts = |tts: TtsConversationalConfigInput| ConversationalConfig {
            tts: Some(tts),
            ..Default::default()
        };
        match self.clone() {
            Typed::Prompt(value) => prompt(PromptAgentApiModelInput {
                prompt: Some(value),
                ..Default::default()
            }),
            Typed::Llm(value) => prompt(PromptAgentApiModelInput {
                llm: Some(value),
                ..Default::default()
            }),
            Typed::Temperature(value) => prompt(PromptAgentApiModelInput {
                temperature: Some(value),
                ..Default::default()
            }),
            Typed::FirstMessage(value) => agent(AgentConfig {
                first_message: Some(value),
                ..Default::default()
            }),
            Typed::Language(value) => agent(AgentConfig {
                language: Some(value),
                ..Default::default()
            }),
            Typed::VoiceId(value) => tts(TtsConversationalConfigInput {
                voice_id: Some(value),
                ..Default::default()
            }),
            Typed::TtsModel(value) => tts(TtsConversationalConfigInput {
                model_id: Some(value),
                ..Default::default()
            }),
        }
    }

    /// The API-format path this option sets, and its value in API format.
    fn wire(&self) -> (Vec<String>, Value) {
        let mut wire = serde_json::to_value(self.piece()).unwrap_or(Value::Null);
        let mut path = vec!["conversation_config".to_string()];
        loop {
            let next = match &wire {
                Value::Object(map) if map.len() == 1 => map
                    .iter()
                    .next()
                    .map(|(key, value)| (key.clone(), value.clone())),
                _ => None,
            };
            match next {
                Some((key, value)) => {
                    path.push(key);
                    wire = value;
                }
                None => return (path, wire),
            }
        }
    }
}

const PROMPT_PATH: &[&str] = &["conversation_config", "agent", "prompt", "prompt"];

/// An agent, written as code: its settings, its tools and its workflow in one definition. Push it
/// with `client.agents_as_code().push(&agent, ..)`, which creates or updates the tools and the
/// agent together.
///
/// ```no_run
/// use elevenlabs_rust_sdk::agent_builder::{Agent, AgentOptions};
/// use elevenlabs_rust_sdk::agent_tools::{Schema, SystemTool, Tool};
///
/// let get_order = Tool::webhook("get_order", "Look up an order by its number")
///     .get("https://api.example.com/orders/{order_id}")
///     .path_param("order_id", Schema::string().describe("The order number"));
/// let agent = Agent::new(
///     "Support line",
///     AgentOptions::new()
///         .prompt("You are a support agent for Acme.")
///         .first_message("Hi, how can I help?")
///         .tools(vec![SystemTool::end_call().into(), get_order.into()]),
/// );
/// agent.validate().unwrap();
/// ```
///
/// An `Agent` holds no client: it can be validated without an API key and pushed to several
/// workspaces. Like [`Workflow`], it isn't `Send`.
pub struct Agent {
    name: String,
    typed: Vec<Typed>,
    tags: Option<Vec<String>>,
    tools: Vec<Tool>,
    tool_ids: Vec<String>,
    workflow: OnceCell<Workflow>,
    /// A loaded workflow nobody has read yet, kept in API format so it round-trips exactly.
    raw_workflow: Option<Value>,
    tools_by_id: HashMap<String, Tool>,
    /// The loaded `tool_ids`, so a loaded agent keeps its tool order.
    loaded_tool_ids: Vec<String>,
    /// Whether `tools` or `tool_ids` was given (even empty), so the agent sets `tool_ids` rather
    /// than leaving it alone.
    tool_list_given: bool,
    /// Loaded system tools with their config as loaded, sent as-is while the tool is unchanged.
    loaded_system: Vec<(Tool, Value)>,
    /// Settings loaded from a config, in API format. Typed and raw options go on top.
    base: WireMap,
    raw_conversation_config: Vec<ConversationalConfig>,
    raw_platform_settings: Vec<AgentPlatformSettingsRequestModel>,
    /// The project entry this agent was loaded from or recorded in: `(project, entry index)`.
    /// Carried by clones, so a clone of `project.agent(name)` still matches its entry.
    pub(crate) origin: Option<(u64, usize)>,
}

impl Clone for Agent {
    fn clone(&self) -> Self {
        let workflow = OnceCell::new();
        if let Some(wf) = self.workflow.get() {
            let _ = workflow.set(wf.deep_clone());
        }
        Self {
            name: self.name.clone(),
            typed: self.typed.clone(),
            tags: self.tags.clone(),
            tools: self.tools.clone(),
            tool_ids: self.tool_ids.clone(),
            workflow,
            raw_workflow: self.raw_workflow.clone(),
            tools_by_id: self.tools_by_id.clone(),
            loaded_tool_ids: self.loaded_tool_ids.clone(),
            tool_list_given: self.tool_list_given,
            loaded_system: self.loaded_system.clone(),
            base: self.base.clone(),
            raw_conversation_config: self.raw_conversation_config.clone(),
            raw_platform_settings: self.raw_platform_settings.clone(),
            origin: self.origin,
        }
    }
}

impl fmt::Debug for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Agent")
            .field("name", &self.name)
            .field("tools", &self.tools)
            .field("tool_ids", &self.tool_ids)
            .finish_non_exhaustive()
    }
}

impl Agent {
    /// An agent named `name` with `options`.
    pub fn new(name: impl Into<String>, options: AgentOptions) -> Self {
        let mut agent = Agent {
            name: name.into(),
            typed: Vec::new(),
            tags: None,
            tools: Vec::new(),
            tool_ids: Vec::new(),
            workflow: OnceCell::new(),
            raw_workflow: None,
            tools_by_id: HashMap::new(),
            loaded_tool_ids: Vec::new(),
            tool_list_given: false,
            loaded_system: Vec::new(),
            base: WireMap::new(),
            raw_conversation_config: Vec::new(),
            raw_platform_settings: Vec::new(),
            origin: None,
        };
        agent.update(options);
        agent
    }

    /// Loads an agent config in API format (snake_case): a CLI `agent_configs/*.json` file, or
    /// the `to_json()` of an agent. Fields this SDK doesn't model are kept, so an unedited agent
    /// round-trips exactly. Every id in `tool_ids` stays an attached id: use
    /// [`from_json_with_tools`](Self::from_json_with_tools) to load some as [`Tool`] objects.
    pub fn from_json(config: &Value) -> Self {
        Self::from_json_with_tools(config, &HashMap::new())
    }

    /// Like [`from_json`](Self::from_json). Webhook and client tools whose ids are in
    /// `tools_by_id` become [`Tool`] objects `push` manages (in `tools()`); every other id,
    /// including tools of other kinds (such as `mcp`), stays in `tool_ids()`. In the workflow's
    /// tool nodes every `tools_by_id` match becomes a tool object; one of another kind keeps the
    /// id it was loaded under.
    pub fn from_json_with_tools(config: &Value, tools_by_id: &HashMap<String, Tool>) -> Self {
        let mut base = config.as_object().cloned().unwrap_or_default();
        let name = match base.remove("name") {
            Some(Value::String(name)) => name,
            _ => String::new(),
        };
        let mut agent = Agent::new(name, AgentOptions::default());
        agent.tags = match base.remove("tags") {
            Some(Value::Array(tags)) => Some(
                tags.iter()
                    .filter_map(|t| t.as_str().map(str::to_string))
                    .collect(),
            ),
            _ => None,
        };
        let workflow = base.remove("workflow");
        agent.tools_by_id = tools_by_id.clone();

        if let Some(Value::Array(ids)) = get_path_in(&base, TOOL_IDS_PATH).cloned() {
            set_path(&mut base, TOOL_IDS_PATH, None);
            agent.tool_list_given = true;
            for id in ids.iter().filter_map(Value::as_str) {
                agent.loaded_tool_ids.push(id.to_string());
                // Only tools push manages become tool objects. Other kinds (such as `mcp`) stay
                // attached by id, as `pull` leaves them, so replacing `tools` keeps them.
                match tools_by_id.get(id) {
                    Some(tool) if is_managed(tool) => agent.tools.push(tool.clone()),
                    _ => agent.tool_ids.push(id.to_string()),
                }
            }
        }

        // System tools become SystemTool objects, so replacing `tools` replaces them like on an
        // agent built in code.
        if let Some(Value::Object(built_in)) = get_path_in(&base, BUILT_IN_TOOLS_PATH).cloned() {
            let mut kept = built_in.clone();
            for (key, raw) in &built_in {
                let Value::Object(fields) = raw else {
                    continue;
                };
                let mut with_type = Map::new();
                with_type.insert("type".into(), json!("system"));
                with_type.extend(fields.clone());
                let tool = Tool::from_json(&Value::Object(with_type));
                if tool.as_system().map(|s| s.system_tool_type()) != Some(key.as_str()) {
                    continue;
                }
                agent.loaded_system.push((tool.clone(), raw.clone()));
                agent.tools.push(tool);
                kept.remove(key);
            }
            let value = if kept.is_empty() {
                None
            } else {
                Some(Value::Object(kept))
            };
            set_path(&mut base, BUILT_IN_TOOLS_PATH, value);
        }

        if let Some(workflow @ Value::Object(_)) = workflow {
            agent.raw_workflow = Some(workflow);
        }
        agent.base = base;
        agent
    }

    /// Loads an agent config from an API-format JSON string. See [`from_json`](Self::from_json).
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        Ok(Self::from_json(&serde_json::from_str(json)?))
    }

    /// The agent's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The agent's workflow, or `None` if it has none. A loaded workflow is turned into a
    /// [`Workflow`] the first time it's read here, with its tool ids swapped for the [`Tool`]
    /// objects the agent knows, so it can be edited (through its node handles).
    pub fn workflow(&self) -> Option<&Workflow> {
        if let Some(wf) = self.workflow.get() {
            return Some(wf);
        }
        let raw = self.raw_workflow.as_ref()?;
        Some(
            self.workflow
                .get_or_init(|| Workflow::from_json_with_tools(raw, &self.tools_by_id)),
        )
    }

    /// Changes the agent. Typed options, `tools`, `tool_ids` and `workflow` replace what was
    /// there; `conversation_config` and `platform_settings` merge.
    pub fn update(&mut self, options: AgentOptions) -> &mut Self {
        let AgentOptions {
            name,
            prompt,
            first_message,
            language,
            llm,
            temperature,
            voice_id,
            tts_model,
            tags,
            tools,
            tool_ids,
            workflow,
            conversation_config,
            platform_settings,
        } = options;
        if let Some(name) = name {
            self.name = name;
        }
        let typed = [
            prompt.map(Typed::Prompt),
            first_message.map(Typed::FirstMessage),
            language.map(Typed::Language),
            llm.map(Typed::Llm),
            temperature.map(Typed::Temperature),
            voice_id.map(Typed::VoiceId),
            tts_model.map(Typed::TtsModel),
        ];
        for value in typed.into_iter().flatten() {
            match self.typed.iter_mut().find(|t| t.key() == value.key()) {
                Some(slot) => *slot = value,
                None => self.typed.push(value),
            }
        }
        if let Some(tags) = tags {
            self.tags = Some(tags);
        }
        if let Some(tools) = tools {
            self.tools = tools;
            self.tool_list_given = true;
        }
        if let Some(tool_ids) = tool_ids {
            self.tool_ids = tool_ids;
            self.tool_list_given = true;
        }
        if let Some(workflow) = workflow {
            self.workflow = OnceCell::from(workflow);
            self.raw_workflow = None;
        }
        if let Some(config) = conversation_config {
            self.raw_conversation_config.push(config);
        }
        if let Some(settings) = platform_settings {
            self.raw_platform_settings.push(settings);
        }
        self
    }

    /// Adds a tool the agent can use anywhere.
    pub fn add_tool(&mut self, tool: impl Into<Tool>) -> &mut Self {
        self.tools.push(tool.into());
        self.tool_list_given = true;
        self
    }

    /// Every webhook and client tool the agent and its workflow use, each once, in first-use
    /// order: the tools `push` creates or updates. System tools aren't included; they live inside
    /// the agent's config. Nor are tools of other types (such as `mcp`): they stay attached by the
    /// id the agent was loaded with. Tools are values, so "once" means one of each distinct
    /// definition.
    pub fn tools(&self) -> Vec<Tool> {
        let mut out: Vec<Tool> = Vec::new();
        for tool in self.tools.iter().cloned().chain(self.workflow_tools()) {
            if is_managed(&tool) && !out.iter().any(|t| t.same_as(&tool)) {
                out.push(tool);
            }
        }
        out
    }

    /// The id a tool was loaded under (through `tools_by_id`), checking the loaded `tool_ids`
    /// first so the answer doesn't depend on map order.
    fn known_id(&self, tool: &Tool) -> Option<String> {
        let mut rest: Vec<&String> = self.tools_by_id.keys().collect();
        rest.sort();
        self.loaded_tool_ids
            .iter()
            .chain(rest)
            .find(|id| self.tools_by_id.get(*id).is_some_and(|t| t.same_as(tool)))
            .cloned()
    }

    fn workflow_tools(&self) -> Vec<Tool> {
        match self.workflow.get() {
            Some(wf) => wf.tools(),
            None => raw_workflow_tool_ids(self.raw_workflow.as_ref())
                .iter()
                .filter_map(|id| self.tools_by_id.get(id).cloned())
                .collect(),
        }
    }

    /// The ids of existing tools attached as they are, which `push` doesn't manage.
    pub fn tool_ids(&self) -> &[String] {
        &self.tool_ids
    }

    /// Checks everything that doesn't need tool ids: the tools, the workflow, the settings, and
    /// that no two tools share a name. CI can run this without an API key.
    pub fn validate(&self) -> Result<(), AgentValidationError> {
        self.assemble(None, &mut |_| {}).map(|_| ())
    }

    /// Like [`validate`](Self::validate), but `on_warning` is called once per non-fatal problem,
    /// such as an agent without a prompt.
    pub fn validate_with_warnings<F: FnMut(&AgentIssue)>(
        &self,
        mut on_warning: F,
    ) -> Result<(), AgentValidationError> {
        self.assemble(None, &mut on_warning).map(|_| ())
    }

    /// The agent config in API format (snake_case keys, sorted at every level): the shape of a
    /// CLI `agent_configs/*.json` file. `tool_ids` maps each managed tool's name to its id;
    /// every tool in [`tools`](Self::tools) needs one (`push` passes them).
    pub fn to_json(
        &self,
        tool_ids: &HashMap<String, String>,
    ) -> Result<Value, AgentValidationError> {
        self.assemble(Some(tool_ids), &mut |_| {})
    }

    /// Like [`to_json`](Self::to_json), but `on_warning` is called once per non-fatal problem.
    pub fn to_json_with_warnings<F: FnMut(&AgentIssue)>(
        &self,
        tool_ids: &HashMap<String, String>,
        mut on_warning: F,
    ) -> Result<Value, AgentValidationError> {
        self.assemble(Some(tool_ids), &mut on_warning)
    }

    /// [`to_json`](Self::to_json) parsed into the generated create-agent request, which
    /// `client.agents.create` accepts. Fields this SDK doesn't model are dropped by the generated
    /// type; `push` sends them too.
    pub fn build(
        &self,
        tool_ids: &HashMap<String, String>,
    ) -> Result<BodyCreateAgentV1ConvaiAgentsCreatePost, AgentValidationError> {
        let config = self.to_json(tool_ids)?;
        serde_json::from_value(config).map_err(|err| AgentValidationError {
            agent_name: self.name.clone(),
            issues: vec![AgentIssue::error(
                AgentIssueCode::InvalidModel,
                format!("The agent doesn't match the generated create-agent request model: {err}. A loaded field likely has the wrong shape; fix it in the source JSON"),
                None,
            )],
        })
    }

    /// Builds the API-format config. Without `tool_ids` (validation only), tools stand under
    /// placeholder ids and missing ids aren't reported.
    fn assemble(
        &self,
        tool_ids: Option<&HashMap<String, String>>,
        on_warning: &mut dyn FnMut(&AgentIssue),
    ) -> Result<Value, AgentValidationError> {
        let mut issues: Vec<AgentIssue> = Vec::new();
        let mut config = self.base.clone();
        // Webhook and client tools get the id push gives them; other kinds keep the id they were
        // loaded with (a missing one is reported below).
        let id_of = |tool: &Tool| -> String {
            if !is_managed(tool) {
                if let Some(id) = self.known_id(tool) {
                    return id;
                }
            }
            tool_ids
                .and_then(|ids| ids.get(tool.name()).cloned())
                .unwrap_or_else(|| format!("<tool:{}>", tool.name()))
        };

        if self.name.trim().is_empty() {
            issues.push(AgentIssue::error(
                AgentIssueCode::MissingName,
                "The agent needs a name",
                None,
            ));
        }

        // Raw options first, so the typed ones win and can be checked against them.
        let mut raw = WireMap::new();
        for settings in &self.raw_conversation_config {
            match serde_json::to_value(settings) {
                Ok(Value::Object(wire)) => {
                    let mut piece = WireMap::new();
                    piece.insert("conversation_config".into(), Value::Object(wire));
                    deep_merge(&mut raw, &piece);
                }
                Ok(_) => {}
                Err(err) => issues.push(invalid_config("conversation_config", &err)),
            }
        }
        for settings in &self.raw_platform_settings {
            match serde_json::to_value(settings) {
                Ok(Value::Object(wire)) => {
                    let mut piece = WireMap::new();
                    piece.insert("platform_settings".into(), Value::Object(wire));
                    deep_merge(&mut raw, &piece);
                }
                Ok(_) => {}
                Err(err) => issues.push(invalid_config("platform_settings", &err)),
            }
        }
        deep_merge(&mut config, &raw);

        for typed in &self.typed {
            let (path, wire) = typed.wire();
            if get_path_in(&raw, &path).is_some() {
                issues.push(conflict(typed.key(), &path));
            }
            set_path(&mut config, &path, Some(wire));
        }

        // Tools: every one validates, and no two different tools share a name.
        let mut by_name: Vec<Tool> = Vec::new();
        for tool in self.tools.iter().cloned().chain(self.workflow_tools()) {
            match by_name.iter().find(|t| t.name() == tool.name()) {
                Some(other) => {
                    if !other.same_as(&tool) {
                        issues.push(AgentIssue::error(
                            AgentIssueCode::DuplicateToolName,
                            format!("Two different tools are named \"{}\"", tool.name()),
                            Some(format!("tools.{}", tool.name())),
                        ));
                    }
                }
                None => {
                    // Push sends `to_json()`. Only an editable tool must also fit the generated model.
                    let checked = if tool.kind() == ToolKind::Other {
                        tool.to_json().map(|_| ())
                    } else {
                        tool.build_config().map(|_| ())
                    };
                    if let Err(err) = checked {
                        issues.push(AgentIssue::error(
                            AgentIssueCode::InvalidTool,
                            err.to_string(),
                            Some(format!("tools.{}", tool.name())),
                        ));
                    }
                    if tool.kind() == ToolKind::Other && self.known_id(&tool).is_none() {
                        let tool_type = match &tool {
                            Tool::Other(other) => other.tool_type(),
                            _ => None,
                        }
                        .unwrap_or(ToolKind::Other.as_str());
                        issues.push(AgentIssue::error(
                            AgentIssueCode::InvalidTool,
                            format!(
                                "Tool \"{}\" has type \"{tool_type}\", which push doesn't create or update: load it with its id or attach it with tool_ids",
                                tool.name()
                            ),
                            Some(format!("tools.{}", tool.name())),
                        ));
                    }
                    by_name.push(tool);
                }
            }
        }

        let attached: Vec<&Tool> = self
            .tools
            .iter()
            .filter(|t| t.kind() != ToolKind::System)
            .collect();
        if self.tool_list_given {
            if get_path_in(&raw, TOOL_IDS_PATH).is_some() {
                issues.push(conflict("tools", TOOL_IDS_PATH));
            }
            let mut ids = self.tool_ids.clone();
            for tool in &attached {
                let id = id_of(tool);
                // An mcp tool added back with `add_tool` while its id is still attached.
                if is_managed(tool) || !ids.contains(&id) {
                    ids.push(id);
                }
            }
            let ids = keep_order(ids, &self.loaded_tool_ids);
            set_path(&mut config, TOOL_IDS_PATH, Some(json!(ids)));
        }
        for tool in self.tools.iter().filter(|t| t.kind() == ToolKind::System) {
            let Some(system) = tool.as_system() else {
                continue;
            };
            let mut path: Vec<&str> = BUILT_IN_TOOLS_PATH.to_vec();
            path.push(system.system_tool_type());
            if get_path_in(&raw, &path).is_some() {
                issues.push(conflict("tools", &path));
            }
            let loaded = self
                .loaded_system
                .iter()
                .find(|(t, _)| t.same_as(tool))
                .map(|(_, raw)| raw.clone());
            if let Some(wire) = loaded.or_else(|| tool.to_json().ok()) {
                set_path(&mut config, &path, Some(wire));
            }
        }

        // Workflow: its own checks, plus tool nodes can only run webhook and client tools.
        if let Some(wf) = self.workflow.get() {
            let wf_tools = wf.tools();
            for tool in wf_tools.iter().filter(|t| t.kind() == ToolKind::System) {
                issues.push(AgentIssue::error(
                    AgentIssueCode::SystemToolInWorkflow,
                    format!(
                        "System tool \"{}\" can't be used in a workflow node; add it to the agent's tools",
                        tool.name()
                    ),
                    Some(format!("workflow.tools.{}", tool.name())),
                ));
            }
            let ids: HashMap<String, String> = wf_tools
                .iter()
                .map(|tool| (tool.name().to_string(), id_of(tool)))
                .collect();
            match wf.build_wire(&ids) {
                Ok(wire) => {
                    config.insert("workflow".into(), wire);
                }
                Err(err) => issues.push(invalid_workflow(&err)),
            }
        } else if let Some(raw_workflow) = &self.raw_workflow {
            // Check the loaded graph without replacing the raw copy, which keeps fields this SDK
            // doesn't model.
            if let Err(err) = Workflow::from_json(raw_workflow).build() {
                issues.push(invalid_workflow(&err));
            }
            // Its managed tools may have new ids, e.g. when pushed as a new agent.
            let mut wire = raw_workflow.clone();
            map_raw_workflow_tool_ids(&mut wire, &|id: &str| {
                self.tools_by_id
                    .get(id)
                    .filter(|tool| is_managed(tool))
                    .and_then(|tool| tool_ids.and_then(|ids| ids.get(tool.name())))
                    .cloned()
                    .unwrap_or_else(|| id.to_string())
            });
            config.insert("workflow".into(), wire);
        }

        if let Some(ids) = tool_ids {
            for tool in self.tools() {
                if !ids.contains_key(tool.name()) {
                    issues.push(AgentIssue::error(
                        AgentIssueCode::UnresolvedTool,
                        format!(
                            "Tool \"{}\" has no id yet. Push the agent with agents_as_code().push(), or pass its id",
                            tool.name()
                        ),
                        Some(format!("tools.{}", tool.name())),
                    ));
                }
            }
        }

        let config_value = Value::Object(config);
        if !get_path(&config_value, PROMPT_PATH).is_some_and(Value::is_string) {
            issues.push(AgentIssue {
                code: AgentIssueCode::MissingPrompt,
                severity: Severity::Warning,
                message: "The agent needs a prompt".into(),
                path: Some(PROMPT_PATH.join(".")),
            });
        }

        let unique = dedupe(issues);
        let (errors, warnings): (Vec<AgentIssue>, Vec<AgentIssue>) = unique
            .into_iter()
            .partition(|issue| issue.severity == Severity::Error);
        if !errors.is_empty() {
            return Err(AgentValidationError {
                agent_name: self.name.clone(),
                issues: errors,
            });
        }
        for warning in &warnings {
            on_warning(warning);
        }

        let Value::Object(mut config) = config_value else {
            unreachable!("built as an object");
        };
        config.insert("name".into(), json!(self.name));
        for key in ["conversation_config", "platform_settings"] {
            if config.get(key).is_none_or(Value::is_null) {
                config.insert(key.into(), json!({}));
            }
        }
        config.insert("tags".into(), json!(self.tags.clone().unwrap_or_default()));
        Ok(js_canonical(&Value::Object(config)))
    }
}

/// Whether `push` creates and updates the tool: webhook and client tools only.
fn is_managed(tool: &Tool) -> bool {
    matches!(tool.kind(), ToolKind::Webhook | ToolKind::Client)
}

/// `ids` with the ones in `loaded` first, in that order, then the rest as they are.
fn keep_order(ids: Vec<String>, loaded: &[String]) -> Vec<String> {
    let rank = |id: &String| loaded.iter().position(|l| l == id).unwrap_or(loaded.len());
    let mut ranked: Vec<(usize, usize, String)> = ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (rank(&id), i, id))
        .collect();
    ranked.sort_by_key(|(rank, i, _)| (*rank, *i));
    ranked.into_iter().map(|(_, _, id)| id).collect()
}

fn invalid_config(option: &str, err: &serde_json::Error) -> AgentIssue {
    AgentIssue::error(
        AgentIssueCode::InvalidConfig,
        format!("Invalid {option}: {err}"),
        Some(option.to_string()),
    )
}

fn invalid_workflow(err: &crate::agent_workflows::WorkflowValidationError) -> AgentIssue {
    AgentIssue::error(
        AgentIssueCode::InvalidWorkflow,
        err.to_string().trim_end().to_string(),
        Some("workflow".into()),
    )
}

fn conflict<S: AsRef<str>>(option: &str, path: &[S]) -> AgentIssue {
    let path = path
        .iter()
        .map(|p| p.as_ref())
        .collect::<Vec<_>>()
        .join(".");
    AgentIssue::error(
        AgentIssueCode::OptionConflict,
        format!("\"{option}\" and the raw config both set {path}; set it in one place"),
        Some(path),
    )
}

fn dedupe(issues: Vec<AgentIssue>) -> Vec<AgentIssue> {
    let mut seen = HashSet::new();
    issues
        .into_iter()
        .filter(|issue| seen.insert((issue.code, issue.path.clone(), issue.message.clone())))
        .collect()
}
