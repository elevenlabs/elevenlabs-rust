# ElevenLabs Rust Library

![LOGO](https://github.com/elevenlabs/elevenlabs-python/assets/12028621/21267d89-5e82-4e7e-9c81-caf30b237683)

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-SDK%20generated%20by%20Fern-brightgreen)](https://buildwithfern.com/?utm_source=fern-elevenlabs/elevenlabs-rust/readme)
[![Discord](https://badgen.net/badge/black/ElevenLabs/icon?icon=discord&label)](https://discord.gg/elevenlabs)
[![Twitter](https://badgen.net/badge/black/elevenlabsio/icon?icon=twitter&label)](https://twitter.com/elevenlabsio)
[![crates.io shield](https://img.shields.io/crates/v/elevenlabs-rust-sdk)](https://crates.io/crates/elevenlabs-rust-sdk)

The official Rust SDK for [ElevenLabs](https://elevenlabs.io/). ElevenLabs brings the most compelling, rich and lifelike voices to creators and developers in just a few lines of code.

## 📖 API & Docs

Check out the [HTTP API documentation](https://elevenlabs.io/docs/api-reference). A full reference for this library is available [here](./reference.md).

## Requirements

This SDK targets the Rust 2021 edition and builds on a recent stable toolchain.

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs-rust-sdk = "1.0.0"
```

Or add it with cargo:

```sh
cargo add elevenlabs-rust-sdk
```

## Usage

### Main Models

1. **Eleven v4** (`eleven_v4`)
    - Highest quality and most expressive speech, with reliable voice cloning across long generations
    - 90+ languages with consistent voice quality across language switches
    - Up to 10,000 characters (~10 minutes of audio) per request
    - Used via the [Text to Dialogue API](https://elevenlabs.io/docs/api-reference/text-to-dialogue/convert)

2. **Eleven v4 Turbo** (`eleven_v4_turbo`)
    - State-of-the-art real-time speech, with a median latency of ~100ms
    - 90+ languages
    - Ideal for voice agents and other latency-sensitive use cases

3. **Eleven Multilingual v2** (`eleven_multilingual_v2`)
    - Excels in stability, language diversity, and accent accuracy
    - The most stable choice for long-form Text to Speech
    - Supports 29 languages

4. **Eleven Flash v2.5** (`eleven_flash_v2_5`)
    - Ultra-low latency and lowest cost
    - Supports 32 languages

For more detailed information about these models and others, visit the [ElevenLabs Models documentation](https://elevenlabs.io/docs/models).

Instantiate and use the client with the following:

```rust
use elevenlabs_rust_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ClientConfig::default() reads the API key from the ELEVENLABS_API_KEY environment variable.
    // Pass it explicitly with ClientConfig::builder().api_key("YOUR_API_KEY").build()? if you prefer.
    let client = ApiClient::new(ClientConfig::default())?;

    let request = TextToSpeechConvertRequest::builder()
        .output_format(ConvertOutputFormatQueryParameter1::Mp3_44100_128)
        .body(
            BodyTextToSpeechFull::builder()
                .text("The first move is what sets everything in motion.")
                .model_id("eleven_multilingual_v2")
                .build()?,
        )
        .build()?;

    let audio = client
        .text_to_speech
        .convert("JBFqnCBsd6RMkjVDRZzb", &request, None)
        .await?;

    // `convert` returns a lazy `ByteStream`. `collect` buffers the whole body in one call.
    std::fs::write("output.mp3", audio.collect().await?)?;
    Ok(())
}
```

### Text to Dialogue (Eleven v4)

Eleven v4 and Eleven v4 Turbo are served through the Text to Dialogue API, which turns a list of speaker turns into a single expressive audio stream.

```rust
let request = TextToDialogueConvertRequest::builder()
    .output_format(ConvertOutputFormatQueryParameter1::Mp3_44100_128)
    .body(
        BodyTextToDialogueMultiVoiceV1TextToDialoguePost::builder()
            .model_id("eleven_v4")
            .inputs(vec![
                DialogueInput::builder()
                    .text("[giggling] Knock knock")
                    .voice_id("JBFqnCBsd6RMkjVDRZzb")
                    .build()?,
                DialogueInput::builder()
                    .text("[curious] Who is there?")
                    .voice_id("Aw4FAjKCGjjNkVhN1Xmq")
                    .build()?,
            ])
            .build()?,
    )
    .build()?;

let audio = client.text_to_dialogue.convert(&request, None).await?;
std::fs::write("dialogue.mp3", audio.collect().await?)?;
```

## Voices

List all your available voices with `search`.

```rust
let response = client.voices.search(&SearchQueryRequest::default(), None).await?;
for voice in response.voices {
    if let Some(name) = voice.name {
        println!("{name}");
    }
}
```

For information about the structure of the voices output, please refer to the [official ElevenLabs API documentation for Get Voices](https://elevenlabs.io/docs/api-reference/get-voices).

## Streaming

Stream audio in real-time, as it's being generated. `stream` returns a lazy `ByteStream`: the request
is sent on the first poll and each chunk is yielded as it arrives, so playback or a file write can
start before the server has finished.

```rust
use std::io::Write;

let request = StreamRequest::builder()
    .output_format(StreamOutputFormatQueryParameter1::Mp3_44100_128)
    .body(
        BodyTextToSpeechStream::builder()
            .text("This is a... streaming voice.")
            .model_id("eleven_multilingual_v2")
            .build()?,
    )
    .build()?;

let mut audio = client
    .text_to_speech
    .stream("JBFqnCBsd6RMkjVDRZzb", &request, None)
    .await?;

let mut stdout = std::io::stdout();
while let Some(chunk) = audio.try_next().await? {
    // Process each chunk as it arrives.
    stdout.write_all(&chunk)?;
}
```

Call `collect` instead of iterating to buffer the whole body as one `Vec<u8>`.

## Environments

This SDK allows you to configure different environments for API requests. It defaults to the production environment.

```rust
let config = ClientConfig::builder()
    .environment(Environment::ProductionUs)
    .build()?;
let client = ApiClient::new(config)?;
```

Available environments:
- `Environment::Production`
- `Environment::ProductionUs`
- `Environment::ProductionEu`
- `Environment::ProductionIndia`
- `Environment::ProductionSingapore`

## Errors

When the API returns a non-success status code (4xx or 5xx response), the SDK returns an `ApiError`.
A 400 is surfaced as `ApiError::BadRequestError` and a 422 as `ApiError::UnprocessableEntityError`
with the parsed response body; every other status arrives as `ApiError::Http { status, message }`.

```rust
match client.text_to_speech.convert("JBFqnCBsd6RMkjVDRZzb", &request, None).await {
    Ok(audio) => {
        std::fs::write("output.mp3", audio.collect().await?)?;
    }
    Err(ApiError::Http { status, message }) => {
        println!("API Error {status}: {message}");
    }
    Err(e) => {
        println!("Other error: {e:?}");
    }
}
```

## Agent tools as code

`agent_tools` builds agent tools (webhook, client and system tools) in code. `build()` validates the
tool and returns the `ToolRequestModel` that `client.tools.create` and `client.tools.update` accept;
`to_json_string()` returns the same config as canonical JSON (sorted keys, 2-space indent), the
format of the CLI's `tool_configs/*.json` files, so tools can be reviewed and stored in git.

```rust
use elevenlabs_rust_sdk::agent_tools::{
    Destination, Header, Schema, SystemTool, Tool, ToolOptions,
};

let get_order = Tool::webhook("get_order", "Look up an order by the number the caller gives")
    .get("https://api.example.com/orders/{order_id}")
    .path_param("order_id", Schema::string().describe("The order number, e.g. A-1042"))
    .query_param("include", Schema::string().constant("items"))
    .header("Authorization", Header::secret("sec_orders_api"))
    .assign("order_status", "order.status", Default::default())
    .options(ToolOptions::new().set("response_timeout_secs", 10).set("pre_tool_speech", "force"));

let refund = Tool::webhook("issue_refund", "Refund an order the caller asked to return")
    .post("https://api.example.com/refunds")
    .body(
        Schema::object()
            .property("order_id", Schema::string().from_variable("order_id"))
            .property(
                "reason",
                Schema::string()
                    .describe("Why the caller wants a refund")
                    .choices(["damaged", "late", "other"]),
            )
            .property("note", Schema::string().describe("Anything else the caller said").optional()),
    );

let transfer = SystemTool::transfer_to_number(Default::default())
    .transfer("Caller asks for billing", Destination::phone("+15551234567"), Default::default())
    .transfer(
        "Caller asks for a manager",
        Destination::sip_variable("manager_sip"),
        ToolOptions::new().set("transfer_type", "sip_refer"),
    );

let created = client.tools.create(&get_order.build()?, None).await?;
std::fs::write("tool_configs/issue_refund.json", refund.to_json_string()?)?;
let built_in_entry = transfer.build_config()?; // for the agent's prompt.built_in_tools
```

- Every literal param needs exactly one value source: `describe` (the LLM fills it in),
  `from_variable`, `constant`, `system_provided` or `omit`. Params are required unless marked
  `optional()`.
- Options use the API field names in snake_case (`response_timeout_secs`, `content_type`,
  `expects_response`, `transfer_type`...). A value can be plain JSON or one of the SDK's generated
  enums.
- `build()` reports every problem at once in a `ToolValidationError` (`err.issues`, each with a
  `code` such as `missing_path_param` and a `path`) instead of failing on the first one.
- `Tool::from_json` loads a tool config (or a `tools.get()` response with `Tool::from_response`) for
  editing: `into_webhook()`, `into_client()` and `into_system()` return the typed tool. Fields the
  builder doesn't model, and tools of types it can't edit, are written back unchanged.

## Webhooks

ElevenLabs signs every webhook it sends with your webhook secret. Use `webhooks::construct_event`
to verify the `ElevenLabs-Signature` header and decode the event. Pass the raw request body, not
a re-serialized one: any change to the bytes breaks verification.

```rust
use elevenlabs_rust_sdk::webhooks::{construct_event, WebhookError};

fn handle_webhook(raw_body: &[u8], signature_header: &str) -> Result<(), WebhookError> {
    let secret = std::env::var("ELEVENLABS_WEBHOOK_SECRET").unwrap_or_default();
    let event = construct_event(raw_body, signature_header, &secret)?;

    if event["type"] == "post_call_transcription" {
        println!("Transcript for agent {}", event["data"]["agent_id"]);
    }
    Ok(())
}
```

`construct_event` returns the event as a `serde_json::Value`. It returns a `WebhookError` if the
header or secret is missing, the signature does not match, or the timestamp is more than 30 minutes
old.

## Flows

Image, video and text-to-speech generations and template runs are asynchronous: `create` returns
straight away with a `pending` status, and `get` reports progress. `create_and_wait` does both and
returns once the work is `completed` or `failed`:

```rust
use elevenlabs_rust_sdk::flows::{FlowsWaitError, FlowsWaitOptions};
use std::time::Duration;

let wait = FlowsWaitOptions::default().wait_timeout(Duration::from_secs(300));
match client.flows().image().create_and_wait(&request, None, wait).await {
    Ok(generation) => match generation.as_completed() {
        Some(completed) => println!("{}", completed.content_url),
        None => println!("Generation failed: {:?}", generation.as_failed()),
    },
    Err(FlowsWaitError::Timeout(timeout)) => {
        // Still running on the server: resume with client.image.get(&timeout.id, None).
        println!("{timeout}");
    }
    Err(e) => return Err(e.into()),
}
```

`client.flows().video()` and `client.flows().text_to_speech()` work the same way, and
`client.flows().runs().create_and_wait(template_id, &request, None, wait)` waits for a template
run. A failed generation or run is returned, not raised, so check its status.

Between status checks the SDK waits as long as the server's `Retry-After` header asks, never less
than `min_poll_interval` (3 seconds by default, 0.5 at the least), plus up to 20% jitter.
`wait_timeout` bounds the whole wait and defaults to waiting indefinitely; when it runs out you get
`FlowsWaitError::Timeout`, which carries the generation or run id. Request options apply to every
request, except `additional_body_params`, which only the create call gets.

This is meant for scripts and quickstarts. In production, set `webhook` on the request and handle
the webhook event instead of holding a process open.

## Composing music with metadata

`POST /v1/music/detailed` returns the song's metadata and its audio file in one `multipart/mixed`
response. `MusicDetailed::compose_detailed` sends that request and splits the response for you.

```rust
use elevenlabs_rust_sdk::music_detailed::MusicDetailed;
use elevenlabs_rust_sdk::{BodyComposeMusicWithADetailedResponseV1MusicDetailedPost, ComposeDetailedRequest};

let music = MusicDetailed::new(&client);
let song = music
    .compose_detailed(
        &ComposeDetailedRequest {
            body: Some(BodyComposeMusicWithADetailedResponseV1MusicDetailedPost {
                prompt: Some("Epic orchestral music with dramatic strings and powerful brass".into()),
                music_length_ms: Some(60_000),
                ..Default::default()
            }),
            ..Default::default()
        },
        None,
    )
    .await?;

println!("{}", song.json["song_metadata"]["title"]);
std::fs::write(&song.filename, &song.audio)?;
```

- `json` is the metadata part as the API sends it: `composition_plan`, `song_metadata` and
  `words_timestamps`.
- `audio` is the audio file's bytes and `filename` the name the API suggests for it.
- `song_id` is set when the request has `store_for_inpainting` enabled.
- Use this rather than `client.music.compose_detailed`, which can't decode the multipart response.

## WebSockets

The SDK supports WebSocket connections for real-time communication. Use the generated channel clients
to connect, send, and receive messages.

```rust
use elevenlabs_rust_sdk::prelude::*;
use elevenlabs_rust_sdk::api::websocket::TextToDialogueMultiContextConnectOptions;

let client = ApiClient::new(ClientConfig::default())?;

// Connect to the WebSocket.
let mut socket = client
    .text_to_dialogue_multi_context
    .realtime(&TextToDialogueMultiContextConnectOptions::default())
    .await?;

// Iterate over messages as they arrive.
while let Some(Ok(message)) = socket.recv().await {
    println!("{message:?}");
}

// Send a message.
socket.send_publish(&TextToDialogueMultiClientMessage { /* fields */ }).await?;

// Close the connection when done.
socket.close().await?;
```

The SDK exposes one channel client per real-time endpoint:

- `client.text_to_speech.realtime`
- `client.text_to_speech_multi_context.realtime`
- `client.text_to_dialogue.realtime`
- `client.text_to_dialogue_multi_context.realtime`
- `client.speech_to_text.realtime`
- `client.translation.realtime`
- `client.dubbing.live`
- `client.speech_engine.upstream`

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
let client = ApiClient::new(config)?;
```

The builder, which reads better for a longer chain and offers `.environment(…)` as a shorthand for
the base URL:

```rust
let config = ClientConfig::builder()
    .api_key("YOUR_API_KEY")
    .build()?;
let client = ApiClient::new(config)?;
```

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 3).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

Pass `RequestOptions` per call to override the retry limit:

```rust
let audio = client
    .text_to_speech
    .convert(
        "JBFqnCBsd6RMkjVDRZzb",
        &request,
        Some(RequestOptions::new().max_retries(3)),
    )
    .await?;
```

### Timeouts

The SDK defaults to a 240 second timeout. Use `timeout_seconds` to configure this behavior per call.

```rust
let audio = client
    .text_to_speech
    .convert(
        "JBFqnCBsd6RMkjVDRZzb",
        &request,
        Some(RequestOptions::new().timeout_seconds(30)),
    )
    .await?;
```

### Additional Headers

You can add custom headers to a single request using `RequestOptions`.

```rust
let audio = client
    .text_to_speech
    .convert(
        "JBFqnCBsd6RMkjVDRZzb",
        &request,
        Some(
            RequestOptions::new()
                .additional_header("X-Custom-Header", "custom-value")
                .additional_header("X-Another-Header", "another-value"),
        ),
    )
    .await?;
```

### Additional Query String Parameters

You can add custom query parameters to a single request using `RequestOptions`.

```rust
let audio = client
    .text_to_speech
    .convert(
        "JBFqnCBsd6RMkjVDRZzb",
        &request,
        Some(
            RequestOptions::new()
                .additional_query_param("filter", "active")
                .additional_query_param("sort", "desc"),
        ),
    )
    .await?;
```

### Authentication

Most calls authenticate with an API key, read from the `ELEVENLABS_API_KEY` environment variable by
default. The client also supports a bearer token, basic auth and OAuth client credentials, set either
on `ClientConfig` or through `ApiClientBuilder`:

```rust
let client = ApiClientBuilder::default()
    .environment(Environment::Production)
    .api_key("YOUR_API_KEY")
    .build()?;
```

## Languages Supported

Explore [all models & languages](https://elevenlabs.io/docs/models).

## Contributing

While we value open-source contributions to this SDK, this library is generated programmatically.
Additions made directly to this library would have to be moved over to our generation code,
otherwise they would be overwritten upon the next generated release. Feel free to open a PR as
a proof of concept, but know that we will not be able to merge it as-is. We suggest opening
an issue first to discuss with us!

On the other hand, contributions to the README are always very welcome!
