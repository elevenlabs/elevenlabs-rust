//! Wait for a flows generation or template run to finish.
//!
//! The flows endpoints are asynchronous: `create` returns straight away with a `pending` status,
//! and `get` reports progress until the work is `completed` or `failed`. `create_and_wait` does
//! both, polling `get` at the pace the server asks for in its `Retry-After` header.
//!
//! ```rust,no_run
//! use elevenlabs_rust_sdk::flows::FlowsWaitOptions;
//! use elevenlabs_rust_sdk::{ApiClient, ClientConfig, ImageGenerationRequest};
//! use std::time::Duration;
//!
//! # async fn run(request: ImageGenerationRequest) -> Result<(), Box<dyn std::error::Error>> {
//! let client = ApiClient::new(ClientConfig::builder().api_key("YOUR_API_KEY").build()?)?;
//! let generation = client
//!     .flows()
//!     .image()
//!     .create_and_wait(
//!         &request,
//!         None,
//!         FlowsWaitOptions::default().wait_timeout(Duration::from_secs(300)),
//!     )
//!     .await?;
//! if let Some(completed) = generation.as_completed() {
//!     println!("{}", completed.content_url);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! This whole module is hand-written (it is listed in `.fernignore`), mirroring
//! `create_and_wait` in the Python SDK and `createAndWait` in the TypeScript SDK. The generated
//! `create` and `get` methods do not expose response headers, so the requests are built here
//! with the same configuration, auth and retries the generated methods use, which lets the wait
//! read the server's `Retry-After` header.
//!
//! The helpers always use the root client's configuration, for the transport (base URL, timeout,
//! retries) as well as the credentials, so both always come from the same place. Replacing a public
//! sub-client such as `client.image` with a standalone one does not change where they send
//! requests.

mod poll;

#[cfg(test)]
mod tests;

use std::marker::PhantomData;
use std::time::Duration;

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::types::{
    ImageGenerationRequest, MediaGenerationCreateResponse, MediaGenerationResponse,
    TemplateRunCreateRequest, TemplateRunResponse, TemplateRunStatus,
    TextToSpeechGenerationRequest, VideoGenerationRequest,
};
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::{HttpClient, RequestOptions};
use crate::error::ApiError;
use crate::ApiClient;

use poll::{Pacer, Polled, RealPacer};

/// The default shortest wait between status checks. Matches the server's shortest `Retry-After`
/// (3s for image, audio and text), so the SDK polls no faster than the server would pace it when
/// the header is missing.
pub const DEFAULT_MIN_POLL_INTERVAL: Duration = Duration::from_secs(3);

/// The smallest accepted `min_poll_interval`. Below this, a missing server hint would turn the
/// wait into a tight loop of status checks.
pub const SMALLEST_MIN_POLL_INTERVAL: Duration = Duration::from_millis(500);

/// How long `create_and_wait` waits, and how often it checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlowsWaitOptions {
    /// Total time to wait before returning [`FlowsWaitError::Timeout`], which carries the id so
    /// you can resume with `get`. `None` (the default) waits indefinitely; `Duration::ZERO` times
    /// out straight after the create. Not to be confused with the client's `timeout` or
    /// `RequestOptions::timeout_seconds`, which limit each HTTP request.
    pub wait_timeout: Option<Duration>,
    /// The shortest wait between status checks, and the wait used when the server suggests none.
    /// Defaults to [`DEFAULT_MIN_POLL_INTERVAL`]; must be at least [`SMALLEST_MIN_POLL_INTERVAL`].
    pub min_poll_interval: Duration,
}

impl Default for FlowsWaitOptions {
    fn default() -> Self {
        Self {
            wait_timeout: None,
            min_poll_interval: DEFAULT_MIN_POLL_INTERVAL,
        }
    }
}

impl FlowsWaitOptions {
    /// Wait at most `timeout` in total.
    pub fn wait_timeout(mut self, timeout: Duration) -> Self {
        self.wait_timeout = Some(timeout);
        self
    }

    /// Never wait less than `interval` between status checks.
    pub fn min_poll_interval(mut self, interval: Duration) -> Self {
        self.min_poll_interval = interval;
        self
    }

