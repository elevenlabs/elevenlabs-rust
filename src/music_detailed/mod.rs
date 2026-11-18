//! Music `compose_detailed`, parsed: compose a song and get its metadata and audio file as separate
//! fields.
//!
//! ```no_run
//! # async fn run(client: elevenlabs_rust_sdk::ApiClient) -> Result<(), elevenlabs_rust_sdk::ApiError> {
//! use elevenlabs_rust_sdk::music_detailed::MusicDetailed;
//! use elevenlabs_rust_sdk::{BodyComposeMusicWithADetailedResponseV1MusicDetailedPost, ComposeDetailedRequest};
//!
//! let music = MusicDetailed::new(&client);
//! let song = music
//!     .compose_detailed(
//!         &ComposeDetailedRequest {
//!             body: Some(BodyComposeMusicWithADetailedResponseV1MusicDetailedPost {
//!                 prompt: Some("Epic orchestral music with dramatic strings".into()),
//!                 music_length_ms: Some(60_000),
//!                 ..Default::default()
//!             }),
//!             ..Default::default()
//!         },
//!         None,
//!     )
//!     .await?;
//! println!("{}", song.json["song_metadata"]["title"]);
//! std::fs::write(&song.filename, &song.audio).unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! The API answers `POST /v1/music/detailed` with a `multipart/mixed` body (a JSON part, then the
//! audio). The generated `client.music.compose_detailed` decodes that body as JSON and fails, so this
//! sends the same request (built exactly as the generated method builds it, through the client's
//! shared HTTP client and configuration) and splits the parts.
//!
//! Hand-written (see `.fernignore`). The generated method can't be overridden with a different return
//! type, so this is a separate entry point rather than a method on `client.music`. Mirrors
//! `music.compose_detailed` in the Python SDK.

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::api::types::ComposeDetailedRequest;
use crate::client::ApiClient;
use crate::core::config_manager::ConfigManager;
use crate::core::{HttpClient, RequestOptions};
use crate::error::ApiError;

/// The file name used when the audio part doesn't suggest one.
pub const DEFAULT_FILENAME: &str = "generated_music.mp3";

/// A composed song from [`MusicDetailed::compose_detailed`]: the metadata the API returns with it, the
/// audio file, and its name.
#[derive(Debug, Clone, PartialEq)]
pub struct ComposeDetailedResponse {
    /// The JSON metadata part as the API sent it (snake_case keys, always an object):
    /// `composition_plan`, `song_metadata` (title, description, genres, languages, is_explicit),
    /// `words_timestamps` (null unless `with_timestamps` is set) and, with `with_waveform_visual`,
    /// `waveform_visual`.
    pub json: Value,
    /// The audio file's bytes, in the requested output format.
    pub audio: Vec<u8>,
    /// The file name the API suggests for the audio, or [`DEFAULT_FILENAME`].
    pub filename: String,
    /// The `song-id` response header, sent when `store_for_inpainting` is set.
    pub song_id: Option<String>,
}

