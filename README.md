# Elevenlabs Rust Library

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2Felevenlabs%2Felevenlabs-rust)
[![crates.io shield](https://img.shields.io/crates/v/elevenlabs-rust-sdk)](https://crates.io/crates/elevenlabs-rust-sdk)

The Elevenlabs Rust library provides convenient access to the Elevenlabs APIs from Rust.

## Table of Contents

- [Installation](#installation)
- [Reference](#reference)
- [Usage](#usage)
- [Environments](#environments)
- [Errors](#errors)
- [Request Types](#request-types)
- [Websockets](#websockets)
- [Advanced](#advanced)
  - [Configuration](#configuration)
  - [Retries](#retries)
  - [Timeouts](#timeouts)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
  - [Custom Client](#custom-client)
- [Contributing](#contributing)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs-rust-sdk = "1.0.0"
```

Or install via cargo:

```sh
cargo add elevenlabs-rust-sdk
```

## Reference

A full reference for this library is available [here](./reference.md).

## Usage

Instantiate and use the client with the following:

```rust
use elevenlabs_rust_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig::default();
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                ..Default::default()
            },
            None,
        )
        .await;
}
```

## Environments

This SDK allows you to configure different environments for API requests.

```rust
let config = ClientConfig::builder()
    .environment(Environment::Default)
    .build()
    .expect("Failed to build config");
let client = ApiClient::new(config).expect("Failed to build client");
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                ..Default::default()
            },
            None,
        ).await {
    Ok(response) => {
        println!("Success: {:?}", response);
    },
    Err(ApiError::Http { status, message }) => {
        println!("API Error {}: {:?}", status, message);
    },
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Request Types

The SDK exports all request types as Rust structs. Simply import them from the crate to access them:

```rust
use elevenlabs-rust-sdk::prelude::{*};

let request = DownloadHistoryRequest {
    ...
};
```

## Websockets

The SDK supports WebSocket connections for real-time communication. Use the generated channel clients to connect, send, and receive messages.

```rust
use elevenlabs_rust_sdk::prelude::*;
use elevenlabs_rust_sdk::api::websocket::TextToDialogueMultiContextConnectOptions;

let client = ApiClient::new(ClientConfig {
    api_key: Some("YOUR_API_KEY".to_string()),
    ..Default::default()
})
.expect("Failed to create client");

// Connect to the WebSocket
let mut text_to_dialogue_multi_context = client.text_to_dialogue_multi_context.realtime(&TextToDialogueMultiContextConnectOptions::default()).await.expect("Failed to connect");

// Iterate over messages as they arrive
while let Some(Ok(message)) = text_to_dialogue_multi_context.recv().await {
    println!("{:?}", message);
}

// Send a message
text_to_dialogue_multi_context.send_publish(&TextToDialogueMultiClientMessage { /* fields */ }).await.expect("Failed to send");

// Close the connection when done
text_to_dialogue_multi_context.close().await.expect("Failed to close");
```

## Advanced

### Configuration

The client takes a `ClientConfig`, which you can build either way — pick whichever reads better
for what you are setting.

A struct literal, when you are setting one or two fields. It cannot fail, so there is nothing to
unwrap:

```rust
let config = ClientConfig {
    api_key: Some("YOUR_API_KEY".to_string()),
    ..Default::default()
};
let client = ApiClient::new(config).expect("Failed to build client");
```

The builder, which reads better for a longer chain and offers `.environment(…)` as a shorthand for
the base URL:

```rust
let config = ClientConfig::builder()
    .api_key("YOUR_API_KEY")
    .build()
    .expect("Failed to build config");
let client = ApiClient::new(config).expect("Failed to build client");
```

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 3).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

Use the `max_retries` method to configure this behavior.

```rust
let response = client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                ..Default::default()
            },
            Some(elevenlabs-rust-sdk::core::RequestOptions::new().max_retries(3)),
        ).await?;
```

### Timeouts

The SDK defaults to a 240 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                ..Default::default()
            },
            Some(elevenlabs-rust-sdk::core::RequestOptions::new().timeout_seconds(30)),
        ).await?;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                ..Default::default()
            },
            Some(elevenlabs-rust-sdk::core::RequestOptions::new().additional_header("X-Custom-Header", "custom-value").additional_header("X-Another-Header", "another-value")),
        ).await?;
```

### Additional Query String Parameters

You can add custom query parameters to requests using `RequestOptions`.

```rust
let response = client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                ..Default::default()
            },
            Some(elevenlabs-rust-sdk::core::RequestOptions::new().additional_query_param("filter", "active").additional_query_param("sort", "desc")),
        ).await?;
```

### Custom Client

The SDK builds its own `reqwest` client by default, but you can supply your own through
`ClientConfig.reqwest_client` (or `ApiClientBuilder::reqwest_client`) when you need control over the
transport — custom root certificates, client certificates, proxies or connection tuning. The supplied
client is used as-is; authentication, custom headers and retries are still applied by the SDK.

```rust
use elevenlabs-rust-sdk::prelude::*;

let certificate = reqwest::Certificate::from_pem(&std::fs::read("ca.pem")?)?;
let reqwest_client = reqwest::Client::builder()
    .add_root_certificate(certificate)
    .build()
    .expect("Failed to build reqwest client");
let config = ClientConfig {
    reqwest_client: Some(reqwest_client),
    ..Default::default()
};
let client = ApiClient::new(config).expect("Failed to build client");
```

## Contributing

While we value open-source contributions to this SDK, this library is generated programmatically.
Additions made directly to this library would have to be moved over to our generation code,
otherwise they would be overwritten upon the next generated release. Feel free to open a PR as
a proof of concept, but know that we will not be able to merge it as-is. We suggest opening
an issue first to discuss with us!

On the other hand, contributions to the README are always very welcome!
