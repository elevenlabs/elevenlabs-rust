pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BodySpeechToTextV1SpeechToTextPost {
    /// The ID of the model to use for transcription.
    #[serde(default)]
    pub model_id: String,
    /// The file to transcribe (100ms minimum audio length). All major audio and video formats are supported. Exactly one of the file or cloud_storage_url parameters must be provided. The file size must be less than 5.0GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    /// An ISO-639-1 or ISO-639-3 language_code corresponding to the language of the audio file. Can sometimes improve transcription performance if known beforehand. Defaults to null, in this case the language is predicted automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Additional ISO-639-1 or ISO-639-3 language codes that may be present in the audio, alongside language_code. Improves transcription accuracy for multilingual audio. Supported by scribe_v3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_language_codes: Option<Vec<String>>,
    /// How numbers are formatted in the transcript: 'digits' (e.g. '42') or 'words' (e.g. 'forty-two'). Supported by scribe_v3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbers_format: Option<NumbersFormat>,
    /// Natural-language instruction applied to the final transcript (max 2000 characters). The edited text is returned in 'edited_transcript' alongside the original transcript. Cannot be combined with entity_detection, entity_redaction or use_multi_channel. Usage of this parameter will incur an additional 30% surcharge on the base transcription cost, billed for at least 10 seconds of audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_edit: Option<String>,
    /// Whether to tag audio events like (laughter), (footsteps), etc. in the transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_audio_events: Option<bool>,
    /// The maximum amount of speakers talking in the uploaded file. Can help with predicting who speaks when. The maximum amount of speakers that can be predicted is 32. Defaults to null, in this case the amount of speakers is set to the maximum value the model supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub num_speakers: Option<i64>,
    /// The granularity of the timestamps in the transcription. 'word' provides word-level timestamps and 'character' provides character-level timestamps per word.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps_granularity: Option<BodySpeechToTextV1SpeechToTextPostTimestampsGranularity>,
    /// Whether to annotate which speaker is currently talking in the uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,
    /// Diarization threshold to apply during speaker diarization. A higher value means there will be a lower chance of one speaker being diarized as two different speakers but also a higher chance of two different speakers being diarized as one speaker (less total speakers predicted). A low value means there will be a higher chance of one speaker being diarized as two different speakers but also a lower chance of two different speakers being diarized as one speaker (more total speakers predicted). Can only be set when diarize=True and num_speakers=None. Defaults to None, in which case we will choose a threshold based on the model_id (0.22 usually).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub diarization_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_formats: Option<AdditionalFormats>,
    /// The format of input audio. Options are 'pcm_s16le_16' or 'other' For `pcm_s16le_16`, the input audio must be 16-bit PCM at a 16kHz sample rate, single channel (mono), and little-endian byte order. Latency will be lower than with passing an encoded waveform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<FileFormat>,
    /// The URL of an audio or video file to transcribe. Supports hosted video or audio files, YouTube video URLs, TikTok video URLs, and other video hosting services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Whether to send the transcription result to configured speech-to-text webhooks.  If set the request will return early without the transcription, which will be delivered later via webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<bool>,
    /// Optional specific webhook ID to send the transcription result to. Only valid when webhook is set to true. If not provided, transcription will be sent to all configured speech-to-text webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// Controls the randomness of the transcription output. Accepts values between 0.0 and 2.0, where higher values result in more diverse and less deterministic results. If omitted, we will use a temperature based on the model you selected which is usually 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub temperature: Option<f64>,
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed. Must be an integer between 0 and 2147483647.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub seed: Option<i64>,
    /// Whether the audio file contains multiple channels where each channel contains a single speaker. When enabled, each channel is transcribed independently. By default a separate transcript is returned per channel; set multichannel_output_style='combined' to instead receive a single transcript with all channels merged and sorted by time. Each word in the response includes a 'channel_index' field indicating which channel it was spoken on. A maximum of 5 channels is supported. Each channel is billed independently at the full audio duration, so cost scales linearly with the number of channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_multi_channel: Option<bool>,
    /// Controls the response shape when use_multi_channel is enabled. 'separate' (default) returns one transcript per channel under 'transcripts'. 'combined' merges all channels into a single transcript whose words are sorted by start time, each carrying a 'channel_index' - matching the single-channel response shape. 'combined' requires timestamps (timestamps_granularity must not be 'none') and does not support entity detection or redaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multichannel_output_style: Option<MultichannelOutputStyle>,
    /// Optional metadata to be included in the webhook response. This should be a JSON string representing an object with a maximum depth of 2 levels and maximum size of 16KB. Useful for tracking internal IDs, job references, or other contextual information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_metadata: Option<WebhookMetadata>,
    /// Detect entities in the transcript. Can be 'all' to detect all entities, a single entity type or category string, or a list of entity types/categories. Categories include 'pii', 'phi', 'pci', 'other', 'offensive_language'. When enabled, detected entities will be returned in the 'entities' field with their text, type, and character positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detection: Option<EntityDetection>,
    /// If true, the transcription will not have any filler words, false starts and non-speech sounds. Supported by scribe_v2, scribe_v2_medical and scribe_v3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_verbatim: Option<bool>,
    /// Whether to use the speaker library for identifying known speakers during diarization. When enabled and diarize is true, detected speakers will be matched against registered speakers in the workspace's speaker library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_library: Option<bool>,
    /// Whether to detect speaker roles (agent vs customer). Requires diarize=true. Cannot be used with use_multi_channel=true. When enabled, speaker_id values will be 'agent' and 'customer' instead of 'speaker_0', 'speaker_1', etc. Usage incurs an additional 10% surcharge on base transcription cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_speaker_roles: Option<bool>,
    /// Redact entities from the transcript text. Accepts the same format as entity_detection: 'all', a category ('pii', 'phi'), or specific entity types. Must be a subset of entity_detection. When redaction is enabled, the entities field will not be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_redaction: Option<EntityRedaction>,
    /// How to format redacted entities. 'redacted' replaces with {REDACTED}, 'entity_type' replaces with {ENTITY_TYPE}, 'enumerated_entity_type' replaces with {ENTITY_TYPE_N} where N enumerates each occurrence. Only used when entity_redaction is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_redaction_mode: Option<String>,
    /// A list of keyterms to bias the transcription towards.           The keyterms are words or phrases you want the model to recognise more accurately.           The number of keyterms cannot exceed 1000.           The length of each keyterm must be at most 50 characters.           Keyterms can contain at most 5 words (after normalisation).           For example ["hello", "world", "technical term"].           The following characters are not supported: `<`, `>`, `{`, `}`, `[`, `]`, ``.           Usage of this parameter will incur an additional 20% surcharge on the base transcription cost.           When more than 100 keyterms are provided, a minimum billable duration of 20 seconds applies per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
}

