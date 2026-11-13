pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUploadMusicV1MusicUploadPost {
    /// The audio file to upload.
    #[serde(default)]
    pub file: Vec<u8>,
    /// Whether to generate and return the composition plan for the uploaded song. Pass a model id (`music_v1`, `music_v2` or `music_v2_5`) to control which composition plan format is returned. Passing `true`/`false` is deprecated; `true` defaults to the `music_v1` plan format. Enabling this will increase the latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_composition_plan: Option<ExtractCompositionPlan>,
    /// Whether to transcribe the uploaded song and return word-level timestamps. If True, the response will include words_timestamps but will increase the latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_timestamps: Option<bool>,
    /// Whether to return the visual waveform of the uploaded song.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_waveform_visual: Option<bool>,
}

impl BodyUploadMusicV1MusicUploadPost {
    pub fn builder() -> BodyUploadMusicV1MusicUploadPostBuilder {
        <BodyUploadMusicV1MusicUploadPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUploadMusicV1MusicUploadPostBuilder {
    file: Option<Vec<u8>>,
    extract_composition_plan: Option<ExtractCompositionPlan>,
    with_timestamps: Option<bool>,
    with_waveform_visual: Option<bool>,
}

impl BodyUploadMusicV1MusicUploadPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn extract_composition_plan(mut self, value: ExtractCompositionPlan) -> Self {
        self.extract_composition_plan = Some(value);
        self
    }

    pub fn with_timestamps(mut self, value: bool) -> Self {
        self.with_timestamps = Some(value);
        self
    }

    pub fn with_waveform_visual(mut self, value: bool) -> Self {
        self.with_waveform_visual = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUploadMusicV1MusicUploadPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyUploadMusicV1MusicUploadPostBuilder::file)
    pub fn build(self) -> Result<BodyUploadMusicV1MusicUploadPost, BuildError> {
        Ok(BodyUploadMusicV1MusicUploadPost {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            extract_composition_plan: self.extract_composition_plan,
            with_timestamps: self.with_timestamps,
            with_waveform_visual: self.with_waveform_visual,
        })
    }
}

impl BodyUploadMusicV1MusicUploadPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file)
                .file_name("file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        if let Some(value) = self.extract_composition_plan {
            form = form.text(
                "extract_composition_plan",
                serde_json::to_string(&value).unwrap_or_default(),
            );
        }
        if let Some(value) = self.with_timestamps {
            form = form.text("with_timestamps", value.to_string());
        }
        if let Some(value) = self.with_waveform_visual {
            form = form.text("with_waveform_visual", value.to_string());
        }
        form
    }
}