    fn validate(&self) -> Result<(), FlowsWaitError> {
        if self.min_poll_interval < SMALLEST_MIN_POLL_INTERVAL {
            return Err(FlowsWaitError::InvalidOptions(format!(
                "min_poll_interval must be at least {}s, got {}s",
                SMALLEST_MIN_POLL_INTERVAL.as_secs_f64(),
                self.min_poll_interval.as_secs_f64()
            )));
        }
        // A negative or NaN `wait_timeout` cannot be expressed as a `Duration`, so the remaining
        // checks of the Python and TypeScript SDKs hold by construction.
        Ok(())
    }
}

/// Returned by `create_and_wait` when a generation or template run is still unfinished after
/// `wait_timeout`.
///
/// The work keeps running on the server. Resume waiting by calling `get` with `id` (and
/// `template_id` for a template run), or let a webhook deliver the result.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{}", self.message())]
pub struct FlowsWaitTimeoutError {
    /// The generation id, or the run id for a template run.
    pub id: String,
    /// The template id, for a template run; `None` for a generation.
    pub template_id: Option<String>,
    /// The `wait_timeout` that ran out.
    pub wait_timeout: Duration,
    /// The status of the last response received, such as `pending` or `generating`.
    pub last_status: String,
}

impl FlowsWaitTimeoutError {
    fn message(&self) -> String {
        let seconds = self.wait_timeout.as_secs_f64();
        match &self.template_id {
            Some(template_id) => format!(
                "Template run {} of template {} is still {} after {}s; call `runs.get` with `template_id` and `id` to keep waiting",
                self.id, template_id, self.last_status, seconds
            ),
            None => format!(
                "Generation {} is still {} after {}s; call `get` with this id to keep waiting",
                self.id, self.last_status, seconds
            ),
        }
    }
}

/// Why `create_and_wait` returned without a finished generation or run.
#[derive(Debug, thiserror::Error)]
pub enum FlowsWaitError {
    /// The wait options were rejected before any request was sent.
    #[error("Invalid wait options: {0}")]
    InvalidOptions(String),
    /// The work was still unfinished when `wait_timeout` ran out.
    #[error(transparent)]
    Timeout(#[from] FlowsWaitTimeoutError),
    /// The create call or a status check failed.
    #[error(transparent)]
    Api(#[from] ApiError),
}

impl ApiClient {
    /// `create_and_wait` helpers for the flows endpoints: [`image`](Flows::image),
    /// [`video`](Flows::video), [`text_to_speech`](Flows::text_to_speech) and
    /// [`runs`](Flows::runs) (template runs).
    pub fn flows(&self) -> Flows<'_> {
        Flows { client: self }
    }
}

/// Entry point for the flows `create_and_wait` helpers. Returned by [`ApiClient::flows`].
#[derive(Clone, Copy)]
pub struct Flows<'a> {
    client: &'a ApiClient,
}

impl<'a> Flows<'a> {
    /// Image generations (`/v1/flows/image`), the same endpoints as `client.image`.
    pub fn image(&self) -> MediaFlow<'a, ImageGenerationRequest> {
        MediaFlow::new(
            self.client,
            "/v1/flows/image",
            ConfigManager::get_image_config,
        )
    }

    /// Video generations (`/v1/flows/video`), the same endpoints as `client.video`.
    pub fn video(&self) -> MediaFlow<'a, VideoGenerationRequest> {
        MediaFlow::new(
            self.client,
            "/v1/flows/video",
            ConfigManager::get_video_config,
        )
    }

    /// Text-to-speech generations (`/v1/flows/text-to-speech`), the same endpoints as
    /// `client.text_to_speech.create` and `client.text_to_speech.get`.
    pub fn text_to_speech(&self) -> MediaFlow<'a, TextToSpeechGenerationRequest> {
        MediaFlow::new(
            self.client,
            "/v1/flows/text-to-speech",
            ConfigManager::get_text_to_speech_config,
        )
    }

    /// Template runs (`/v1/flows/templates/{template_id}/runs`), the same endpoints as
    /// `client.runs`.
    pub fn runs(&self) -> TemplateRunsFlow<'a> {
        TemplateRunsFlow {
            client: self.client,
        }
    }
}

/// `create_and_wait` for one media generation endpoint. `B` is that endpoint's request body.
pub struct MediaFlow<'a, B> {
    client: &'a ApiClient,
    path: &'static str,
    config: fn(&ConfigManager) -> ClientConfig,
    body: PhantomData<fn(&B)>,
}