/// Compose a song and get its metadata and audio file as separate fields. See the module docs.
#[derive(Clone)]
pub struct MusicDetailed {
    http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl MusicDetailed {
    /// Uses the root client's shared HTTP client and configuration (base URL, credentials, retries,
    /// timeout), as `client.music` does.
    pub fn new(client: &ApiClient) -> Self {
        Self {
            http_client: client.music.http_client.clone(),
            config: client.config.clone(),
        }
    }

    /// Compose a song from a prompt or a composition plan, and return its metadata and audio.
    ///
    /// Takes the same arguments as the generated `client.music.compose_detailed`. An error status is
    /// returned as the generated method returns it; a body that isn't a multipart body with a JSON
    /// part and an audio part is an [`ApiError::DecodeError`].
    pub async fn compose_detailed(
        &self,
        request: &ComposeDetailedRequest,
        options: Option<RequestOptions>,
    ) -> Result<ComposeDetailedResponse, ApiError> {
        // Built exactly as the generated `MusicClient::compose_detailed` builds it.
        let output_format = request.output_format.clone();
        let enable_logging = request.enable_logging;
        let body = request.body.clone();
        let url = "/v1/music/detailed";

        let mut request = self.http_client.post(url);
        if let Some(val) = &output_format {
            request = request.query(&[("output_format", &val.to_string())]);
        }
        if let Some(val) = &enable_logging {
            request = request.query(&[("enable_logging", &val.to_string())]);
        }
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        if let Some(b) = body {
            request = match options.as_ref().and_then(|opts| opts.merge_into_body(&b)) {
                Some(merged) => request.json(&merged),
                None => request.json(&b),
            };
        }
        let config = self.config.get_music_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(ApiError::from_response(status, Some(body.as_str())));
        }

        // Where the generated method decodes JSON: read the bytes and split the parts instead.
        let status = response.status().as_u16();
        let song_id = response
            .headers()
            .get("song-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);
        let boundary = boundary_from(
            response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
        );
        let bytes = response.bytes().await?;
        let parsed = parse_multipart(&bytes, boundary.as_deref())
            .map_err(|message| ApiError::DecodeError { status, message })?;
        Ok(ComposeDetailedResponse {
            json: parsed.json,
            audio: parsed.audio,
            filename: parsed.filename,
            song_id,
        })
    }
}

pub(crate) struct Parsed {
    pub(crate) json: Value,
    pub(crate) audio: Vec<u8>,
    pub(crate) filename: String,
}

struct Part {
    headers: HashMap<String, String>,
    body: Vec<u8>,
    separated: bool,
}

impl Part {
    fn content_type(&self) -> &str {
        self.headers
            .get("content-type")
            .map(String::as_str)
            .unwrap_or("")
    }
}

/// The `boundary` parameter of a `Content-Type` header, unquoted, or `None`.
pub(crate) fn boundary_from(content_type: Option<&str>) -> Option<String> {
    let content_type = content_type?;
    // ASCII lowercasing keeps byte offsets, so an index into it indexes the original too.
    let i = content_type.to_ascii_lowercase().find("boundary=")?;
    let rest = &content_type[i + "boundary=".len()..];
    let value = match rest.strip_prefix('"') {
        Some(quoted) => quoted.split('"').next().unwrap_or(""),
        None => rest
            .split(|c: char| c == ';' || c.is_whitespace())
            .next()
            .unwrap_or(""),
    };
    (!value.is_empty()).then(|| value.to_string())
}

/// Splits the `multipart/mixed` body into its JSON part and its audio part. Works on bytes throughout,
/// so the audio is returned exactly as sent: the line break before each delimiter and the closing
/// `--boundary--` are stripped, never left on the end of the audio. Mirrors `parseMultipart` in the
/// TypeScript SDK.
///
/// `boundary` is the `boundary` parameter of the response's `Content-Type`; without it the delimiter
/// is read from the body's first line. A body that ends before its closing `--boundary--` (truncated)
/// is an error.
pub(crate) fn parse_multipart(body: &[u8], boundary: Option<&str>) -> Result<Parsed, String> {
    let delimiter = delimiter_for(body, boundary)?;
    let delimiter = delimiter.as_slice();

    // The first delimiter starts the body or a line: anything before it is a preamble, which is ignored.
    let mut pos = if body.starts_with(delimiter) {
        0
    } else {
        find(body, &[b"\n", delimiter].concat(), 0)
            .map(|found| found + 1)
            .ok_or_else(|| {
                "Could not find the multipart boundary in the music response".to_string()
            })?
    };
    // The body uses one line ending throughout; read it from the first delimiter line.
    let line_break: &[u8] = if body[pos + delimiter.len()..].starts_with(b"\r\n") {
        b"\r\n"
    } else {
        b"\n"
    };
    let break_then_delimiter = [line_break, delimiter].concat();

    let mut parts = Vec::new();
    loop {
        let after = pos + delimiter.len();
        if body[after..].starts_with(b"--") {
            break; // Closing delimiter.
        }
        let start = if body[after..].starts_with(line_break) {
            after + line_break.len()
        } else {
            after
        };
        // Every part ends at a delimiter, and the body at `--boundary--`; the API always sends it. A
        // body that stops first was cut off, so its last part (the audio, usually) is incomplete.
        let next = find(body, &break_then_delimiter, start).ok_or_else(|| {
            "The music response ended before its closing multipart boundary; it is incomplete"
                .to_string()
        })?;
        parts.push(split_part(&body[start..next], line_break));
        pos = next + line_break.len();
    }

    let mut json_part: Option<&Part> = None;
    let mut audio_part: Option<&Part> = None;
    for part in &parts {
        let is_json = part
            .content_type()
            .to_ascii_lowercase()
            .starts_with("application/json");
        if json_part.is_none() && is_json {
            json_part = Some(part);
        } else if audio_part.is_none() && !is_json {
            audio_part = Some(part);
        }
    }

    let json_part = json_part
        .ok_or_else(|| "Could not find the JSON part in the music response".to_string())?;
    let json: Value = serde_json::from_slice(trim_ascii(&json_part.body))
        .map_err(|e| format!("Could not parse the JSON part of the music response: {e}"))?;
    if !json.is_object() {
        return Err("The JSON part of the music response is not an object".to_string());
    }
    let audio_part = match audio_part {
        Some(part) if part.separated => part,
        _ => return Err("Could not find the audio part in the music response".to_string()),
    };

    Ok(Parsed {
        json,
        audio: audio_part.body.clone(),
        filename: filename(audio_part.headers.get("content-disposition")),
    })
}

fn delimiter_for(body: &[u8], boundary: Option<&str>) -> Result<Vec<u8>, String> {
    if let Some(boundary) = boundary.filter(|b| !b.is_empty()) {
        return Ok(format!("--{boundary}").into_bytes());
    }
    let first_break = body.iter().position(|&b| b == b'\n');
    let delimiter = trim_ascii(&body[..first_break.unwrap_or(body.len())]);
    if !delimiter.starts_with(b"--") || delimiter.len() < 3 {
        return Err("Could not find the multipart boundary in the music response".to_string());
    }
    Ok(delimiter.to_vec())
}

fn split_part(part: &[u8], line_break: &[u8]) -> Part {
    // Prepend a line break so a part with no headers (the blank line straight after the delimiter)
    // is found at offset 0 too.
    let with_break = [line_break, part].concat();
    let double_break = [line_break, line_break].concat();
    match find(&with_break, &double_break, 0) {
        None => Part {
            headers: headers(part, line_break),
            body: Vec::new(),
            separated: false,
        },
        Some(separator) => {
            let header_len = separator.saturating_sub(line_break.len());
            let header_block = &with_break[line_break.len()..line_break.len() + header_len];
            Part {
                headers: headers(header_block, line_break),
                body: with_break[separator + 2 * line_break.len()..].to_vec(),
                separated: true,
            }
        }
    }
}

fn headers(block: &[u8], line_break: &[u8]) -> HashMap<String, String> {
    let text = String::from_utf8_lossy(block);
    let separator = String::from_utf8_lossy(line_break);
    text.split(separator.as_ref())
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            Some((name.trim().to_ascii_lowercase(), value.trim().to_string()))
        })
        .collect()
}

