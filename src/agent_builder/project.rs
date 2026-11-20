//! `AgentProject`: agents and tools in the ElevenLabs CLI's project layout.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Map, Value};

use crate::agent_tools::Tool;

use super::agent::Agent;
use super::errors::AgentProjectError;
use super::format::{normalize_path, unique_path};
use super::fs::{read_project, write_project_files};
use super::wire::{format_config, format_index, tool_definition};

pub const AGENTS_FILE: &str = "agents.json";
pub const TOOLS_FILE: &str = "tools.json";
pub const AGENT_CONFIGS_DIR: &str = "agent_configs";
pub const TOOL_CONFIGS_DIR: &str = "tool_configs";

/// One entry in `agents.json`, as the CLI writes it.
#[derive(Debug, Clone)]
struct AgentEntry {
    /// Path to the agent's config file, relative to the project root.
    config: String,
    id: Option<String>,
    branch_id: Option<String>,
    version_id: Option<String>,
    /// `{ name: { config, branch_id, version_id? } }`, kept as loaded.
    branches: Option<Map<String, Value>>,
}

/// One entry in `tools.json`, as the CLI writes it.
#[derive(Debug, Clone)]
struct ToolEntry {
    /// `"webhook"` or `"client"`.
    kind: Option<String>,
    config: String,
    id: Option<String>,
}

struct AgentSlot {
    entry: AgentEntry,
    agent: Agent,
}

struct ToolSlot {
    entry: ToolEntry,
    tool: Tool,
    /// The config as loaded, in API format, sent as-is while the tool is unchanged so fields the
    /// SDK doesn't model survive.
    raw: Option<Value>,
    /// The tool's definition as loaded, to tell whether it was edited since.
    baseline: Option<String>,
}

/// How `record_tool` / `record_agent` find an existing entry: `push` by definition (the entry the
/// object was loaded from, else the only one with its name), `pull` by id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MatchBy {
    Definition,
    Id,
}

/// An agents-as-code project in the CLI's layout: `agents.json` and `tools.json` list the agents
/// and tools with their ids, and each one's config sits in `agent_configs/` or `tool_configs/` in
/// API format. A project made by `elevenlabs agents init` / `pull` works here, and one written
/// here works with the CLI.
///
/// Pass it to `push` and `pull` (`PushOptions { project: Some(&mut project), .. }`): the project
/// records which agent and tools on the server each definition is, so pushing again updates them
/// instead of making copies.
///
/// ```no_run
/// # async fn run(client: &elevenlabs_rust_sdk::ApiClient) -> Result<(), Box<dyn std::error::Error>> {
/// use elevenlabs_rust_sdk::agent_builder::{AgentOptions, AgentProject, PushOptions};
///
/// let mut project = AgentProject::open(".")?;
/// let mut agent = project.agent("Support line")?.clone();
/// agent.update(AgentOptions::new().first_message("Hello!"));
/// client
///     .agents_as_code()
///     .push(&agent, PushOptions { project: Some(&mut project), ..Default::default() })
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Only `push` and `pull` write files, after they know every id. A file nothing changed keeps
/// its exact text. YAML and TOML configs, `tests.json`, and creating branches aren't supported
/// yet.
pub struct AgentProject {
    dir: Option<PathBuf>,
    uid: u64,
    files: HashMap<String, String>,
    agents: Vec<AgentSlot>,
    tools: Vec<ToolSlot>,
    /// Paths written since loading, with their new contents, in the order first written.
    changed: Vec<(String, String)>,
    /// Paths in use on disk, so new files don't overwrite one the project doesn't list.
    taken: HashSet<String>,
}

impl fmt::Debug for AgentProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AgentProject")
            .field("dir", &self.dir)
            .field("agents", &self.agents.len())
            .field("tools", &self.tools.len())
            .field("changed_files", &self.changed_files())
            .finish()
    }
}

