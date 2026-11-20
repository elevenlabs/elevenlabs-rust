//! `client.agents_as_code().push()` and `.pull()`.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use serde_json::{json, Value};

use crate::agent_tools::json::js_canonical;
use crate::agent_tools::{Tool, ToolKind};
use crate::config::ClientConfig;
use crate::core::{HttpClient, RequestOptions};
use crate::error::ApiError;
use crate::ApiClient;

use super::agent::Agent;
use super::errors::{
    AgentPushAction, AgentPushError, AgentPushResult, PartialPushResult, PullError, PushError,
    ToolPush, ToolPushAction,
};
use super::project::{AgentProject, MatchBy};
use super::wire::{
    get_path, is_subset, raw_workflow_tool_ids, tool_definition, tool_ids_in, TOOL_IDS_PATH,
};

/// Options for [`AgentsAsCode::push`].
#[derive(Debug, Default)]
pub struct PushOptions<'a> {
    /// The agent to update. Without it (and without a project that knows the agent), `push`
    /// creates a new agent, so keep the returned `agent_id`. Not allowed with `project`, which
    /// records the id itself.
    pub agent_id: Option<String>,
    /// An agents-as-code project in the CLI's layout. Its `agents.json` and `tools.json` say
    /// which agent and tools on the server this definition is; `push` records new ids there and
    /// writes the configs it sent (and saves, for a project opened from a directory).
    pub project: Option<&'a mut AgentProject>,
    /// Update this branch of the agent instead of its main branch.
    pub branch_id: Option<String>,
    /// Describes the new version, for agents with versioning.
    pub version_description: Option<String>,
    /// Passed to every API call `push` makes.
    pub request_options: Option<RequestOptions>,
}

/// Options for [`AgentsAsCode::pull`].
#[derive(Debug, Default)]
pub struct PullOptions<'a> {
    /// Writes the agent and its tools into this project, and records their ids.
    pub project: Option<&'a mut AgentProject>,
    /// Pull this branch instead of the main one.
    pub branch_id: Option<String>,
    /// Passed to every API call `pull` makes.
    pub request_options: Option<RequestOptions>,
}

impl ApiClient {
    /// `push` and `pull` for agents written as code ([`Agent`]), on top of the generated
    /// `client.agents` and `client.tools` endpoints.
    pub fn agents_as_code(&self) -> AgentsAsCode<'_> {
        AgentsAsCode { client: self }
    }
}

/// `push` and `pull` for agents written as code. Returned by [`ApiClient::agents_as_code`].
///
/// The requests use the root client's configuration for the agents and tools endpoints (base
/// URL, credentials, timeout, retries), like the generated `client.agents` and `client.tools`.
/// The futures aren't `Send`, because [`Agent`] holds a [`Workflow`](crate::agent_workflows::Workflow);
/// await them directly (e.g. in `#[tokio::main]`) rather than spawning them.
#[derive(Clone, Copy)]
pub struct AgentsAsCode<'a> {
    client: &'a ApiClient,
}