impl<'a, B: Serialize> MediaFlow<'a, B> {
    fn new(
        client: &'a ApiClient,
        path: &'static str,
        config: fn(&ConfigManager) -> ClientConfig,
    ) -> Self {
        Self {
            client,
            path,
            config,
            body: PhantomData,
        }
    }

    /// Start a generation and wait until it is `completed` or `failed`, then return the final
    /// generation.
    ///
    /// Between status checks this waits as long as the server's `Retry-After` header suggests,
    /// never less than `wait.min_poll_interval`, plus up to 20% jitter. This is meant for scripts
    /// and quickstarts: in production, set `webhook` on the request and handle the webhook event
    /// instead of holding a process open.
    ///
    /// The result is always [`MediaGenerationResponse::Completed`] or
    /// [`MediaGenerationResponse::Failed`]. A failed generation is returned, not raised, so check
    /// it with `is_completed` or `as_failed`.
    ///
    /// `options` applies to the create call and to every status check, except
    /// `additional_body_params`, which only the create call gets. `timeout_seconds` and
    /// `max_retries` apply to each request on its own; `wait.wait_timeout` bounds the whole wait.
    pub async fn create_and_wait(
        &self,
        body: &B,
        options: Option<RequestOptions>,
        wait: FlowsWaitOptions,
    ) -> Result<MediaGenerationResponse, FlowsWaitError> {
        self.create_and_wait_with(body, options, wait, &RealPacer::start())
            .await
    }

    async fn create_and_wait_with(
        &self,
        body: &B,
        options: Option<RequestOptions>,
        wait: FlowsWaitOptions,
        pacer: &dyn Pacer,
    ) -> Result<MediaGenerationResponse, FlowsWaitError> {
        wait.validate()?;
        let config = (self.config)(&self.client.config);
        let http_client = transport(&config)?;
        let (created, headers): (MediaGenerationCreateResponse, HeaderMap) = send(
            &http_client,
            &config,
            http_client.post(self.path),
            Some(body),
            options.as_ref(),
        )
        .await?;
        let poll_options = poll_request_options(options);
        let generation_id = created.id.clone();
        let url = format!("{}/{}", self.path, generation_id);
        let first_status = serde_json::to_value(&created.status)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned))
            .unwrap_or_default();
        poll::wait_until_terminal::<MediaGenerationResponse, _, _>(
            poll::Target {
                id: generation_id,
                template_id: None,
            },
            first_status,
            None,
            headers,
            || {
                let config = (self.config)(&self.client.config);
                http_client.set_base_url(config.base_url.clone());
                let request = http_client.get(&url);
                let options = poll_options.as_ref();
                let http_client = &http_client;
                async move {
                    send::<(), MediaGenerationResponse>(
                        http_client,
                        &config,
                        request,
                        None,
                        options,
                    )
                    .await
                }
            },
            &wait,
            pacer,
        )
        .await
    }
}

/// `create_and_wait` for template runs.
pub struct TemplateRunsFlow<'a> {
    client: &'a ApiClient,
}

impl<'a> TemplateRunsFlow<'a> {
    /// Start a run of a flows template and wait until its `status` is `completed` or `failed`,
    /// then return the final run.
    ///
    /// Between status checks this waits as long as the server's `Retry-After` header suggests,
    /// never less than `wait.min_poll_interval`, plus up to 20% jitter. This is meant for scripts
    /// and quickstarts: in production, set `webhook` on the request and handle the
    /// `flows_template_run` webhook instead of holding a process open.
    ///
    /// A failed run is returned, not raised; check `status` on the result and on each output.
    /// On timeout the error carries both the run `id` and `template_id`, for `client.runs.get`.
    ///
    /// `options` applies to the create call and to every status check, except
    /// `additional_body_params`, which only the create call gets.
    pub async fn create_and_wait(
        &self,
        template_id: &str,
        body: &TemplateRunCreateRequest,
        options: Option<RequestOptions>,
        wait: FlowsWaitOptions,
    ) -> Result<TemplateRunResponse, FlowsWaitError> {
        self.create_and_wait_with(template_id, body, options, wait, &RealPacer::start())
            .await
    }

