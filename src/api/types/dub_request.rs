pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubRequest {
    /// A list of file paths to audio recordings intended for voice cloning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    /// CSV file containing transcription/translation metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_file: Option<Vec<u8>>,
    /// For use only with csv input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_audio_file: Option<Vec<u8>>,
    /// For use only with csv input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio_file: Option<Vec<u8>>,
    /// Name of the dubbing project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the source video/audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Source language. Expects a valid iso639-1 or iso639-3 language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<String>,
    /// The Target language to dub the content into. Expects a valid iso639-1 or iso639-3 language code.
    #[serde(default)]
    pub target_lang: Option<String>,
    /// [Experimental] An accent to apply when selecting voices from the library and to use to inform translation of the dialect to prefer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_accent: Option<String>,
    /// Number of speakers to use for the dubbing. Set to 0 to automatically detect the number of speakers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub num_speakers: Option<i64>,
    /// Whether to apply watermark to the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,
    /// Start time of the source video/audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub start_time: Option<i64>,
    /// End time of the source video/audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub end_time: Option<i64>,
    /// Whether to use the highest resolution available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_resolution: Option<bool>,
    /// An advanced setting. Whether to drop background audio from the final dub. This can improve dub quality where it's known that audio shouldn't have a background track such as for speeches or monologues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_background_audio: Option<bool>,
    /// [BETA] Whether transcripts should have profanities censored with the words '[censored]'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_profanity_filter: Option<bool>,
    /// Whether to prepare dub for edits in dubbing studio or edits as a dubbing resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_studio: Option<bool>,
    /// Instead of using a voice clone in dubbing, use a similar voice from the ElevenLabs Voice Library. Voices used from the library will contribute towards a workspace's custom voices limit, and if there aren't enough available slots the dub will fail. Using this feature requires the caller to have the 'add_voice_from_voice_library' permission on their workspace to access new voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_voice_cloning: Option<bool>,
    /// The mode in which to run this Dubbing job. Defaults to automatic, use manual if specifically providing a CSV transcript to use. Note that manual mode is experimental and production use is strongly discouraged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DubRequestMode>,
    /// Frames per second to use when parsing a CSV file for dubbing. If not provided, FPS will be inferred from timecodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub csv_fps: Option<f64>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub csv_file_file_name: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub foreground_audio_file_file_name: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub background_audio_file_file_name: Option<String>,
}

