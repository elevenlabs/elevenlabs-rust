use crate::config::ClientConfig;
use crate::core::RequestOptions;
use futures::stream::{Stream, StreamExt};
use reqwest::{Client, RequestBuilder, Response};
use std::pin::Pin;
use std::sync::RwLock;

pub struct ByteStream {
    content_length: Option<u64>,
    inner: Pin<Box<dyn Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send>>,
}

impl ByteStream {
    pub(crate) fn new(response: Response) -> Self {
        let content_length = response.content_length();
        Self {
            content_length,
            inner: Box::pin(response.bytes_stream()),
        }
    }

    /// `Content-Length` if the server sent one. `None` for a chunked response, which is not the
    /// same as a zero-length body.
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// The next chunk, or `None` once the stream is exhausted.
    pub async fn try_next(&mut self) -> Result<Option<bytes::Bytes>, crate::error::ApiError> {
        match self.inner.next().await {
            Some(Ok(bytes)) => Ok(Some(bytes)),
            Some(Err(source)) => Err(crate::error::ApiError::Network(source)),
            None => Ok(None),
        }
    }

    /// Buffer the whole body. Prefer `try_next()` for anything large - buffering here is exactly
    /// what returning a stream instead of a `Vec<u8>` exists to let the caller opt out of.
    pub async fn collect(mut self) -> Result<Vec<u8>, crate::error::ApiError> {
        let mut collected = Vec::new();
        while let Some(chunk) = self.try_next().await? {
            collected.extend_from_slice(&chunk);
        }
        Ok(collected)
    }
}

impl std::fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ByteStream")
            .field("content_length", &self.content_length)
            .finish_non_exhaustive()
    }
}

pub struct HttpClient {
    client: Client,
    base_url: RwLock<String>,
    timeout: std::time::Duration,
    apply_timeout: bool,
    max_retries: u32,
}

impl HttpClient {
    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {
        let ClientConfig {
            base_url,
            timeout,
            max_retries,
            user_agent,
            reqwest_client,
            ..
        } = config;
        let (client, apply_timeout) = match reqwest_client {
            Some(client) => (client, false),
            None => {
                let client = Client::builder()
                    .user_agent(&user_agent)
                    .connect_timeout(timeout)
                    .read_timeout(timeout)
                    .build()
                    .map_err(crate::error::ApiError::Network)?;
                (client, true)
            }
        };
        Ok(Self {
            client,
            timeout,
            apply_timeout,
            base_url: RwLock::new(base_url),
            max_retries,
        })
    }

    pub fn set_base_url(&self, base_url: impl Into<String>) {
        if let Ok(mut url) = self.base_url.write() {
            *url = base_url.into();
        }
    }

    fn url(&self, path: &str) -> String {
        let base = self.base_url.read().map(|u| u.clone()).unwrap_or_default();
        format!("{}{}", base.trim_end_matches('/'), path)
    }

    pub fn get(&self, path: impl AsRef<str>) -> RequestBuilder {
        {
            let request = self.client.get(self.url(path.as_ref()));
            if self.apply_timeout {
                request.timeout(self.timeout)
            } else {
                request
            }
        }
    }

    pub fn post(&self, path: impl AsRef<str>) -> RequestBuilder {
        {
            let request = self.client.post(self.url(path.as_ref()));
            if self.apply_timeout {
                request.timeout(self.timeout)
            } else {
                request
            }
        }
    }

    pub fn put(&self, path: impl AsRef<str>) -> RequestBuilder {
        {
            let request = self.client.put(self.url(path.as_ref()));
            if self.apply_timeout {
                request.timeout(self.timeout)
            } else {
                request
            }
        }
    }

    pub fn delete(&self, path: impl AsRef<str>) -> RequestBuilder {
        {
            let request = self.client.delete(self.url(path.as_ref()));
            if self.apply_timeout {
                request.timeout(self.timeout)
            } else {
                request
            }
        }
    }

    pub fn patch(&self, path: impl AsRef<str>) -> RequestBuilder {
        {
            let request = self.client.patch(self.url(path.as_ref()));
            if self.apply_timeout {
                request.timeout(self.timeout)
            } else {
                request
            }
        }
    }

    /// A request for a streamed binary download. Unlike the verb methods it carries no total
    /// deadline, so a body that keeps arriving is never cut off; the client-level read timeout
    /// still bounds the connection, the response headers, and any stall between chunks.
    pub fn stream_request(&self, method: reqwest::Method, path: impl AsRef<str>) -> RequestBuilder {
        self.client.request(method, self.url(path.as_ref()))
    }