impl AgentProject {
    fn new(
        files: HashMap<String, String>,
        dir: Option<PathBuf>,
        taken: Vec<String>,
    ) -> Result<Self, AgentProjectError> {
        static NEXT_UID: AtomicU64 = AtomicU64::new(1);
        let files: HashMap<String, String> = files
            .into_iter()
            .map(|(path, text)| (normalize_path(&path), text))
            .collect();
        let mut taken_set: HashSet<String> = files.keys().cloned().collect();
        taken_set.extend(taken.iter().map(|p| normalize_path(p)));
        let mut project = AgentProject {
            dir,
            uid: NEXT_UID.fetch_add(1, Ordering::Relaxed),
            files,
            agents: Vec::new(),
            tools: Vec::new(),
            changed: Vec::new(),
            taken: taken_set,
        };
        project.load()?;
        Ok(project)
    }

    /// A project from file contents, keyed by path relative to the project root. No I/O. Only
    /// `agents.json`, `tools.json` and the config files they list are read. Missing index files
    /// mean an empty project.
    pub fn from_files<K, V, I>(files: I) -> Result<Self, AgentProjectError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let files = files
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect();
        Self::new(files, None, Vec::new())
    }

    /// Reads the project in `dir`. A directory without `agents.json` and `tools.json` opens as an
    /// empty project, and `save()` creates them. Uses blocking file I/O.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self, AgentProjectError> {
        let dir = dir.as_ref();
        let (files, taken) = read_project(dir)?;
        Self::new(files, Some(dir.to_path_buf()), taken)
    }

    /// The directory the project was opened from, if any. `save()` writes there.
    pub fn dir(&self) -> Option<&Path> {
        self.dir.as_deref()
    }

    /// Every agent in the project, in `agents.json` order.
    pub fn agents(&self) -> Vec<&Agent> {
        self.agents.iter().map(|slot| &slot.agent).collect()
    }

    /// The agent named `name`. The same object every time; clone it to edit and push it (a clone
    /// still matches this entry).
    pub fn agent(&self, name: &str) -> Result<&Agent, AgentProjectError> {
        let index = self.agent_by_name(name)?;
        Ok(&self.agents[index].agent)
    }

    /// The agent named `name`, for editing in place.
    pub fn agent_mut(&mut self, name: &str) -> Result<&mut Agent, AgentProjectError> {
        let index = self.agent_by_name(name)?;
        Ok(&mut self.agents[index].agent)
    }

    fn agent_by_name(&self, name: &str) -> Result<usize, AgentProjectError> {
        let matches: Vec<usize> = (0..self.agents.len())
            .filter(|&i| self.agents[i].agent.name() == name)
            .collect();
        match matches.as_slice() {
            [index] => Ok(*index),
            [] => Err(AgentProjectError::new(format!(
                "No agent named \"{name}\" in {AGENTS_FILE}"
            ))),
            more => Err(AgentProjectError::new(format!(
                "{} agents in {AGENTS_FILE} are named \"{name}\"",
                more.len()
            ))),
        }
    }

    /// Every tool in `tools.json`, in order.
    pub fn tools(&self) -> Vec<&Tool> {
        self.tools.iter().map(|slot| &slot.tool).collect()
    }

    /// The id `agents.json` records for this agent, if it was pushed or pulled. The entry is the
    /// one the agent was loaded from, else the only one with its name.
    pub fn agent_id(&self, agent: &Agent) -> Result<Option<&str>, AgentProjectError> {
        Ok(self
            .agent_slot(agent)?
            .and_then(|i| self.agents[i].entry.id.as_deref()))
    }

    /// The id `tools.json` records for this tool, if it was pushed or pulled. The entry is one
    /// holding this same definition, else the only one with its name.
    pub fn tool_id(&self, tool: &Tool) -> Result<Option<&str>, AgentProjectError> {
        Ok(self
            .tool_slot(tool)?
            .and_then(|i| self.tools[i].entry.id.as_deref()))
    }

    /// The project's files, keyed by path: the index files and every config they list (branch
    /// configs included), with what push and pull wrote since loading. Files nothing changed
    /// keep their exact text.
    pub fn to_files(&self) -> BTreeMap<String, String> {
        let mut files = BTreeMap::new();
        let agents_index = self
            .current(AGENTS_FILE)
            .map(str::to_string)
            .unwrap_or_else(|| self.agents_index());
        let tools_index = self
            .current(TOOLS_FILE)
            .map(str::to_string)
            .unwrap_or_else(|| self.tools_index());
        files.insert(AGENTS_FILE.to_string(), agents_index);
        files.insert(TOOLS_FILE.to_string(), tools_index);
        let mut paths: Vec<String> = self
            .agents
            .iter()
            .map(|slot| slot.entry.config.clone())
            .collect();
        for slot in &self.agents {
            paths.extend(branch_configs(&slot.entry));
        }
        paths.extend(self.tools.iter().map(|slot| slot.entry.config.clone()));
        for path in paths {
            if let Some(text) = self.current(&path) {
                files.insert(path, text.to_string());
            }
        }
        files
    }

    fn current(&self, path: &str) -> Option<&str> {
        let key = normalize_path(path);
        self.changed
            .iter()
            .find(|(p, _)| *p == key)
            .map(|(_, text)| text.as_str())
            .or_else(|| self.files.get(&key).map(String::as_str))
    }

    /// The paths push and pull changed since the project was loaded or last saved.
    pub fn changed_files(&self) -> Vec<String> {
        self.changed.iter().map(|(path, _)| path.clone()).collect()
    }

    /// Writes the files push and pull changed to the project's directory, and returns their
    /// paths. `push` and `pull` call this themselves for a project opened with `open()`.
    pub fn save(&mut self) -> Result<Vec<String>, AgentProjectError> {
        let Some(dir) = &self.dir else {
            return Err(AgentProjectError::new(
                "This project wasn't opened from a directory; use to_files() instead",
            ));
        };
        if self.changed.is_empty() {
            return Ok(Vec::new());
        }
        write_project_files(dir, &self.changed)?;
        let written = std::mem::take(&mut self.changed);
        let paths = written.iter().map(|(path, _)| path.clone()).collect();
        for (path, text) in written {
            self.files.insert(path, text);
        }
        Ok(paths)
    }

    /// The entry for an agent: the one it was loaded from, else the only one with its name.
    fn agent_slot(&self, agent: &Agent) -> Result<Option<usize>, AgentProjectError> {
        if let Some((uid, index)) = agent.origin {
            if uid == self.uid && index < self.agents.len() {
                return Ok(Some(index));
            }
        }
        single(
            (0..self.agents.len())
                .filter(|&i| self.agents[i].agent.name() == agent.name())
                .collect(),
            AGENTS_FILE,
            agent.name(),
        )
    }

    /// The entry for a tool: one holding this same definition, else the only one with its name.
    fn tool_slot(&self, tool: &Tool) -> Result<Option<usize>, AgentProjectError> {
        if let Some(index) = self.tools.iter().position(|slot| slot.tool.same_as(tool)) {
            return Ok(Some(index));
        }
        single(
            (0..self.tools.len())
                .filter(|&i| self.tools[i].tool.name() == tool.name())
                .collect(),
            TOOLS_FILE,
            tool.name(),
        )
    }

    /// The loaded config to send for a tool whose definition matches its project entry (the
    /// loaded tool, or one rebuilt in code with the same name), so fields this SDK doesn't model
    /// survive.
    pub(crate) fn unchanged_raw(&self, tool: &Tool) -> Result<Option<Value>, AgentProjectError> {
        let Some(index) = self.tool_slot(tool)? else {
            return Ok(None);
        };
        let slot = &self.tools[index];
        match (&slot.raw, &slot.baseline) {
            (Some(raw), Some(baseline)) if tool_definition(tool).as_ref() == Some(baseline) => {
                Ok(Some(raw.clone()))
            }
            _ => Ok(None),
        }
    }

    /// Records a tool's id and the config now on the server, adding an entry if it's new.
    pub(crate) fn record_tool(
        &mut self,
        tool: &Tool,
        id: &str,
        config: &Value,
        match_by: MatchBy,
    ) -> Result<(), AgentProjectError> {
        let found = match match_by {
            MatchBy::Id => self
                .tools
                .iter()
                .position(|slot| slot.entry.id.as_deref() == Some(id)),
            MatchBy::Definition => self.tool_slot(tool)?,
        };
        let index = match found {
            Some(index) => index,
            None => {
                let path = unique_path(TOOL_CONFIGS_DIR, tool.name(), &self.taken);
                self.taken.insert(path.clone());
                self.tools.push(ToolSlot {
                    entry: ToolEntry {
                        kind: None,
                        config: path,
                        id: None,
                    },
                    tool: tool.clone(),
                    raw: None,
                    baseline: None,
                });
                self.tools.len() - 1
            }
        };
        let slot = &mut self.tools[index];
        slot.tool = tool.clone();
        slot.entry.id = Some(id.to_string());
        slot.entry.kind = Some(tool.kind().as_str().to_string());
        slot.raw = Some(config.clone());
        slot.baseline = tool_definition(tool);
        let path = slot.entry.config.clone();
        self.write(&path, format_config(config));
        let index_text = self.tools_index();
        self.write(TOOLS_FILE, index_text);
        Ok(())
    }

    /// Records an agent's id and its config, adding an entry if it's new. A branch push writes
    /// the branch's file and leaves the main agent alone. Returns the entry's index.
    pub(crate) fn record_agent(
        &mut self,
        agent: &Agent,
        id: &str,
        config: &Value,
        branch_id: Option<&str>,
        version_id: Option<&str>,
        match_by: MatchBy,
    ) -> Result<usize, AgentProjectError> {
        let found = match match_by {
            MatchBy::Id => self
                .agents
                .iter()
                .position(|slot| slot.entry.id.as_deref() == Some(id)),
            MatchBy::Definition => self.agent_slot(agent)?,
        };
        let index = match found {
            Some(index) => index,
            None => {
                let path = unique_path(AGENT_CONFIGS_DIR, agent.name(), &self.taken);
                self.taken.insert(path.clone());
                self.agents.push(AgentSlot {
                    entry: AgentEntry {
                        config: path,
                        id: None,
                        branch_id: None,
                        version_id: None,
                        branches: None,
                    },
                    agent: agent.clone(),
                });
                self.agents.len() - 1
            }
        };
        let uid = self.uid;
        let slot = &mut self.agents[index];
        slot.entry.id = Some(id.to_string());
        let branch = branch_id.and_then(|branch_id| {
            slot.entry.branches.as_mut().and_then(|branches| {
                branches
                    .values_mut()
                    .find(|b| b.get("branch_id").and_then(Value::as_str) == Some(branch_id))
            })
        });
        let path = match branch {
            // A branch has its own file; the slot's agent is the main branch's.
            Some(branch) => {
                if let (Some(version_id), Some(obj)) = (version_id, branch.as_object_mut()) {
                    obj.insert("version_id".into(), Value::String(version_id.to_string()));
                }
                branch
                    .get("config")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string()
            }
            None => {
                let mut stored = agent.clone();
                stored.origin = Some((uid, index));
                slot.agent = stored;
                if let Some(version_id) = version_id {
                    slot.entry.version_id = Some(version_id.to_string());
                }
                slot.entry.config.clone()
            }
        };
        self.write(&path, format_config(config));
        let index_text = self.agents_index();
        self.write(AGENTS_FILE, index_text);
        Ok(index)
    }

    /// Whether `branch_id` is one of the entry's branches with its own file (so recording it left
    /// the entry's main agent alone).
    pub(crate) fn is_branch_of(&self, index: usize, branch_id: Option<&str>) -> bool {
        let Some(branch_id) = branch_id else {
            return false;
        };
        self.agents.get(index).is_some_and(|slot| {
            slot.entry.branches.as_ref().is_some_and(|branches| {
                branches
                    .values()
                    .any(|b| b.get("branch_id").and_then(Value::as_str) == Some(branch_id))
            })
        })
    }

    /// The `origin` an agent recorded at `index` carries, so it matches that entry later.
    pub(crate) fn origin(&self, index: usize) -> (u64, usize) {
        (self.uid, index)
    }

    /// Rejects a branch the project has no file for, before anything is written: its settings
    /// would otherwise land in the main branch's file, and a later CLI push would send them to
    /// the main branch.
    pub(crate) fn check_branch(
        &self,
        agent: &Agent,
        branch_id: Option<&str>,
    ) -> Result<(), AgentProjectError> {
        if branch_id.is_none() {
            return Ok(());
        }
        let slot = self.agent_slot(agent)?.map(|i| &self.agents[i]);
        check_branch_of(slot, agent.name(), branch_id)
    }

    /// `check_branch` for `pull`, which finds the agent by id.
    pub(crate) fn check_branch_by_id(
        &self,
        agent_id: &str,
        branch_id: Option<&str>,
    ) -> Result<(), AgentProjectError> {
        let slot = self
            .agents
            .iter()
            .find(|s| s.entry.id.as_deref() == Some(agent_id));
        check_branch_of(slot, agent_id, branch_id)
    }

    fn write(&mut self, path: &str, text: String) {
        let key = normalize_path(path);
        if self.files.get(&key) == Some(&text) {
            self.changed.retain(|(p, _)| *p != key);
            return;
        }
        match self.changed.iter_mut().find(|(p, _)| *p == key) {
            Some(slot) => slot.1 = text,
            None => self.changed.push((key, text)),
        }
    }

    fn load(&mut self) -> Result<(), AgentProjectError> {
        let agents_index = self.read_index(AGENTS_FILE, "agents")?;
        let tools_index = self.read_index(TOOLS_FILE, "tools")?;

        let mut by_id: HashMap<String, Tool> = HashMap::new();
        for entry in tools_index {
            let config = config_path(&entry, TOOLS_FILE)?;
            let raw = self.read_config(&config, TOOLS_FILE)?;
            let tool = Tool::from_json(&raw);
            let id = string_field(&entry, "id");
            if let Some(id) = &id {
                by_id.insert(id.clone(), tool.clone());
            }
            self.tools.push(ToolSlot {
                entry: ToolEntry {
                    kind: string_field(&entry, "type"),
                    config,
                    id,
                },
                baseline: tool_definition(&tool),
                tool,
                raw: Some(raw),
            });
        }
        for entry in agents_index {
            let config = config_path(&entry, AGENTS_FILE)?;
            let raw = self.read_config(&config, AGENTS_FILE)?;
            let mut agent = Agent::from_json_with_tools(&raw, &by_id);
            agent.origin = Some((self.uid, self.agents.len()));
            self.agents.push(AgentSlot {
                entry: AgentEntry {
                    config,
                    id: string_field(&entry, "id"),
                    branch_id: string_field(&entry, "branch_id"),
                    version_id: string_field(&entry, "version_id"),
                    branches: entry.get("branches").and_then(Value::as_object).cloned(),
                },
                agent,
            });
        }
        Ok(())
    }

    fn read_index(&self, path: &str, key: &str) -> Result<Vec<Value>, AgentProjectError> {
        let Some(text) = self.files.get(path) else {
            return Ok(Vec::new());
        };
        let parsed: Value = serde_json::from_str(text)
            .map_err(|err| AgentProjectError::new(format!("{path} isn't valid JSON: {err}")))?;
        match parsed.get(key) {
            None | Some(Value::Null) => Ok(Vec::new()),
            Some(Value::Array(entries)) => Ok(entries.clone()),
            Some(_) => Err(AgentProjectError::new(format!(
                "{path}: \"{key}\" isn't a list"
            ))),
        }
    }

    fn read_config(&self, path: &str, index: &str) -> Result<Value, AgentProjectError> {
        if !path.to_ascii_lowercase().ends_with(".json") {
            return Err(AgentProjectError::new(format!(
                "{index} points at {path}: only JSON config files are supported yet (YAML and TOML aren't)"
            )));
        }
        let Some(text) = self.files.get(&normalize_path(path)) else {
            return Err(AgentProjectError::new(format!(
                "{index} points at {path}, which doesn't exist"
            )));
        };
        let value: Value = serde_json::from_str(text)
            .map_err(|err| AgentProjectError::new(format!("{path} isn't valid JSON: {err}")))?;
        if !value.is_object() {
            return Err(AgentProjectError::new(format!(
                "{path} isn't a JSON object"
            )));
        }
        Ok(value)
    }

    fn agents_index(&self) -> String {
        let agents: Vec<Value> = self
            .agents
            .iter()
            .map(|slot| {
                // The CLI writes fields in this order and leaves unset ones out.
                let entry = &slot.entry;
                let mut obj = Map::new();
                obj.insert("config".into(), Value::String(entry.config.clone()));
                insert_some(&mut obj, "id", &entry.id);
                insert_some(&mut obj, "branch_id", &entry.branch_id);
                insert_some(&mut obj, "version_id", &entry.version_id);
                if let Some(branches) = &entry.branches {
                    obj.insert("branches".into(), Value::Object(branches.clone()));
                }
                Value::Object(obj)
            })
            .collect();
        let mut index = Map::new();
        index.insert("agents".into(), Value::Array(agents));
        format_index(&Value::Object(index))
    }

    fn tools_index(&self) -> String {
        let tools: Vec<Value> = self
            .tools
            .iter()
            .map(|slot| {
                let entry = &slot.entry;
                let mut obj = Map::new();
                insert_some(&mut obj, "type", &entry.kind);
                obj.insert("config".into(), Value::String(entry.config.clone()));
                insert_some(&mut obj, "id", &entry.id);
                Value::Object(obj)
            })
            .collect();
        let mut index = Map::new();
        index.insert("tools".into(), Value::Array(tools));
        format_index(&Value::Object(index))
    }
}

