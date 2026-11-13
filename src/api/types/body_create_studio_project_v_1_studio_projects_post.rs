pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCreateStudioProjectV1StudioProjectsPost {
    /// The name of the Studio project, used for identification only.
    #[serde(default)]
    pub name: String,
    /// The voice_id that corresponds to the default voice used for new titles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_title_voice_id: Option<String>,
    /// The voice_id that corresponds to the default voice used for new paragraphs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_paragraph_voice_id: Option<String>,
    /// The ID of the model to be used for this Studio project, you can query GET /v1/models to list all available models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model_id: Option<String>,
    /// An optional URL from which we will extract content to initialize the Studio project. If this is set, 'from_url' and 'from_content' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
    /// An optional .epub, .pdf, .txt or similar file can be provided. If provided, we will initialize the Studio project with its content. If this is set, 'from_url' and 'from_content' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_document: Option<Vec<u8>>,
    ///
    /// An optional content to initialize the Studio project with. If this is set, 'from_url' and 'from_document' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    ///
    /// Example:
    /// [{"name": "Chapter A", "blocks": [{"sub_type": "p", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "A", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "B", "type": "tts_node"}]}, {"sub_type": "h1", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "C", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "D", "type": "tts_node"}]}]}, {"name": "Chapter B", "blocks": [{"sub_type": "p", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "E", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "F", "type": "tts_node"}]}, {"sub_type": "h2", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "G", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "H", "type": "tts_node"}]}]}]
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_content_json: Option<String>,
    /// Output quality of the generated audio. Must be one of:
    /// 'standard' - standard output format, 128kbps with 44.1kHz sample rate.
    /// 'high' - high quality output format, 192kbps with 44.1kHz sample rate and major improvements on our side.
    /// 'ultra' - ultra quality output format, 192kbps with 44.1kHz sample rate and highest improvements on our side.
    /// 'ultra_lossless' - ultra quality output format, 705.6kbps with 44.1kHz sample rate and highest improvements on our side in a fully lossless format.
    /// If not provided, defaults to the highest quality preset available on your subscription tier.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_preset: Option<QualityPresetType>,
    /// An optional name of the author of the Studio project, this will be added as metadata to the mp3 file on Studio project or chapter download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// An optional name of the author of the Studio project, this will be added as metadata to the mp3 file on Studio project or chapter download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// An optional description of the Studio project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An optional list of genres associated with the Studio project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    /// An optional target audience of the Studio project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<BodyCreateStudioProjectV1StudioProjectsPostTargetAudience>,
    /// An optional language of the Studio project. Two-letter language code (ISO 639-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// An optional content type of the Studio project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// An optional original publication date of the Studio project, in the format YYYY-MM-DD or YYYY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_publication_date: Option<String>,
    /// An optional specification of whether this Studio project contains mature content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mature_content: Option<bool>,
    /// An optional ISBN number of the Studio project you want to create, this will be added as metadata to the mp3 file on Studio project or chapter download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn_number: Option<String>,
    /// [Deprecated] When the Studio project is downloaded, should the returned audio have postprocessing in order to make it compliant with audiobook normalized volume requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acx_volume_normalization: Option<bool>,
    /// When the Studio project is downloaded, should the returned audio have postprocessing in order to make it compliant with audiobook normalized volume requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_normalization: Option<bool>,
    /// A list of pronunciation dictionary locators (pronunciation_dictionary_id, version_id) encoded as a list of JSON strings for pronunciation dictionaries to be applied to the text. A list of json encoded strings is required as adding projects may occur through formData as opposed to jsonBody. To specify multiple dictionaries use multiple --form lines in your curl, such as --form 'pronunciation_dictionary_locators="{\"pronunciation_dictionary_id\":\"Vmd4Zor6fplcA7WrINey\",\"version_id\":\"hRPaxjlTdR7wFMhV4w0b\"}"' --form 'pronunciation_dictionary_locators="{\"pronunciation_dictionary_id\":\"JzWtcGQMJ6bnlWwyMo7e\",\"version_id\":\"lbmwxiLu4q6txYxgdZqn\"}"'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<String>>,
    ///
    /// A url that will be called by our service when the Studio project is converted. Request will contain a json blob containing the status of the conversion
    /// Messages:
    /// 1. When project was converted successfully:
    /// {
    /// type: "project_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// conversion_status: "success",
    /// project_snapshot_id: "22m00Tcm4TlvDq8ikMAT",
    /// error_details: None,
    /// }
    /// }
    /// 2. When project conversion failed:
    /// {
    /// type: "project_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// conversion_status: "error",
    /// project_snapshot_id: None,
    /// error_details: "Error details if conversion failed"
    /// }
    /// }
    ///
    /// 3. When chapter was converted successfully:
    /// {
    /// type: "chapter_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// chapter_id: "22m00Tcm4TlvDq8ikMAT",
    /// conversion_status: "success",
    /// chapter_snapshot_id: "23m00Tcm4TlvDq8ikMAV",
    /// error_details: None,
    /// }
    /// }
    /// 4. When chapter conversion failed:
    /// {
    /// type: "chapter_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// chapter_id: "22m00Tcm4TlvDq8ikMAT",
    /// conversion_status: "error",
    /// chapter_snapshot_id: None,
    /// error_details: "Error details if conversion failed"
    /// }
    /// }
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// An optional specification of whether the content of this Studio project is fiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiction: Option<BodyCreateStudioProjectV1StudioProjectsPostFiction>,
    ///
    /// This parameter controls text normalization with four modes: 'auto', 'on', 'apply_english' and 'off'.
    /// When set to 'auto', the system will automatically decide whether to apply text normalization
    /// (e.g., spelling out numbers). With 'on', text normalization will always be applied, while
    /// with 'off', it will be skipped. 'apply_english' is the same as 'on' but will assume that text is in English.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization:
        Option<BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization>,
    /// Whether to auto convert the Studio project to audio or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    /// [Alpha Feature] Whether automatically assign voices to phrases in the create Project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_voices: Option<bool>,
    /// The type of Studio project to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<BodyCreateStudioProjectV1StudioProjectsPostSourceType>,
    /// Optional voice settings overrides for the project, encoded as a list of JSON strings.
    ///
    /// Example:
    /// ["{\"voice_id\": \"21m00Tcm4TlvDq8ikWAM\", \"stability\": 0.7, \"similarity_boost\": 0.8, \"style\": 0.5, \"speed\": 1.0, \"use_speaker_boost\": true}"]
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<Vec<String>>,
    /// If true, creates a corresponding read for direct publishing in draft state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_publishing_read: Option<bool>,
}