impl DubRequest {
    pub fn builder() -> DubRequestBuilder {
        <DubRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubRequestBuilder {
    file: Option<Vec<u8>>,
    csv_file: Option<Vec<u8>>,
    foreground_audio_file: Option<Vec<u8>>,
    background_audio_file: Option<Vec<u8>>,
    name: Option<String>,
    source_url: Option<String>,
    source_lang: Option<String>,
    target_lang: Option<String>,
    target_accent: Option<String>,
    num_speakers: Option<i64>,
    watermark: Option<bool>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    highest_resolution: Option<bool>,
    drop_background_audio: Option<bool>,
    use_profanity_filter: Option<bool>,
    dubbing_studio: Option<bool>,
    disable_voice_cloning: Option<bool>,
    mode: Option<DubRequestMode>,
    csv_fps: Option<f64>,
    file_file_name: Option<String>,
    csv_file_file_name: Option<String>,
    foreground_audio_file_file_name: Option<String>,
    background_audio_file_file_name: Option<String>,
}

impl DubRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn csv_file(mut self, value: Vec<u8>) -> Self {
        self.csv_file = Some(value);
        self
    }

    pub fn foreground_audio_file(mut self, value: Vec<u8>) -> Self {
        self.foreground_audio_file = Some(value);
        self
    }

    pub fn background_audio_file(mut self, value: Vec<u8>) -> Self {
        self.background_audio_file = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn source_lang(mut self, value: impl Into<String>) -> Self {
        self.source_lang = Some(value.into());
        self
    }

    pub fn target_lang(mut self, value: impl Into<String>) -> Self {
        self.target_lang = Some(value.into());
        self
    }

    pub fn target_accent(mut self, value: impl Into<String>) -> Self {
        self.target_accent = Some(value.into());
        self
    }

    pub fn num_speakers(mut self, value: i64) -> Self {
        self.num_speakers = Some(value);
        self
    }

    pub fn watermark(mut self, value: bool) -> Self {
        self.watermark = Some(value);
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn highest_resolution(mut self, value: bool) -> Self {
        self.highest_resolution = Some(value);
        self
    }

    pub fn drop_background_audio(mut self, value: bool) -> Self {
        self.drop_background_audio = Some(value);
        self
    }

    pub fn use_profanity_filter(mut self, value: bool) -> Self {
        self.use_profanity_filter = Some(value);
        self
    }

    pub fn dubbing_studio(mut self, value: bool) -> Self {
        self.dubbing_studio = Some(value);
        self
    }

    pub fn disable_voice_cloning(mut self, value: bool) -> Self {
        self.disable_voice_cloning = Some(value);
        self
    }

    pub fn mode(mut self, value: DubRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn csv_fps(mut self, value: f64) -> Self {
        self.csv_fps = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    pub fn csv_file_file_name(mut self, value: impl Into<String>) -> Self {
        self.csv_file_file_name = Some(value.into());
        self
    }

    pub fn foreground_audio_file_file_name(mut self, value: impl Into<String>) -> Self {
        self.foreground_audio_file_file_name = Some(value.into());
        self
    }

    pub fn background_audio_file_file_name(mut self, value: impl Into<String>) -> Self {
        self.background_audio_file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubRequest`].
    pub fn build(self) -> Result<DubRequest, BuildError> {
        Ok(DubRequest {
            file: self.file,
            csv_file: self.csv_file,
            foreground_audio_file: self.foreground_audio_file,
            background_audio_file: self.background_audio_file,
            name: self.name,
            source_url: self.source_url,
            source_lang: self.source_lang,
            target_lang: self.target_lang,
            target_accent: self.target_accent,
            num_speakers: self.num_speakers,
            watermark: self.watermark,
            start_time: self.start_time,
            end_time: self.end_time,
            highest_resolution: self.highest_resolution,
            drop_background_audio: self.drop_background_audio,
            use_profanity_filter: self.use_profanity_filter,
            dubbing_studio: self.dubbing_studio,
            disable_voice_cloning: self.disable_voice_cloning,
            mode: self.mode,
            csv_fps: self.csv_fps,
            file_file_name: self.file_file_name,
            csv_file_file_name: self.csv_file_file_name,
            foreground_audio_file_file_name: self.foreground_audio_file_file_name,
            background_audio_file_file_name: self.background_audio_file_file_name,
        })
    }
}

impl DubRequest {
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
        if let Some(value) = self.csv_file {
            form = form.part("csv_file", {
                let file_name = self
                    .csv_file_file_name
                    .unwrap_or_else(|| "csv_file".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.foreground_audio_file {
            form = form.part("foreground_audio_file", {
                let file_name = self
                    .foreground_audio_file_file_name
                    .unwrap_or_else(|| "foreground_audio_file".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.background_audio_file {
            form = form.part("background_audio_file", {
                let file_name = self
                    .background_audio_file_file_name
                    .unwrap_or_else(|| "background_audio_file".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.name {
            form = form.text("name", value.to_string());
        }
        if let Some(value) = self.source_url {
            form = form.text("source_url", value.to_string());
        }
        if let Some(value) = self.source_lang {
            form = form.text("source_lang", value.to_string());
        }
        if let Some(value) = self.target_lang {
            form = form.text("target_lang", value.to_string());
        }
        if let Some(value) = self.target_accent {
            form = form.text("target_accent", value.to_string());
        }
        if let Some(value) = self.num_speakers {
            form = form.text("num_speakers", value.to_string());
        }
        if let Some(value) = self.watermark {
            form = form.text("watermark", value.to_string());
        }
        if let Some(value) = self.start_time {
            form = form.text("start_time", value.to_string());
        }
        if let Some(value) = self.end_time {
            form = form.text("end_time", value.to_string());
        }
        if let Some(value) = self.highest_resolution {
            form = form.text("highest_resolution", value.to_string());
        }
        if let Some(value) = self.drop_background_audio {
            form = form.text("drop_background_audio", value.to_string());
        }
        if let Some(value) = self.use_profanity_filter {
            form = form.text("use_profanity_filter", value.to_string());
        }
        if let Some(value) = self.dubbing_studio {
            form = form.text("dubbing_studio", value.to_string());
        }
        if let Some(value) = self.disable_voice_cloning {
            form = form.text("disable_voice_cloning", value.to_string());
        }
        if let Some(value) = self.mode {
            form = form.text("mode", value.to_string());
        }
        if let Some(value) = self.csv_fps {
            form = form.text("csv_fps", value.to_string());
        }
        form
    }
}