fn check_branch_of(
    slot: Option<&AgentSlot>,
    label: &str,
    branch_id: Option<&str>,
) -> Result<(), AgentProjectError> {
    let Some(branch_id) = branch_id else {
        return Ok(());
    };
    if slot.is_some_and(|s| s.entry.branch_id.as_deref() == Some(branch_id)) {
        return Ok(());
    }
    let known = slot
        .and_then(|s| s.entry.branches.as_ref())
        .is_some_and(|branches| {
            branches
                .values()
                .any(|b| b.get("branch_id").and_then(Value::as_str) == Some(branch_id))
        });
    if known {
        return Ok(());
    }
    Err(AgentProjectError::new(format!(
        "Branch {branch_id} of \"{label}\" has no config file in {AGENTS_FILE}. Add it under \"branches\" (the CLI's `agents pull --all-branches` does), or push the main branch"
    )))
}

fn branch_configs(entry: &AgentEntry) -> Vec<String> {
    entry
        .branches
        .iter()
        .flat_map(|branches| branches.values())
        .filter_map(|b| b.get("config").and_then(Value::as_str).map(str::to_string))
        .collect()
}

fn single(
    matches: Vec<usize>,
    index: &str,
    name: &str,
) -> Result<Option<usize>, AgentProjectError> {
    if matches.len() > 1 {
        return Err(AgentProjectError::new(format!(
            "{} entries in {index} are named \"{name}\"; push the loaded one",
            matches.len()
        )));
    }
    Ok(matches.first().copied())
}

fn config_path(entry: &Value, index: &str) -> Result<String, AgentProjectError> {
    entry
        .get("config")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| {
            AgentProjectError::new(format!("{index} has an entry without a \"config\" path"))
        })
}

fn string_field(entry: &Value, key: &str) -> Option<String> {
    entry.get(key).and_then(Value::as_str).map(str::to_string)
}

fn insert_some(obj: &mut Map<String, Value>, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        obj.insert(key.into(), Value::String(value.clone()));
    }
}
