mod api_key;

use crate::config::ClientConfig;
use reqwest::RequestBuilder;

pub fn apply_auth(
    config: &ClientConfig,
    mut request: RequestBuilder,
    uses_api_key: bool,
) -> RequestBuilder {
    if uses_api_key {
        request = api_key::apply(config, request);
    }
    request
}
