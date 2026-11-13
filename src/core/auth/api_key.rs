use crate::config::ClientConfig;
use reqwest::RequestBuilder;

pub fn apply(config: &ClientConfig, request: RequestBuilder) -> RequestBuilder {
    if let Some(ref key) = config.api_key {
        request.header("xi-api-key", key)
    } else {
        request
    }
}