    pub async fn send_with_retry(
        &self,
        builder: RequestBuilder,
        options: Option<&RequestOptions>,
    ) -> Result<Response, reqwest::Error> {
        let max_retries = options
            .and_then(|o| o.max_retries)
            .unwrap_or(self.max_retries);
        let max_attempts: u32 = max_retries.saturating_add(1);
        let base_delay_ms: u64 = 150;
        let max_delay_ms: u64 = 5000;
        let backoff_factor: f64 = 2.0;
        let jitter_ms: u64 = 50;
        let max_retry_after_delay_ms: u64 = 60000;

        let request = builder.build()?;
        // Retries are gated on the configured set of HTTP methods (httpMethodsToRetry).
        // Non-idempotent methods (e.g. POST/PATCH) are excluded by default so a request
        // the server already processed is not replayed and its side effect duplicated.
        let is_retryable_method = matches!(
            request.method().as_str(),
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS"
        );

        for attempt in 0..max_attempts {
            let attempt_request = match request.try_clone() {
                Some(r) => r,
                None => return self.client.execute(request).await,
            };

            match self.client.execute(attempt_request).await {
                Ok(response) => {
                    if is_retryable_method
                        && attempt + 1 < max_attempts
                        && matches!(response.status().as_u16(), 408 | 429 | 500..=599)
                    {
                        // A server-directed delay (Retry-After / X-RateLimit-Reset) takes
                        // precedence over the computed exponential backoff when present.
                        let delay =
                            match Self::retry_after_delay(&response, max_retry_after_delay_ms) {
                                Some(header_delay) => header_delay,
                                None => {
                                    let exp_delay = (base_delay_ms as f64
                                        * backoff_factor.powi(attempt as i32))
                                        as u64;
                                    // Jitter from a real (thread-local) RNG so concurrent retries de-correlate.
                                    let jitter = rand::random_range(0..jitter_ms * 2);
                                    exp_delay.saturating_add(jitter).min(max_delay_ms)
                                }
                            };
                        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                        continue;
                    }
                    return Ok(response);
                }
                Err(err) => {
                    // Transient transport failure (timeout, connection reset, DNS lookup):
                    // retry retryable methods with backoff while attempts remain, else propagate.
                    if is_retryable_method && attempt + 1 < max_attempts {
                        let delay = {
                            let exp_delay =
                                (base_delay_ms as f64 * backoff_factor.powi(attempt as i32)) as u64;
                            // Jitter from a real (thread-local) RNG so concurrent retries de-correlate.
                            let jitter = rand::random_range(0..jitter_ms * 2);
                            exp_delay.saturating_add(jitter).min(max_delay_ms)
                        };
                        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                        continue;
                    }
                    return Err(err);
                }
            }
        }

        // Unreachable: max_attempts is always >= 1, so the loop always returns.
        // Kept as the tail expression that gives the function its return type.
        self.client.execute(request).await
    }

    /// Returns the server-directed retry delay (in ms) from rate-limit response headers,
    /// honoring Retry-After (delta-seconds or HTTP-date) and, when absent, X-RateLimit-Reset
    /// (epoch seconds), clamped to `max_cap_ms`. Returns `None` when no usable header is present
    /// so the caller falls back to the computed exponential backoff.
    fn retry_after_delay(response: &Response, max_cap_ms: u64) -> Option<u64> {
        if max_cap_ms == 0 {
            return None;
        }
        let headers = response.headers();
        // retry-after-ms (milliseconds) is a non-standard but finer-grained hint some
        // APIs send (e.g. OpenAI); it takes precedence over the whole-second Retry-After.
        if let Some(ms) = headers
            .get("retry-after-ms")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.trim())
            // Strict delta format (\d+(\.\d+)?) — no sign/exponent, matching
            // Retry-After parsing and the other SDKs so every language agrees.
            .filter(|v| {
                let digits =
                    |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
                match v.split_once('.') {
                    Some((int_part, frac)) => digits(int_part) && digits(frac),
                    None => digits(v),
                }
            })
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|ms| *ms >= 0.0)
        {
            // A huge value parses to f64::INFINITY, which saturates to u64::MAX
            // on the cast and is then clamped to the cap (parity with the other SDKs).
            return Some((ms as u64).min(max_cap_ms));
        }
        let seconds = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(Self::parse_retry_after)
            .or_else(|| {
                // X-RateLimit-Reset is epoch seconds; an already-elapsed window is
                // stale, so fall back to backoff (unlike an elapsed Retry-After).
                headers
                    .get("x-ratelimit-reset")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.trim().parse::<f64>().ok())
                    .filter(|epoch| epoch.is_finite())
                    .and_then(|epoch| {
                        // Float subtraction (like the reference impl) avoids any
                        // integer overflow panic on a malformed/huge epoch value.
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs_f64();
                        let delta = epoch - now;
                        if delta > 0.0 {
                            Some(delta)
                        } else {
                            None
                        }
                    })
            })?;
        // The f64->u64 cast saturates (never panics) and the following .min(cap)
        // clamps it, so an HTTP-date far in the future is bounded, not overflowing.
        let ms = (seconds * 1000.0).max(0.0) as u64;
        Some(ms.min(max_cap_ms))
    }

    /// Parses a Retry-After header value: integer/float delta-seconds, or an HTTP-date
    /// (a past date yields 0). Returns the delay in seconds, or `None` if unparseable.
    fn parse_retry_after(value: &str) -> Option<f64> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return None;
        }
        // Delta-seconds: digits with an optional single fractional part (\d+(\.\d+)?).
        let (int_part, frac_part) = match trimmed.split_once('.') {
            Some((i, f)) => (i, Some(f)),
            None => (trimmed, None),
        };
        let frac_is_digits = match frac_part {
            None => true,
            Some(f) => !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()),
        };
        let is_delta =
            !int_part.is_empty() && int_part.bytes().all(|b| b.is_ascii_digit()) && frac_is_digits;
        if is_delta {
            return trimmed.parse::<f64>().ok().map(|s| s.max(0.0));
        }
        // HTTP-date: seconds until that instant (a past date -> 0).
        match httpdate::parse_http_date(trimmed) {
            Ok(when) => Some(
                when.duration_since(std::time::SystemTime::now())
                    .map(|d| d.as_secs_f64())
                    .unwrap_or(0.0),
            ),
            Err(_) => None,
        }
    }
}