impl BodyCreateStudioProjectV1StudioProjectsPost {
    pub fn builder() -> BodyCreateStudioProjectV1StudioProjectsPostBuilder {
        <BodyCreateStudioProjectV1StudioProjectsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateStudioProjectV1StudioProjectsPostBuilder {
    name: Option<String>,
    default_title_voice_id: Option<String>,
    default_paragraph_voice_id: Option<String>,
    default_model_id: Option<String>,
    from_url: Option<String>,
    from_document: Option<Vec<u8>>,
    from_content_json: Option<String>,
    quality_preset: Option<QualityPresetType>,
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    genres: Option<Vec<String>>,
    target_audience: Option<BodyCreateStudioProjectV1StudioProjectsPostTargetAudience>,
    language: Option<String>,
    content_type: Option<String>,
    original_publication_date: Option<String>,
    mature_content: Option<bool>,
    isbn_number: Option<String>,
    acx_volume_normalization: Option<bool>,
    volume_normalization: Option<bool>,
    pronunciation_dictionary_locators: Option<Vec<String>>,
    callback_url: Option<String>,
    fiction: Option<BodyCreateStudioProjectV1StudioProjectsPostFiction>,
    apply_text_normalization: Option<BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization>,
    auto_convert: Option<bool>,
    auto_assign_voices: Option<bool>,
    source_type: Option<BodyCreateStudioProjectV1StudioProjectsPostSourceType>,
    voice_settings: Option<Vec<String>>,
    create_publishing_read: Option<bool>,
}

impl BodyCreateStudioProjectV1StudioProjectsPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_title_voice_id(mut self, value: impl Into<String>) -> Self {
        self.default_title_voice_id = Some(value.into());
        self
    }

    pub fn default_paragraph_voice_id(mut self, value: impl Into<String>) -> Self {
        self.default_paragraph_voice_id = Some(value.into());
        self
    }

    pub fn default_model_id(mut self, value: impl Into<String>) -> Self {
        self.default_model_id = Some(value.into());
        self
    }

    pub fn from_url(mut self, value: impl Into<String>) -> Self {
        self.from_url = Some(value.into());
        self
    }

    pub fn from_document(mut self, value: Vec<u8>) -> Self {
        self.from_document = Some(value);
        self
    }

    pub fn from_content_json(mut self, value: impl Into<String>) -> Self {
        self.from_content_json = Some(value.into());
        self
    }

    pub fn quality_preset(mut self, value: QualityPresetType) -> Self {
        self.quality_preset = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn genres(mut self, value: Vec<String>) -> Self {
        self.genres = Some(value);
        self
    }

    pub fn target_audience(
        mut self,
        value: BodyCreateStudioProjectV1StudioProjectsPostTargetAudience,
    ) -> Self {
        self.target_audience = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn original_publication_date(mut self, value: impl Into<String>) -> Self {
        self.original_publication_date = Some(value.into());
        self
    }

    pub fn mature_content(mut self, value: bool) -> Self {
        self.mature_content = Some(value);
        self
    }

    pub fn isbn_number(mut self, value: impl Into<String>) -> Self {
        self.isbn_number = Some(value.into());
        self
    }

    pub fn acx_volume_normalization(mut self, value: bool) -> Self {
        self.acx_volume_normalization = Some(value);
        self
    }

    pub fn volume_normalization(mut self, value: bool) -> Self {
        self.volume_normalization = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<String>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn callback_url(mut self, value: impl Into<String>) -> Self {
        self.callback_url = Some(value.into());
        self
    }

    pub fn fiction(mut self, value: BodyCreateStudioProjectV1StudioProjectsPostFiction) -> Self {
        self.fiction = Some(value);
        self
    }

    pub fn apply_text_normalization(
        mut self,
        value: BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization,
    ) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn auto_assign_voices(mut self, value: bool) -> Self {
        self.auto_assign_voices = Some(value);
        self
    }

    pub fn source_type(
        mut self,
        value: BodyCreateStudioProjectV1StudioProjectsPostSourceType,
    ) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: Vec<String>) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn create_publishing_read(mut self, value: bool) -> Self {
        self.create_publishing_read = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateStudioProjectV1StudioProjectsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateStudioProjectV1StudioProjectsPostBuilder::name)
    pub fn build(self) -> Result<BodyCreateStudioProjectV1StudioProjectsPost, BuildError> {
        Ok(BodyCreateStudioProjectV1StudioProjectsPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_title_voice_id: self.default_title_voice_id,
            default_paragraph_voice_id: self.default_paragraph_voice_id,
            default_model_id: self.default_model_id,
            from_url: self.from_url,
            from_document: self.from_document,
            from_content_json: self.from_content_json,
            quality_preset: self.quality_preset,
            title: self.title,
            author: self.author,
            description: self.description,
            genres: self.genres,
            target_audience: self.target_audience,
            language: self.language,
            content_type: self.content_type,
            original_publication_date: self.original_publication_date,
            mature_content: self.mature_content,
            isbn_number: self.isbn_number,
            acx_volume_normalization: self.acx_volume_normalization,
            volume_normalization: self.volume_normalization,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            callback_url: self.callback_url,
            fiction: self.fiction,
            apply_text_normalization: self.apply_text_normalization,
            auto_convert: self.auto_convert,
            auto_assign_voices: self.auto_assign_voices,
            source_type: self.source_type,
            voice_settings: self.voice_settings,
            create_publishing_read: self.create_publishing_read,
        })
    }
}

impl BodyCreateStudioProjectV1StudioProjectsPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        if let Some(value) = self.default_title_voice_id {
            form = form.text("default_title_voice_id", value.to_string());
        }
        if let Some(value) = self.default_paragraph_voice_id {
            form = form.text("default_paragraph_voice_id", value.to_string());
        }
        if let Some(value) = self.default_model_id {
            form = form.text("default_model_id", value.to_string());
        }
        if let Some(value) = self.from_url {
            form = form.text("from_url", value.to_string());
        }
        if let Some(value) = self.from_document {
            form = form.part(
                "from_document",
                reqwest::multipart::Part::bytes(value)
                    .file_name("from_document")
                    .mime_str("application/octet-stream")
                    .expect("application/octet-stream is a valid MIME type"),
            );
        }
        if let Some(value) = self.from_content_json {
            form = form.text("from_content_json", value.to_string());
        }
        if let Some(value) = self.quality_preset {
            form = form.text("quality_preset", value.to_string());
        }
        if let Some(value) = self.title {
            form = form.text("title", value.to_string());
        }
        if let Some(value) = self.author {
            form = form.text("author", value.to_string());
        }
        if let Some(value) = self.description {
            form = form.text("description", value.to_string());
        }
        if let Some(value) = self.genres {
            for part_value in value {
                form = form.text("genres", part_value.to_string());
            }
        }
        if let Some(value) = self.target_audience {
            form = form.text("target_audience", value.to_string());
        }
        if let Some(value) = self.language {
            form = form.text("language", value.to_string());
        }
        if let Some(value) = self.content_type {
            form = form.text("content_type", value.to_string());
        }
        if let Some(value) = self.original_publication_date {
            form = form.text("original_publication_date", value.to_string());
        }
        if let Some(value) = self.mature_content {
            form = form.text("mature_content", value.to_string());
        }
        if let Some(value) = self.isbn_number {
            form = form.text("isbn_number", value.to_string());
        }
        if let Some(value) = self.acx_volume_normalization {
            form = form.text("acx_volume_normalization", value.to_string());
        }
        if let Some(value) = self.volume_normalization {
            form = form.text("volume_normalization", value.to_string());
        }
        if let Some(value) = self.pronunciation_dictionary_locators {
            for part_value in value {
                form = form.text("pronunciation_dictionary_locators", part_value.to_string());
            }
        }
        if let Some(value) = self.callback_url {
            form = form.text("callback_url", value.to_string());
        }
        if let Some(value) = self.fiction {
            form = form.text("fiction", value.to_string());
        }
        if let Some(value) = self.apply_text_normalization {
            form = form.text("apply_text_normalization", value.to_string());
        }
        if let Some(value) = self.auto_convert {
            form = form.text("auto_convert", value.to_string());
        }
        if let Some(value) = self.auto_assign_voices {
            form = form.text("auto_assign_voices", value.to_string());
        }
        if let Some(value) = self.source_type {
            form = form.text("source_type", value.to_string());
        }
        if let Some(value) = self.voice_settings {
            for part_value in value {
                form = form.text("voice_settings", part_value.to_string());
            }
        }
        if let Some(value) = self.create_publishing_read {
            form = form.text("create_publishing_read", value.to_string());
        }
        form
    }
}