impl BodySpeechToTextV1SpeechToTextPost {
    pub fn builder() -> BodySpeechToTextV1SpeechToTextPostBuilder {
        <BodySpeechToTextV1SpeechToTextPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySpeechToTextV1SpeechToTextPostBuilder {
    model_id: Option<String>,
    file: Option<Vec<u8>>,
    language_code: Option<String>,
    secondary_language_codes: Option<Vec<String>>,
    numbers_format: Option<NumbersFormat>,
    transcript_edit: Option<String>,
    tag_audio_events: Option<bool>,
    num_speakers: Option<i64>,
    timestamps_granularity: Option<BodySpeechToTextV1SpeechToTextPostTimestampsGranularity>,
    diarize: Option<bool>,
    diarization_threshold: Option<f64>,
    additional_formats: Option<AdditionalFormats>,
    file_format: Option<FileFormat>,
    source_url: Option<String>,
    webhook: Option<bool>,
    webhook_id: Option<String>,
    temperature: Option<f64>,
    seed: Option<i64>,
    use_multi_channel: Option<bool>,
    multichannel_output_style: Option<MultichannelOutputStyle>,
    webhook_metadata: Option<WebhookMetadata>,
    entity_detection: Option<EntityDetection>,
    no_verbatim: Option<bool>,
    use_speaker_library: Option<bool>,
    detect_speaker_roles: Option<bool>,
    entity_redaction: Option<EntityRedaction>,
    entity_redaction_mode: Option<String>,
    keyterms: Option<Vec<String>>,
}

impl BodySpeechToTextV1SpeechToTextPostBuilder {
    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn secondary_language_codes(mut self, value: Vec<String>) -> Self {
        self.secondary_language_codes = Some(value);
        self
    }

    pub fn numbers_format(mut self, value: NumbersFormat) -> Self {
        self.numbers_format = Some(value);
        self
    }

    pub fn transcript_edit(mut self, value: impl Into<String>) -> Self {
        self.transcript_edit = Some(value.into());
        self
    }

    pub fn tag_audio_events(mut self, value: bool) -> Self {
        self.tag_audio_events = Some(value);
        self
    }

    pub fn num_speakers(mut self, value: i64) -> Self {
        self.num_speakers = Some(value);
        self
    }

    pub fn timestamps_granularity(
        mut self,
        value: BodySpeechToTextV1SpeechToTextPostTimestampsGranularity,
    ) -> Self {
        self.timestamps_granularity = Some(value);
        self
    }

    pub fn diarize(mut self, value: bool) -> Self {
        self.diarize = Some(value);
        self
    }

    pub fn diarization_threshold(mut self, value: f64) -> Self {
        self.diarization_threshold = Some(value);
        self
    }

    pub fn additional_formats(mut self, value: AdditionalFormats) -> Self {
        self.additional_formats = Some(value);
        self
    }

    pub fn file_format(mut self, value: FileFormat) -> Self {
        self.file_format = Some(value);
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn webhook(mut self, value: bool) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn use_multi_channel(mut self, value: bool) -> Self {
        self.use_multi_channel = Some(value);
        self
    }

    pub fn multichannel_output_style(mut self, value: MultichannelOutputStyle) -> Self {
        self.multichannel_output_style = Some(value);
        self
    }

    pub fn webhook_metadata(mut self, value: WebhookMetadata) -> Self {
        self.webhook_metadata = Some(value);
        self
    }

    pub fn entity_detection(mut self, value: EntityDetection) -> Self {
        self.entity_detection = Some(value);
        self
    }

    pub fn no_verbatim(mut self, value: bool) -> Self {
        self.no_verbatim = Some(value);
        self
    }

    pub fn use_speaker_library(mut self, value: bool) -> Self {
        self.use_speaker_library = Some(value);
        self
    }

    pub fn detect_speaker_roles(mut self, value: bool) -> Self {
        self.detect_speaker_roles = Some(value);
        self
    }

    pub fn entity_redaction(mut self, value: EntityRedaction) -> Self {
        self.entity_redaction = Some(value);
        self
    }

    pub fn entity_redaction_mode(mut self, value: impl Into<String>) -> Self {
        self.entity_redaction_mode = Some(value.into());
        self
    }

    pub fn keyterms(mut self, value: Vec<String>) -> Self {
        self.keyterms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySpeechToTextV1SpeechToTextPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model_id`](BodySpeechToTextV1SpeechToTextPostBuilder::model_id)
    pub fn build(self) -> Result<BodySpeechToTextV1SpeechToTextPost, BuildError> {
        Ok(BodySpeechToTextV1SpeechToTextPost {
            model_id: self
                .model_id
                .ok_or_else(|| BuildError::missing_field("model_id"))?,
            file: self.file,
            language_code: self.language_code,
            secondary_language_codes: self.secondary_language_codes,
            numbers_format: self.numbers_format,
            transcript_edit: self.transcript_edit,
            tag_audio_events: self.tag_audio_events,
            num_speakers: self.num_speakers,
            timestamps_granularity: self.timestamps_granularity,
            diarize: self.diarize,
            diarization_threshold: self.diarization_threshold,
            additional_formats: self.additional_formats,
            file_format: self.file_format,
            source_url: self.source_url,
            webhook: self.webhook,
            webhook_id: self.webhook_id,
            temperature: self.temperature,
            seed: self.seed,
            use_multi_channel: self.use_multi_channel,
            multichannel_output_style: self.multichannel_output_style,
            webhook_metadata: self.webhook_metadata,
            entity_detection: self.entity_detection,
            no_verbatim: self.no_verbatim,
            use_speaker_library: self.use_speaker_library,
            detect_speaker_roles: self.detect_speaker_roles,
            entity_redaction: self.entity_redaction,
            entity_redaction_mode: self.entity_redaction_mode,
            keyterms: self.keyterms,
        })
    }
}

impl BodySpeechToTextV1SpeechToTextPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("model_id", self.model_id.to_string());
        if let Some(value) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(value)
                    .file_name("file")
                    .mime_str("application/octet-stream")
                    .expect("application/octet-stream is a valid MIME type"),
            );
        }
        if let Some(value) = self.language_code {
            form = form.text("language_code", value.to_string());
        }
        if let Some(value) = self.secondary_language_codes {
            for part_value in value {
                form = form.text("secondary_language_codes", part_value.to_string());
            }
        }
        if let Some(value) = self.numbers_format {
            form = form.text("numbers_format", value.to_string());
        }
        if let Some(value) = self.transcript_edit {
            form = form.text("transcript_edit", value.to_string());
        }
        if let Some(value) = self.tag_audio_events {
            form = form.text("tag_audio_events", value.to_string());
        }
        if let Some(value) = self.num_speakers {
            form = form.text("num_speakers", value.to_string());
        }
        if let Some(value) = self.timestamps_granularity {
            form = form.text("timestamps_granularity", value.to_string());
        }
        if let Some(value) = self.diarize {
            form = form.text("diarize", value.to_string());
        }
        if let Some(value) = self.diarization_threshold {
            form = form.text("diarization_threshold", value.to_string());
        }
        if let Some(value) = self.additional_formats {
            form = form.text(
                "additional_formats",
                serde_json::to_string(&value).unwrap_or_default(),
            );
        }
        if let Some(value) = self.file_format {
            form = form.text("file_format", value.to_string());
        }
        if let Some(value) = self.source_url {
            form = form.text("source_url", value.to_string());
        }
        if let Some(value) = self.webhook {
            form = form.text("webhook", value.to_string());
        }
        if let Some(value) = self.webhook_id {
            form = form.text("webhook_id", value.to_string());
        }
        if let Some(value) = self.temperature {
            form = form.text("temperature", value.to_string());
        }
        if let Some(value) = self.seed {
            form = form.text("seed", value.to_string());
        }
        if let Some(value) = self.use_multi_channel {
            form = form.text("use_multi_channel", value.to_string());
        }
        if let Some(value) = self.multichannel_output_style {
            form = form.text("multichannel_output_style", value.to_string());
        }
        if let Some(value) = self.webhook_metadata {
            form = form.text(
                "webhook_metadata",
                serde_json::to_string(&value).unwrap_or_default(),
            );
        }
        if let Some(value) = self.entity_detection {
            form = form.text(
                "entity_detection",
                serde_json::to_string(&value).unwrap_or_default(),
            );
        }
        if let Some(value) = self.no_verbatim {
            form = form.text("no_verbatim", value.to_string());
        }
        if let Some(value) = self.use_speaker_library {
            form = form.text("use_speaker_library", value.to_string());
        }
        if let Some(value) = self.detect_speaker_roles {
            form = form.text("detect_speaker_roles", value.to_string());
        }
        if let Some(value) = self.entity_redaction {
            form = form.text(
                "entity_redaction",
                serde_json::to_string(&value).unwrap_or_default(),
            );
        }
        if let Some(value) = self.entity_redaction_mode {
            form = form.text("entity_redaction_mode", value.to_string());
        }
        if let Some(value) = self.keyterms {
            for part_value in value {
                form = form.text("keyterms", part_value.to_string());
            }
        }
        form
    }
}