fn filename(disposition: Option<&String>) -> String {
    if let Some(disposition) = disposition {
        // ASCII lowercasing keeps byte offsets, so an index into it indexes the original too.
        if let Some(i) = disposition.to_ascii_lowercase().find("filename=") {
            let rest = &disposition[i + "filename=".len()..];
            let name = match rest.strip_prefix('"') {
                Some(quoted) => quoted.split('"').next().unwrap_or(""),
                None => rest
                    .split(|c: char| c == ';' || c.is_whitespace())
                    .next()
                    .unwrap_or(""),
            };
            if !name.is_empty() {
                return name.to_string();
            }
        }
    }
    DEFAULT_FILENAME.to_string()
}

fn find(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || start > haystack.len() {
        return None;
    }
    haystack[start..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|i| i + start)
}

fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .map_or(start, |i| i + 1);
    &bytes[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audio() -> Vec<u8> {
        (0..=255u8).cycle().take(1024).collect()
    }

    #[test]
    fn finds_the_audio_part_before_the_json_part() {
        let mut body = b"--b\r\nContent-Type: audio/mpeg\r\n\r\n".to_vec();
        body.extend(audio());
        body.extend_from_slice(
            b"\r\n--b\r\nContent-Type: application/json\r\n\r\n{\"a\":1}\r\n--b--\r\n",
        );

        let parsed = parse_multipart(&body, None).unwrap();

        assert_eq!(parsed.json["a"], 1);
        assert_eq!(parsed.audio, audio());
    }

    #[test]
    fn reads_an_unquoted_filename() {
        assert_eq!(
            filename(Some(&"attachment; filename=song.mp3; size=1".to_string())),
            "song.mp3"
        );
        assert_eq!(filename(None), DEFAULT_FILENAME);
    }

    #[test]
    fn reads_the_boundary_parameter() {
        assert_eq!(
            boundary_from(Some("multipart/mixed; boundary=\"abc\"")).as_deref(),
            Some("abc")
        );
        assert_eq!(
            boundary_from(Some("multipart/mixed; boundary=----boundaryf00")).as_deref(),
            Some("----boundaryf00")
        );
        assert_eq!(boundary_from(Some("application/json")), None);
        assert_eq!(boundary_from(None), None);
    }

    #[test]
    fn rejects_a_body_cut_off_before_the_closing_boundary() {
        let mut body = b"--b\r\nContent-Type: application/json\r\n\r\n{}\r\n--b\r\nContent-Type: audio/mpeg\r\n\r\n".to_vec();
        body.extend(audio());
        let error = parse_multipart(&body, Some("b")).err().unwrap();
        assert!(error.contains("incomplete"), "{error}");
        body.extend_from_slice(b"\r\n--");
        assert!(parse_multipart(&body, Some("b")).is_err());
    }

    #[test]
    fn skips_a_preamble() {
        let mut body = b"preamble\r\n--b\r\nContent-Type: application/json\r\n\r\n{\"a\":1}\r\n--b\r\nContent-Type: audio/mpeg\r\n\r\n".to_vec();
        body.extend(audio());
        body.extend_from_slice(b"\r\n--b--\r\n");
        let parsed = parse_multipart(&body, Some("b")).unwrap();
        assert_eq!(parsed.json["a"], 1);
        assert_eq!(parsed.audio, audio());
    }

    #[test]
    fn rejects_a_body_without_a_boundary() {
        assert!(parse_multipart(b"not multipart", None).is_err());
    }
}