impl<'a> AgentsAsCode<'a> {
    /// Creates or updates an agent and every webhook and client tool it uses, in one call.
    ///
    /// 1. Validates the agent, so an invalid one writes nothing.
    /// 2. Creates each tool that's new and updates each one that changed. Which tools already
    ///    exist comes from the project's `tools.json`, or, without a project, from the tools the
    ///    agent (`agent_id`) already uses, by name: on its main branch, or on `branch_id` when
    ///    given. Tools other agents use are never touched.
    /// 3. Creates the agent, or updates it, with the tools' ids filled in.
    ///
    /// Settings the SDK doesn't model, kept from a loaded config, are sent too. Tools the agent
    /// stops using are reported as [`Detached`](ToolPushAction::Detached), not deleted. If a
    /// request fails partway, the error ([`PushError::Push`]) carries what was written, and a
    /// project opened from a directory is saved with those ids.
    pub async fn push(
        &self,
        agent: &Agent,
        options: PushOptions<'_>,
    ) -> Result<AgentPushResult, PushError> {
        let PushOptions {
            agent_id,
            mut project,
            branch_id,
            version_description,
            request_options,
        } = options;
        if project.is_some() && agent_id.is_some() {
            return Err(PushError::InvalidOptions(
                "Pass agent_id or project to push(), not both: the project records the agent's id"
                    .into(),
            ));
        }
        agent.validate()?;
        if let Some(project) = project.as_deref() {
            project.check_branch(agent, branch_id.as_deref())?;
        }
        let agent_id = match project.as_deref() {
            Some(project) => project.agent_id(agent)?.map(str::to_string),
            None => agent_id,
        };

        let api = Api::new(self.client, request_options).map_err(|err| {
            PushError::Push(AgentPushError {
                message: "Couldn't set up the HTTP client".into(),
                result: PartialPushResult::default(),
                cause: Box::new(err),
            })
        })?;
        let mut result = PartialPushResult::default();
        let mut step = String::from("Couldn't read the agent's tools");
        let outcome = push_steps(
            &api,
            agent,
            agent_id.as_deref(),
            branch_id.as_deref(),
            version_description.as_deref(),
            project.as_deref_mut(),
            &mut result,
            &mut step,
        )
        .await;
        if let Err(cause) = outcome {
            if let Some(project) = project.as_deref_mut() {
                if project.dir().is_some() {
                    // The push error is the one to report; the ids it carries are in `result`.
                    let _ = project.save();
                }
            }
            return Err(PushError::Push(AgentPushError {
                message: step,
                result,
                cause,
            }));
        }
        if let Some(project) = project {
            if project.dir().is_some() {
                if let Err(err) = project.save() {
                    return Err(PushError::Push(AgentPushError {
                        message: "Couldn't save the project".into(),
                        result,
                        cause: Box::new(err),
                    }));
                }
            }
        }
        let (Some(agent_id), Some(action)) = (result.agent_id, result.action) else {
            unreachable!("a successful push sets the agent");
        };
        Ok(AgentPushResult {
            agent_id,
            action,
            tools: result.tools,
        })
    }

    /// Loads an existing agent as an [`Agent`], with its webhook and client tools as [`Tool`]
    /// objects; other kinds of tool stay attached by id. With a project, also writes the agent
    /// and its tools there, in the CLI's layout, matching entries by id, and records their ids
    /// (saving a project opened from a directory).
    pub async fn pull(&self, agent_id: &str, options: PullOptions<'_>) -> Result<Agent, PullError> {
        let PullOptions {
            project,
            branch_id,
            request_options,
        } = options;
        if let Some(project) = project.as_deref() {
            project.check_branch_by_id(agent_id, branch_id.as_deref())?;
        }
        let api = Api::new(self.client, request_options).map_err(PullError::Api)?;
        let live = api.get_agent(agent_id, branch_id.as_deref()).await?;
        // The CLI's pulled-config shape: these keys only, so ids and server metadata stay out of
        // the file.
        let mut config = serde_json::Map::new();
        if let Some(name) = live.get("name") {
            config.insert("name".into(), name.clone());
        }
        for (key, default) in [
            ("conversation_config", json!({})),
            ("platform_settings", json!({})),
            ("tags", json!([])),
        ] {
            let value = live.get(key).filter(|v| !v.is_null()).cloned();
            config.insert(key.into(), value.unwrap_or(default));
        }
        if let Some(workflow @ Value::Object(_)) = live.get("workflow") {
            config.insert("workflow".into(), workflow.clone());
        }
        let config = Value::Object(config);

        let mut tools_by_id: HashMap<String, Tool> = HashMap::new();
        let mut pulled: Vec<(String, Tool, Value)> = Vec::new();
        for id in tool_ids_in(&config) {
            let remote = api.get_tool(&id).await?;
            let wire = remote.get("tool_config").cloned().unwrap_or(Value::Null);
            let tool = Tool::from_json(&wire);
            // tools.json lists webhook and client tools; others (MCP...) stay attached by id.
            if matches!(tool.kind(), ToolKind::Webhook | ToolKind::Client) {
                tools_by_id.insert(id.clone(), tool.clone());
                pulled.push((id, tool, js_canonical(&wire)));
            }
        }

        let mut agent = Agent::from_json_with_tools(&config, &tools_by_id);
        if let Some(project) = project {
            for (id, tool, wire) in &pulled {
                project.record_tool(tool, id, wire, MatchBy::Id)?;
            }
            let version_id = live.get("version_id").and_then(Value::as_str);
            let index = project.record_agent(
                &agent,
                agent_id,
                &config,
                branch_id.as_deref(),
                version_id,
                MatchBy::Id,
            )?;
            if !project.is_branch_of(index, branch_id.as_deref()) {
                agent.origin = Some(project.origin(index));
            }
            if project.dir().is_some() {
                project.save()?;
            }
        }
        Ok(agent)
    }
}

