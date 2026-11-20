pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionStartedConfig {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub sample_rate: i64,
    pub audio_format: AsrInputFormat,
    #[serde(default)]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps_granularity: Option<ConfigTimestampsGranularity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_commit_strategy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub vad_silence_threshold_secs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub vad_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub min_speech_duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub min_silence_duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub max_tokens_to_recompute: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<ConfigModelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_logging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_timestamps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_language_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_background_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_verbatim: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detection: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_redaction: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_redaction_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub keepalive_interval_ms: Option<i64>,
}

impl SessionStartedConfig {
    pub fn builder() -> SessionStartedConfigBuilder {
        <SessionStartedConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionStartedConfigBuilder {
    sample_rate: Option<i64>,
    audio_format: Option<AsrInputFormat>,
    language_code: Option<String>,
    secondary_languages: Option<Vec<String>>,
    timestamps_granularity: Option<ConfigTimestampsGranularity>,
    vad_commit_strategy: Option<bool>,
    vad_silence_threshold_secs: Option<f64>,
    vad_threshold: Option<f64>,
    min_speech_duration_ms: Option<i64>,
    min_silence_duration_ms: Option<i64>,
    max_tokens_to_recompute: Option<i64>,
    model_id: Option<ConfigModelId>,
    disable_logging: Option<bool>,
    include_timestamps: Option<bool>,
    include_language_detection: Option<bool>,
    filter_background_audio: Option<bool>,
    keyterms: Option<Vec<String>>,
    no_verbatim: Option<bool>,
    entity_detection: Option<Vec<String>>,
    entity_redaction: Option<Vec<String>>,
    entity_redaction_mode: Option<String>,
    keepalive_interval_ms: Option<i64>,
}

impl SessionStartedConfigBuilder {
    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    pub fn audio_format(mut self, value: AsrInputFormat) -> Self {
        self.audio_format = Some(value);
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn secondary_languages(mut self, value: Vec<String>) -> Self {
        self.secondary_languages = Some(value);
        self
    }

    pub fn timestamps_granularity(mut self, value: ConfigTimestampsGranularity) -> Self {
        self.timestamps_granularity = Some(value);
        self
    }

    pub fn vad_commit_strategy(mut self, value: bool) -> Self {
        self.vad_commit_strategy = Some(value);
        self
    }

    pub fn vad_silence_threshold_secs(mut self, value: f64) -> Self {
        self.vad_silence_threshold_secs = Some(value);
        self
    }

    pub fn vad_threshold(mut self, value: f64) -> Self {
        self.vad_threshold = Some(value);
        self
    }

    pub fn min_speech_duration_ms(mut self, value: i64) -> Self {
        self.min_speech_duration_ms = Some(value);
        self
    }

    pub fn min_silence_duration_ms(mut self, value: i64) -> Self {
        self.min_silence_duration_ms = Some(value);
        self
    }

    pub fn max_tokens_to_recompute(mut self, value: i64) -> Self {
        self.max_tokens_to_recompute = Some(value);
        self
    }

    pub fn model_id(mut self, value: ConfigModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn disable_logging(mut self, value: bool) -> Self {
        self.disable_logging = Some(value);
        self
    }

    pub fn include_timestamps(mut self, value: bool) -> Self {
        self.include_timestamps = Some(value);
        self
    }

    pub fn include_language_detection(mut self, value: bool) -> Self {
        self.include_language_detection = Some(value);
        self
    }

    pub fn filter_background_audio(mut self, value: bool) -> Self {
        self.filter_background_audio = Some(value);
        self
    }

    pub fn keyterms(mut self, value: Vec<String>) -> Self {
        self.keyterms = Some(value);
        self
    }

    pub fn no_verbatim(mut self, value: bool) -> Self {
        self.no_verbatim = Some(value);
        self
    }

    pub fn entity_detection(mut self, value: Vec<String>) -> Self {
        self.entity_detection = Some(value);
        self
    }

    pub fn entity_redaction(mut self, value: Vec<String>) -> Self {
        self.entity_redaction = Some(value);
        self
    }

    pub fn entity_redaction_mode(mut self, value: impl Into<String>) -> Self {
        self.entity_redaction_mode = Some(value.into());
        self
    }

    pub fn keepalive_interval_ms(mut self, value: i64) -> Self {
        self.keepalive_interval_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionStartedConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sample_rate`](SessionStartedConfigBuilder::sample_rate)
    /// - [`audio_format`](SessionStartedConfigBuilder::audio_format)
    pub fn build(self) -> Result<SessionStartedConfig, BuildError> {
        Ok(SessionStartedConfig {
            sample_rate: self
                .sample_rate
                .ok_or_else(|| BuildError::missing_field("sample_rate"))?,
            audio_format: self
                .audio_format
                .ok_or_else(|| BuildError::missing_field("audio_format"))?,
            language_code: self.language_code,
            secondary_languages: self.secondary_languages,
            timestamps_granularity: self.timestamps_granularity,
            vad_commit_strategy: self.vad_commit_strategy,
            vad_silence_threshold_secs: self.vad_silence_threshold_secs,
            vad_threshold: self.vad_threshold,
            min_speech_duration_ms: self.min_speech_duration_ms,
            min_silence_duration_ms: self.min_silence_duration_ms,
            max_tokens_to_recompute: self.max_tokens_to_recompute,
            model_id: self.model_id,
            disable_logging: self.disable_logging,
            include_timestamps: self.include_timestamps,
            include_language_detection: self.include_language_detection,
            filter_background_audio: self.filter_background_audio,
            keyterms: self.keyterms,
            no_verbatim: self.no_verbatim,
            entity_detection: self.entity_detection,
            entity_redaction: self.entity_redaction,
            entity_redaction_mode: self.entity_redaction_mode,
            keepalive_interval_ms: self.keepalive_interval_ms,
        })
    }
}