    async fn create_and_wait_with(
        &self,
        template_id: &str,
        body: &TemplateRunCreateRequest,
        options: Option<RequestOptions>,
        wait: FlowsWaitOptions,
        pacer: &dyn Pacer,
    ) -> Result<TemplateRunResponse, FlowsWaitError> {
        wait.validate()?;
        let config = self.client.config.get_runs_config();
        let http_client = transport(&config)?;
        let (created, headers): (TemplateRunResponse, HeaderMap) = send(
            &http_client,
            &config,
            http_client.post(format!("/v1/flows/templates/{}/runs", template_id)),
            Some(body),
            options.as_ref(),
        )
        .await?;
        let poll_options = poll_request_options(options);
        let url = format!("/v1/flows/templates/{}/runs/{}", template_id, created.id);
        poll::wait_until_terminal::<TemplateRunResponse, _, _>(
            poll::Target {
                id: created.id.clone(),
                template_id: Some(template_id.to_owned()),
            },
            created.status(),
            Some(created),
            headers,
            || {
                let config = self.client.config.get_runs_config();
                http_client.set_base_url(config.base_url.clone());
                let request = http_client.get(&url);
                let options = poll_options.as_ref();
                let http_client = &http_client;
                async move {
                    send::<(), TemplateRunResponse>(http_client, &config, request, None, options)
                        .await
                }
            },
            &wait,
            pacer,
        )
        .await
    }
}

impl poll::Polled for MediaGenerationResponse {
    fn status(&self) -> String {
        match self {
            Self::Completed { .. } => "completed".to_owned(),
            Self::Failed { .. } => "failed".to_owned(),
            Self::Generating { .. } => "generating".to_owned(),
            Self::Pending { .. } => "pending".to_owned(),
            other => serde_json::to_value(other)
                .ok()
                .and_then(|v| v.get("status").and_then(|s| s.as_str()).map(str::to_owned))
                .unwrap_or_default(),
        }
    }
}

impl poll::Polled for TemplateRunResponse {
    fn status(&self) -> String {
        match &self.status {
            TemplateRunStatus::Pending => "pending".to_owned(),
            TemplateRunStatus::Generating => "generating".to_owned(),
            TemplateRunStatus::Completed => "completed".to_owned(),
            TemplateRunStatus::Failed => "failed".to_owned(),
            TemplateRunStatus::__Unknown(status) => status.clone(),
        }
    }
}

/// A transport built from the same configuration that supplies the credentials, so requests never
/// carry the root client's API key or custom headers to another sub-client's base URL. Each status
/// check re-reads the base URL with the credentials, so `set_base_url` during a wait applies to both.
fn transport(config: &ClientConfig) -> Result<HttpClient, ApiError> {
    HttpClient::new(config.clone())
}

/// The caller's options for the status checks: everything except body parameters, which only the
/// create takes.
fn poll_request_options(options: Option<RequestOptions>) -> Option<RequestOptions> {
    options.map(|mut options| {
        options.additional_body_params.clear();
        options
    })
}

/// Sends one request the way the generated `create` and `get` methods do (per-request headers,
/// query parameters, timeout and auth overrides, the client's custom headers and retries, the same
/// error mapping) but also returns the response headers.
async fn send<B: Serialize, T: DeserializeOwned>(
    http_client: &HttpClient,
    config: &ClientConfig,
    mut request: reqwest::RequestBuilder,
    body: Option<&B>,
    options: Option<&RequestOptions>,
) -> Result<(T, HeaderMap), ApiError> {
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
    let config = match options {
        Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
            let mut overridden = config.clone();
            if let Some(api_key) = &opts.api_key {
                overridden.api_key = Some(api_key.clone());
            }
            overridden
        }
        _ => config.clone(),
    };
    for (key, value) in &config.custom_headers {
        request = request.header(key.as_str(), value.as_str());
    }
    request = crate::core::auth::apply_auth(&config, request, true);

    let response = http_client.send_with_retry(request, options).await?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(ApiError::from_response(status, Some(body.as_str())));
    }
    let headers = response.headers().clone();
    let result: T = response.json().await.map_err(|err| ApiError::DecodeError {
        status,
        message: format!("{err}"),
    })?;
    Ok((result, headers))
}
