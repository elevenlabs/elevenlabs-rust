pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateDubbingProjectV1DubbingProjectPost {
    /// The source media file to dub: an audio or video file of at most 3 GiB. Provide this or `source_url`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    /// Public HTTP(S) URL the source media is fetched from server-side, subject to the same size and format limits as an upload. Provide this or `file`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Optional free-form string (at most 500 characters) to identify the project on your end. Stored and echoed back verbatim; it does not affect the dub.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// BCP-47 language tag of the source media; must be a language the transcription model supports. Any region or script subtag is ignored, since transcription is per-language. Omit to auto-detect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// Dubbing model (`dubbing_v1` or `dubbing_v2`) every language target of this project is dubbed with. Defaults to `dubbing_v2`. Fixed at create time -- the source is prepared for this model, so neither the project nor an individual target can change it later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<BodyCreateDubbingProjectV1DubbingProjectPostModelId>,
    /// Key terms to bias transcription and translation toward (for example, product or brand names). At most 1,000 terms; each term at most 50 characters and 5 words; the characters `<>{}[]` are not allowed. Terms are trimmed and deduplicated. Repeat the field once per term, or pass a single JSON array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
    /// IDs of workspace webhooks to notify as this project progresses -- the alternative to polling, and what we recommend. Each receives a `dubbing_project_ready` or `dubbing_project_failed` event for the project, and a `dubbing_language_completed` or `dubbing_language_failed` event for every language under it; `dubbing_language_completed` carries the output download URLs. At most 3 IDs, each already configured in your workspace -- see [Webhooks](https://elevenlabs.io/docs/eleven-api/resources/webhooks) for how to create one and verify its signature. Repeat the field once per ID, or pass a single JSON array or comma-separated string. Delivery is best-effort and can repeat, so we recommend handling events idempotently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_ids: Option<Vec<String>>,
    /// Optional shortcut: also create a language target in this BCP-47 language, queued to start once the project is ready -- equivalent to creating the project and then creating one language target. Must be one of the [languages the dubbing model supports](https://elevenlabs.io/docs/help-center/product/dubbing/which-languages-are-supported-in-dubbing), and a region-qualified tag must be one of the supported dialects. This is the first language target, so it consumes the project's minimum charge rather than adding to it. Its ID is returned in `language_ids`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    /// Enterprise only. Optional JSON transcript to use instead of transcribing the source: a `{"segments": [...]}` document, at most 20,000 segments and 4 MiB. See [Bring your own transcript](https://elevenlabs.io/docs/eleven-api/guides/how-to/dubbing/bring-your-own-transcript) for the segment fields and their constraints. `source_language` is required whenever a transcript is provided. If any segment carries a `translation`, `target_language` is required and every segment must carry one; those translations seed the target created via `target_language`, which then skips machine translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Vec<u8>>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub transcript_file_name: Option<String>,
}

impl BodyCreateDubbingProjectV1DubbingProjectPost {
    pub fn builder() -> BodyCreateDubbingProjectV1DubbingProjectPostBuilder {
        <BodyCreateDubbingProjectV1DubbingProjectPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateDubbingProjectV1DubbingProjectPostBuilder {
    file: Option<Vec<u8>>,
    source_url: Option<String>,
    reference: Option<String>,
    source_language: Option<String>,
    model_id: Option<BodyCreateDubbingProjectV1DubbingProjectPostModelId>,
    keyterms: Option<Vec<String>>,
    webhook_ids: Option<Vec<String>>,
    target_language: Option<String>,
    transcript: Option<Vec<u8>>,
    file_file_name: Option<String>,
    transcript_file_name: Option<String>,
}

impl BodyCreateDubbingProjectV1DubbingProjectPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: BodyCreateDubbingProjectV1DubbingProjectPostModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn keyterms(mut self, value: Vec<String>) -> Self {
        self.keyterms = Some(value);
        self
    }

    pub fn webhook_ids(mut self, value: Vec<String>) -> Self {
        self.webhook_ids = Some(value);
        self
    }

    pub fn target_language(mut self, value: impl Into<String>) -> Self {
        self.target_language = Some(value.into());
        self
    }

    pub fn transcript(mut self, value: Vec<u8>) -> Self {
        self.transcript = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    pub fn transcript_file_name(mut self, value: impl Into<String>) -> Self {
        self.transcript_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateDubbingProjectV1DubbingProjectPost`].
    pub fn build(self) -> Result<BodyCreateDubbingProjectV1DubbingProjectPost, BuildError> {
        Ok(BodyCreateDubbingProjectV1DubbingProjectPost {
            file: self.file,
            source_url: self.source_url,
            reference: self.reference,
            source_language: self.source_language,
            model_id: self.model_id,
            keyterms: self.keyterms,
            webhook_ids: self.webhook_ids,
            target_language: self.target_language,
            transcript: self.transcript,
            file_file_name: self.file_file_name,
            transcript_file_name: self.transcript_file_name,
        })
    }
}

impl BodyCreateDubbingProjectV1DubbingProjectPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(value) = self.file {
            form = form.part("file", {
                let file_name = self.file_file_name.unwrap_or_else(|| "file".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.source_url {
            form = form.text("source_url", value.to_string());
        }
        if let Some(value) = self.reference {
            form = form.text("reference", value.to_string());
        }
        if let Some(value) = self.source_language {
            form = form.text("source_language", value.to_string());
        }
        if let Some(value) = self.model_id {
            form = form.text(
                "model_id",
                serde_json::to_string(&value).unwrap_or_default(),
            );
        }
        if let Some(value) = self.keyterms {
            for part_value in value {
                form = form.text("keyterms", part_value.to_string());
            }
        }
        if let Some(value) = self.webhook_ids {
            for part_value in value {
                form = form.text("webhook_ids", part_value.to_string());
            }
        }
        if let Some(value) = self.target_language {
            form = form.text("target_language", value.to_string());
        }
        if let Some(value) = self.transcript {
            form = form.part("transcript", {
                let file_name = self
                    .transcript_file_name
                    .unwrap_or_else(|| "transcript".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        form
    }
}
