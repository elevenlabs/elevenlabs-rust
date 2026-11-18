//! The polling loop behind `create_and_wait`, with the clock, sleep and jitter behind a trait so
//! the tests can run it without waiting.

use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant, SystemTime};

use reqwest::header::HeaderMap;

use super::{FlowsWaitError, FlowsWaitOptions, FlowsWaitTimeoutError};
use crate::error::ApiError;

/// Spreads out clients that started together so their status checks don't arrive in lockstep.
const POLL_JITTER: f64 = 0.2;

/// A response whose `status` says whether the work has finished.
pub(super) trait Polled {
    fn status(&self) -> String;

    fn is_terminal(&self) -> bool {
        matches!(self.status().as_str(), "completed" | "failed")
    }
}

/// Time and randomness for the wait. `RealPacer` in production; a fake in the tests.
pub(super) trait Pacer: Send + Sync {
    /// Time since the wait started (just before the create call).
    fn elapsed(&self) -> Duration;
    /// The wall clock, for `Retry-After` HTTP dates.
    fn now(&self) -> SystemTime;
    /// A random number in `[0, 1)`.
    fn random(&self) -> f64;
    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

pub(super) struct RealPacer {
    started_at: Instant,
}

impl RealPacer {
    pub(super) fn start() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }
}

impl Pacer for RealPacer {
    fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    fn now(&self) -> SystemTime {
        SystemTime::now()
    }

    fn random(&self) -> f64 {
        rand::random::<f64>()
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(tokio::time::sleep(duration))
    }
}

/// What is being waited on, for the timeout error.
pub(super) struct Target {
    pub(super) id: String,
    pub(super) template_id: Option<String>,
}

/// The wait from a `Retry-After` header, given as whole seconds or an HTTP date (RFC 9110), or
/// `None` if absent or unparseable. A date in the past is a wait of zero. Kept here rather than
/// shared with the generated `HttpClient`, whose private helpers can change on any regeneration.
pub(super) fn parse_retry_after(headers: &HeaderMap, now: SystemTime) -> Option<Duration> {
    let value = headers.get("retry-after")?.to_str().ok()?.trim();
    if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
        // All digits, so this only fails on absurd lengths; treat those as "wait forever".
        return Some(
            value
                .parse::<u64>()
                .map_or(Duration::MAX, Duration::from_secs),
        );
    }
    let when = httpdate::parse_http_date(value).ok()?;
    Some(when.duration_since(now).unwrap_or(Duration::ZERO))
}

/// The wait before the next status check: the server's `Retry-After`, never below `min_interval`,
/// plus 0-20% jitter.
pub(super) fn poll_delay(
    headers: &HeaderMap,
    min_interval: Duration,
    pacer: &dyn Pacer,
) -> Duration {
    let base = parse_retry_after(headers, pacer.now())
        .unwrap_or(min_interval)
        .max(min_interval);
    let factor = 1.0 + pacer.random() * POLL_JITTER;
    Duration::try_from_secs_f64(base.as_secs_f64() * factor).unwrap_or(Duration::MAX)
}

/// Polls with `get` until the response is terminal, starting from the create call's status,
/// response (when it has the polled type) and headers.
pub(super) async fn wait_until_terminal<T, G, Fut>(
    target: Target,
    mut status: String,
    mut last: Option<T>,
    mut headers: HeaderMap,
    mut get: G,
    wait: &FlowsWaitOptions,
    pacer: &dyn Pacer,
) -> Result<T, FlowsWaitError>
where
    T: Polled,
    G: FnMut() -> Fut,
    Fut: Future<Output = Result<(T, HeaderMap), ApiError>>,
{
    loop {
        if let Some(response) = last.take() {
            if response.is_terminal() {
                return Ok(response);
            }
        }
        let mut delay = poll_delay(&headers, wait.min_poll_interval, pacer);
        if let Some(wait_timeout) = wait.wait_timeout {
            // The last sleep ends on the deadline; the GET after it takes time, so the next pass
            // returns the timeout.
            match wait_timeout.checked_sub(pacer.elapsed()) {
                Some(remaining) if !remaining.is_zero() => delay = delay.min(remaining),
                _ => {
                    return Err(FlowsWaitTimeoutError {
                        id: target.id,
                        template_id: target.template_id,
                        wait_timeout,
                        last_status: status,
                    }
                    .into())
                }
            }
        }
        pacer.sleep(delay).await;
        let (response, response_headers) = get().await?;
        status = response.status();
        headers = response_headers;
        last = Some(response);
    }
}