type Cause = Box<dyn std::error::Error + Send + Sync>;

/// The steps of `push` that make requests. Every failure is returned with `step` naming it;
/// `result` holds what was written before.
#[allow(clippy::too_many_arguments)]
async fn push_steps(
    api: &Api,
    agent: &Agent,
    agent_id: Option<&str>,
    branch_id: Option<&str>,
    version_description: Option<&str>,
    mut project: Option<&mut AgentProject>,
    result: &mut PartialPushResult,
    step: &mut String,
) -> Result<(), Cause> {
    let tools = agent.tools();
    // The tools the agent already uses. With a project they're matched by recorded id; without
    // one, by name among the tools this agent uses, so a same-named tool another agent uses is
    // never touched.
    let in_use: Vec<Value> = match (agent_id, branch_id) {
        (None, _) => Vec::new(),
        (Some(agent_id), Some(branch_id)) => api.tools_on_branch(agent_id, branch_id).await?,
        (Some(agent_id), None) => api.list_tools_used_by(agent_id).await?,
    };
    let mut used_by_name: HashMap<String, &Value> = HashMap::new();
    for remote in &in_use {
        used_by_name.insert(tool_name(remote), remote);
    }

    let mut ids: HashMap<String, String> = HashMap::new();
    for tool in &tools {
        let known_id = match project.as_deref() {
            Some(project) => project.tool_id(tool)?.map(str::to_string),
            None => used_by_name
                .get(tool.name())
                .map(|remote| remote_id(remote)),
        };
        let mut remote: Option<Value> = None;
        if let Some(known_id) = &known_id {
            *step = format!("Couldn't read tool \"{}\" ({known_id})", tool.name());
            remote = match in_use.iter().find(|t| remote_id(t) == *known_id) {
                Some(found) => Some(found.clone()),
                None => Some(api.get_tool(known_id).await?),
            };
        }
        let unchanged_raw = match project.as_deref() {
            Some(project) => project.unchanged_raw(tool)?,
            None => None,
        };
        let config = match unchanged_raw {
            Some(raw) => raw,
            None => tool.to_json()?,
        };
        let id = match &remote {
            Some(remote) if same_tool(remote, tool, &config) => {
                let id = remote_id(remote);
                ids.insert(tool.name().to_string(), id.clone());
                result.tools.insert(
                    tool.name().to_string(),
                    ToolPush {
                        id: id.clone(),
                        action: ToolPushAction::Unchanged,
                    },
                );
                // The server already matches, but the project may hold an older definition.
                if let Some(project) = project.as_deref_mut() {
                    project.record_tool(tool, &id, &config, MatchBy::Definition)?;
                }
                continue;
            }
            Some(remote) => {
                let id = remote_id(remote);
                *step = format!("Couldn't update tool \"{}\" ({id})", tool.name());
                api.update_tool(&id, &config).await?;
                result.tools.insert(
                    tool.name().to_string(),
                    ToolPush {
                        id: id.clone(),
                        action: ToolPushAction::Updated,
                    },
                );
                id
            }
            None => {
                *step = format!("Couldn't create tool \"{}\"", tool.name());
                let created = api.create_tool(&config).await?;
                let id = created
                    .get("id")
                    .and_then(Value::as_str)
                    .ok_or("the response has no tool id")?
                    .to_string();
                result.tools.insert(
                    tool.name().to_string(),
                    ToolPush {
                        id: id.clone(),
                        action: ToolPushAction::Created,
                    },
                );
                id
            }
        };
        ids.insert(tool.name().to_string(), id.clone());
        if let Some(project) = project.as_deref_mut() {
            project.record_tool(tool, &id, &config, MatchBy::Definition)?;
        }
    }

    *step = format!("Couldn't build agent \"{}\"", agent.name());
    let config = agent.to_json(&ids)?;

    // Every id the new config still uses, wherever it is: the agent's tool_ids and the
    // workflow's nodes.
    let mut kept: HashSet<String> = get_path(&config, TOOL_IDS_PATH)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|id| id.as_str().map(str::to_string))
        .collect();
    kept.extend(raw_workflow_tool_ids(config.get("workflow")));
    for remote in &in_use {
        let name = tool_name(remote);
        if !kept.contains(&remote_id(remote)) && !result.tools.contains_key(&name) {
            result.tools.insert(
                name,
                ToolPush {
                    id: remote_id(remote),
                    action: ToolPushAction::Detached,
                },
            );
        }
    }

    let (id, version_id) = match agent_id {
        None => {
            *step = format!("Couldn't create agent \"{}\"", agent.name());
            let created = api.create_agent(&config).await?;
            let id = created
                .get("agent_id")
                .and_then(Value::as_str)
                .ok_or("the response has no agent_id")?
                .to_string();
            result.action = Some(AgentPushAction::Created);
            (id, None)
        }
        Some(agent_id) => {
            *step = format!("Couldn't update agent \"{}\" ({agent_id})", agent.name());
            let mut body = config.clone();
            if let (Some(description), Some(obj)) = (version_description, body.as_object_mut()) {
                obj.insert("version_description".into(), json!(description));
            }
            let updated = api.update_agent(agent_id, branch_id, &body).await?;
            result.action = Some(AgentPushAction::Updated);
            let version_id = updated
                .get("version_id")
                .and_then(Value::as_str)
                .map(str::to_string);
            (agent_id.to_string(), version_id)
        }
    };
    result.agent_id = Some(id.clone());
    if let Some(project) = project {
        project.record_agent(
            agent,
            &id,
            &config,
            branch_id,
            version_id.as_deref(),
            MatchBy::Definition,
        )?;
    }
    Ok(())
}

fn remote_id(remote: &Value) -> String {
    remote
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn tool_name(remote: &Value) -> String {
    remote
        .get("tool_config")
        .and_then(|c| c.get("name"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| remote_id(remote))
}

/// Whether the server already has this tool as defined. Modeled fields are compared through the
/// builder and the generated model, so both sides normalize alike; the config to send must also
/// be contained in the server's, so a change to a field the builder doesn't model still counts.
fn same_tool(remote: &Value, tool: &Tool, config: &Value) -> bool {
    let wire = remote.get("tool_config").cloned().unwrap_or(Value::Null);
    let local = tool_definition(tool);
    local.is_some() && tool_definition(&Tool::from_json(&wire)) == local && is_subset(config, &wire)
}

/// One endpoint group's transport: built from the same configuration that supplies the
/// credentials, as the generated clients do.
struct Transport {
    http: HttpClient,
    config: ClientConfig,
}

impl Transport {
    fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http = HttpClient::new(
            config.base_url.clone(),
            config.timeout.as_millis() as u64,
            config.max_retries,
        )
        .map_err(ApiError::Network)?;
        Ok(Self { http, config })
    }
}

/// The requests `push` and `pull` make, in API format. Raw JSON in both directions, so fields
/// the generated types don't declare reach the API and come back.
struct Api {
    agents: Transport,
    tools: Transport,
    options: Option<RequestOptions>,
}

impl Api {
    fn new(client: &ApiClient, options: Option<RequestOptions>) -> Result<Self, ApiError> {
        Ok(Self {
            agents: Transport::new(client.config.get_agents_config())?,
            tools: Transport::new(client.config.get_tools_config())?,
            options,
        })
    }

    async fn get_agent(&self, agent_id: &str, branch_id: Option<&str>) -> Result<Value, ApiError> {
        let query: Vec<(&str, &str)> = branch_id.map(|b| ("branch_id", b)).into_iter().collect();
        let path = format!("/v1/convai/agents/{}", segment(agent_id));
        self.send(&self.agents, reqwest::Method::GET, &path, &query, None)
            .await
    }

    async fn create_agent(&self, body: &Value) -> Result<Value, ApiError> {
        self.send(
            &self.agents,
            reqwest::Method::POST,
            "/v1/convai/agents/create",
            &[],
            Some(body),
        )
        .await
    }

    async fn update_agent(
        &self,
        agent_id: &str,
        branch_id: Option<&str>,
        body: &Value,
    ) -> Result<Value, ApiError> {
        let query: Vec<(&str, &str)> = branch_id.map(|b| ("branch_id", b)).into_iter().collect();
        let path = format!("/v1/convai/agents/{}", segment(agent_id));
        self.send(
            &self.agents,
            reqwest::Method::PATCH,
            &path,
            &query,
            Some(body),
        )
        .await
    }

    async fn get_tool(&self, tool_id: &str) -> Result<Value, ApiError> {
        let path = format!("/v1/convai/tools/{}", segment(tool_id));
        self.send(&self.tools, reqwest::Method::GET, &path, &[], None)
            .await
    }

    async fn create_tool(&self, config: &Value) -> Result<Value, ApiError> {
        let body = json!({ "tool_config": config });
        self.send(
            &self.tools,
            reqwest::Method::POST,
            "/v1/convai/tools",
            &[],
            Some(&body),
        )
        .await
    }

    async fn update_tool(&self, tool_id: &str, config: &Value) -> Result<Value, ApiError> {
        let body = json!({ "tool_config": config });
        let path = format!("/v1/convai/tools/{}", segment(tool_id));
        self.send(&self.tools, reqwest::Method::PATCH, &path, &[], Some(&body))
            .await
    }

    /// Every tool the agent's main branch uses (the API's `used_by_agent_id` filter), following
    /// the cursor.
    async fn list_tools_used_by(&self, agent_id: &str) -> Result<Vec<Value>, ApiError> {
        let mut all = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let mut query = vec![("used_by_agent_id", agent_id), ("page_size", "100")];
            if let Some(cursor) = &cursor {
                query.push(("cursor", cursor.as_str()));
            }
            let page = self
                .send(
                    &self.tools,
                    reqwest::Method::GET,
                    "/v1/convai/tools",
                    &query,
                    None,
                )
                .await?;
            all.extend(
                page.get("tools")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default(),
            );
            let next = page
                .get("next_cursor")
                .and_then(Value::as_str)
                .map(str::to_string);
            match next {
                Some(next) if page.get("has_more") == Some(&Value::Bool(true)) => {
                    cursor = Some(next)
                }
                _ => return Ok(all),
            }
        }
    }

    /// The tools a branch of the agent uses. The tools API's `used_by_agent_id` covers the main
    /// branch only, so they come from the branch's config, by id.
    async fn tools_on_branch(
        &self,
        agent_id: &str,
        branch_id: &str,
    ) -> Result<Vec<Value>, ApiError> {
        let branch = self.get_agent(agent_id, Some(branch_id)).await?;
        let mut tools = Vec::new();
        for id in tool_ids_in(&branch) {
            tools.push(self.get_tool(&id).await?);
        }
        Ok(tools)
    }

    /// Sends one request the way the generated methods do (per-request headers, query
    /// parameters, timeout, API key override and retries, the client's custom headers, the same
    /// error mapping), returning the JSON response.
    async fn send(
        &self,
        transport: &Transport,
        method: reqwest::Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&Value>,
    ) -> Result<Value, ApiError> {
        let options = self.options.as_ref();
        let mut request = match method {
            reqwest::Method::POST => transport.http.post(path),
            reqwest::Method::PATCH => transport.http.patch(path),
            _ => transport.http.get(path),
        };
        if !query.is_empty() {
            request = request.query(query);
        }
        if let Some(opts) = options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(Duration::from_secs(timeout_seconds));
            }
        }
        if let Some(body) = body {
            request = match options.and_then(|opts| opts.merge_into_body(body)) {
                Some(merged) => request.json(&merged),
                None => request.json(body),
            };
        }
        let mut config = transport.config.clone();
        if let Some(api_key) = options.and_then(|opts| opts.api_key.clone()) {
            config.api_key = Some(api_key);
        }
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = transport.http.send_with_retry(request, options).await?;
        let status = response.status().as_u16();
        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ApiError::from_response(status, Some(body.as_str())));
        }
        response.json().await.map_err(|err| ApiError::DecodeError {
            status,
            message: format!("{err}"),
        })
    }
}

/// An id as one URL path segment: anything but unreserved characters is percent-encoded, so an
/// id read from a project file can't change the path.
fn segment(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for byte in id.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}
